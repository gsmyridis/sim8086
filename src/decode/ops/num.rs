use std::fmt;

use crate::decode::error::{DResult, DecodeError};
use crate::decode::fields::{Direction, Mode, Reg, Sign, Width, RM};
use crate::decode::operand::{get_operands, Operand, Value};
use crate::register::Register;

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
    pub fn try_decode_rm_reg(bytes: &[u8], optype: NumOpType) -> DResult<Self, &[u8]> {
        let width = Width::parse_byte(bytes[0], 0);
        let direction = Direction::parse_byte(bytes[0]);
        let mode = Mode::try_parse_byte(bytes[1])?;
        let reg = Reg::parse_byte_mid(bytes[1]);
        let rm = RM::parse_byte(bytes[1]);

        let ((source, dest), rest) = get_operands(mode, direction, width, reg, rm, &bytes[2..])?;
        Ok((Self::new(source, dest, optype), rest))
    }

    /// Tries to decode a Immediate - Register/Memory arithmetic operation.
    /// These include ADD, ADC, SUB, SBB, and CMP.
    pub fn try_decode_im_rm(bytes: &[u8]) -> DResult<Self, &[u8]> {
        let width = Width::parse_byte(bytes[0], 0);
        let sign = Sign::parse_byte(bytes[0]);
        let mode = Mode::try_parse_byte(bytes[1])?;
        let optype = NumOpType::try_parse_byte(bytes[1])?;
        let rm = RM::parse_byte(bytes[1]);

        let (dest, rest) =
            Operand::register_or_memory(width.as_bool(), &mode, rm.as_u8(), &bytes[2..])?;

        match (width, sign) {
            (Width::Byte, Sign::NoExtention) => {
                let source = Operand::immediate(Value::Byte(rest[0]));
                Ok((Self::new(source, dest, optype), &rest[1..]))
            }
            (Width::Word, Sign::NoExtention) => {
                let source =
                    Operand::immediate(Value::Word(u16::from_le_bytes([bytes[1], bytes[2]])));
                Ok((Self::new(source, dest, optype), &rest[2..]))
            }
            (Width::Byte, Sign::Extend) => {
                let source = Operand::immediate(Value::Byte(rest[0]));
                Ok((Self::new(source, dest, optype), &rest[1..]))
            }
            (Width::Word, Sign::Extend) => {
                let val = Sign::extend_sign(rest[0]);
                let source = Operand::immediate(Value::Word(val));
                Ok((Self::new(source, dest, optype), &rest[1..]))
            }
        }
    }

    /// Tries to decode an Immediate - Accumulator arithemtic operation.
    /// These include ADD, ADC, SUB, SBB, and CMP.
    pub fn try_decode_im_acc(bytes: &[u8], optype: NumOpType) -> DResult<Self, &[u8]> {
        match Width::parse_byte(bytes[0], 0) {
            Width::Byte => {
                let dest = Operand::Register(Register::AL);
                let source = Operand::immediate(Value::Byte(bytes[1]));
                Ok((Self::new(source, dest, optype), &bytes[2..]))
            }
            Width::Word => {
                let dest = Operand::Register(Register::AX);
                let source =
                    Operand::immediate(Value::Word(u16::from_le_bytes([bytes[0], bytes[1]])));
                Ok((Self::new(source, dest, optype), &bytes[3..]))
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
            } => write!(f, "add {destination}, {source}"),
            Self::Adc {
                source,
                destination,
            } => write!(f, "adc {destination}, {source}"),
            Self::Sub {
                source,
                destination,
            } => write!(f, "sub {destination}, {source}"),
            Self::Sbb {
                source,
                destination,
            } => write!(f, "sbb {destination}, {source}"),
            Self::Cmp {
                source,
                destination,
            } => write!(f, "cmp {destination}, {source}"),
        }
    }
}
