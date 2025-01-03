use std::fmt;

use crate::code::fields::*;
use crate::code::operand::{get_operands, get_prefix, Operand};
use crate::code::{DResult, DecodeError, Register};
use crate::value::Value;

pub enum NumOpType {
    Add = 0b000,
    Adc = 0b010,
    Sub = 0b101,
    Sbb = 0b011,
    Cmp = 0b111,
}

impl NumOpType {
    fn try_parse_byte(byte: u8) -> Result<Self, DecodeError> {
        match (byte >> 3) & 0b111 {
            0b000 => Ok(Self::Add),
            0b010 => Ok(Self::Adc),
            0b101 => Ok(Self::Sub),
            0b011 => Ok(Self::Sbb),
            0b111 => Ok(Self::Cmp),
            _ => Err(DecodeError::NumType),
        }
    }
}

#[derive(Debug)]
pub enum NumOp {
    Add {
        source: Operand,
        destination: Operand,
    },
    Adc {
        source: Operand,
        destination: Operand,
    },
    Sub {
        source: Operand,
        destination: Operand,
    },
    Sbb {
        source: Operand,
        destination: Operand,
    },
    Cmp {
        source: Operand,
        destination: Operand,
    },
}

impl NumOp {
    fn new(source: Operand, destination: Operand, optype: NumOpType) -> Self {
        match optype {
            NumOpType::Add => Self::Add {
                source,
                destination,
            },
            NumOpType::Adc => Self::Adc {
                source,
                destination,
            },
            NumOpType::Sub => Self::Sub {
                source,
                destination,
            },
            NumOpType::Sbb => Self::Sbb {
                source,
                destination,
            },
            NumOpType::Cmp => Self::Cmp {
                source,
                destination,
            },
        }
    }

    /// Tries to decode a Register/Memory - Register arithmetic operation.
    /// These include ADD, ADC, SUB, SBB, and CMP.
    pub fn try_decode_rm_reg(bytes: &[u8], optype: NumOpType) -> DResult<Self> {
        let width = Width::parse_byte(bytes[0], 0);
        let direction = Direction::parse_byte(bytes[0]);
        let mode = Mode::try_parse_byte(bytes[1])?;
        let reg = Reg::parse_byte_mid(bytes[1]);
        let rm = RM::parse_byte(bytes[1]);

        let ((source, dest), bytes_read) =
            get_operands(mode, direction, width, reg, rm, &bytes[2..])?;
        Ok((Self::new(source, dest, optype), bytes_read + 2))
    }

    /// Tries to decode an Immediate - Register/Memory arithmetic operation.
    /// These include ADD, ADC, SUB, SBB, and CMP.
    pub fn try_decode_im_rm(bytes: &[u8]) -> DResult<Self> {
        let width = Width::parse_byte(bytes[0], 0);
        let sign = Sign::parse_byte(bytes[0]);
        let mode = Mode::try_parse_byte(bytes[1])?;
        let optype = NumOpType::try_parse_byte(bytes[1])?;
        let rm = RM::parse_byte(bytes[1]);

        let (dest, bytes_read) =
            Operand::register_or_memory(width.as_bool(), &mode, rm.as_u8(), &bytes[2..])?;
        let rest = &bytes[2 + bytes_read..];

        match (width, sign) {
            (Width::Byte, Sign::NoExtention) => {
                let source = Operand::immediate(Value::byte(rest[0]));
                Ok((Self::new(source, dest, optype), 3 + bytes_read))
            }
            (Width::Word, Sign::NoExtention) => {
                let source = Operand::immediate(Value::word([rest[0], rest[1]]));
                Ok((Self::new(source, dest, optype), 4 + bytes_read))
            }
            (Width::Byte, Sign::Extend) => {
                let source = Operand::immediate(Value::byte(rest[0]));
                Ok((Self::new(source, dest, optype), 3 + bytes_read))
            }
            (Width::Word, Sign::Extend) => {
                let val = (rest[0] as i8) as i16;
                let source = Operand::immediate(Value::Word(val));
                Ok((Self::new(source, dest, optype), 3 + bytes_read))
            }
        }
    }

    /// Tries to decode an Immediate - Accumulator arithemtic operation.
    /// These include ADD, ADC, SUB, SBB, and CMP.
    pub fn try_decode_im_acc(bytes: &[u8], optype: NumOpType) -> DResult<Self> {
        match Width::parse_byte(bytes[0], 0) {
            Width::Byte => {
                let dest = Operand::Register(Register::AL);
                let source = Operand::immediate(Value::byte(bytes[1]));
                Ok((Self::new(source, dest, optype), 2))
            }
            Width::Word => {
                let dest = Operand::Register(Register::AX);
                let source = Operand::immediate(Value::word([bytes[1], bytes[2]]));
                Ok((Self::new(source, dest, optype), 3))
                // Maybe add Value::Signed(i16)
            }
        }
    }
}

impl fmt::Display for NumOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add {
                source,
                destination,
            } => {
                let prefix = get_prefix(source, destination);
                write!(f, "add {prefix}{destination}, {source}")
            }
            Self::Adc {
                source,
                destination,
            } => {
                let prefix = get_prefix(source, destination);
                write!(f, "adc {prefix}{destination}, {source}")
            }
            Self::Sub {
                source,
                destination,
            } => {
                let prefix = get_prefix(source, destination);
                write!(f, "sub {prefix}{destination}, {source}")
            }
            Self::Sbb {
                source,
                destination,
            } => {
                let prefix = get_prefix(source, destination);
                write!(f, "sbb {prefix}{destination}, {source}")
            }
            Self::Cmp {
                source,
                destination,
            } => {
                let prefix = get_prefix(source, destination);
                write!(f, "cmp {prefix}{destination}, {source}")
            }
        }
    }
}
