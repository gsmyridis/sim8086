use std::fmt;

use crate::code::fields::*;
use crate::code::{get_bit, get_operands, DResult, EffectiveAddr, Operand, Register, Value};

#[derive(Debug)]
pub struct MovOp {
    source: Operand,
    destination: Operand,
}

impl MovOp {
    /// Creates a new move operation with specified source and destination operands.
    fn new(source: Operand, destination: Operand) -> Self {
        Self {
            source,
            destination,
        }
    }

    /// Parse a Register - Register/Memory MOV instruction.
    pub fn try_parse_reg_rm(bytes: &[u8]) -> DResult<Self, &[u8]> {
        let direction = Direction::parse_byte(bytes[0]);
        let width = Width::parse_byte(bytes[0], 0);
        let mode = Mode::try_parse_byte(bytes[1])?;
        let reg = Reg::parse_byte_mid(bytes[1]);
        let rm = RM::parse_byte(bytes[1]);

        let ((source, dest), bytes) = get_operands(mode, direction, width, reg, rm, &bytes[2..])?;
        Ok((MovOp::new(source, dest), bytes))
    }

    /// Parse an Immediate - Register/Memory MOV instruction.
    pub fn try_parse_im_rm(bytes: &[u8]) -> DResult<Self, &[u8]> {
        let width = Width::parse_byte(bytes[0], 0);
        let mode = Mode::try_parse_byte(bytes[1])?;
        assert_eq!((bytes[1] >> 3) & 0b111, 0b000u8);
        let rm = RM::parse_byte(bytes[1]);

        let (dest, remaining) =
            Operand::register_or_memory(width.as_bool(), &mode, rm.as_u8(), &bytes[2..])?;

        match width {
            Width::Byte => {
                let source = Operand::immediate(Value::Byte(remaining[0]));
                Ok((MovOp::new(source, dest), &remaining[1..]))
            }
            Width::Word => {
                let source = Operand::immediate(Value::Word(u16::from_le_bytes([
                    remaining[0],
                    remaining[1],
                ])));
                Ok((MovOp::new(source, dest), &remaining[2..]))
            }
        }
    }

    /// Parses an Immediate to Register MOV instruction.
    pub fn try_parse_im_reg(bytes: &[u8]) -> DResult<Self, &[u8]> {
        let width = Width::parse_byte(bytes[0], 3);
        let n_bytes = width.n_bytes();
        let reg = Reg::parse_byte_low(bytes[0]);
        let value = match width {
            Width::Byte => Value::Byte(bytes[1]),
            Width::Word => Value::Word(u16::from_le_bytes([bytes[1], bytes[2]])),
        };

        let source = Operand::immediate(value);
        let dest = Operand::register(reg.into(), width.as_bool());
        Ok((MovOp::new(source, dest), &bytes[1 + n_bytes..]))
    }

    /// Decodes a Memory to Accumulator MOV instruction.
    pub fn try_decode_mem_acc(bytes: &[u8]) -> DResult<Self, &[u8]> {
        let addr = EffectiveAddr::Direct(u16::from_le_bytes([bytes[1], bytes[2]]));
        let mem = Operand::Memory(addr);
        let acc = match Width::parse_byte(bytes[0], 0) {
            Width::Byte => Operand::Register(Register::AL),
            Width::Word => Operand::Register(Register::AX),
        };
        let (source, dest) = match get_bit(bytes[0], 1) {
            true => (acc, mem),
            false => (mem, acc),
        };
        Ok((MovOp::new(source, dest), &bytes[3..]))
    }
}

impl fmt::Display for MovOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match (&self.source, &self.destination) {
            (Operand::Immediate(Value::Byte(_)), Operand::Memory(_)) => "byte ",
            (Operand::Immediate(Value::Word(_)), Operand::Memory(_)) => "word ",
            _ => "",
        };
        f.write_str(&format!(
            "mov {}, {}{}",
            self.destination, prefix, self.source
        ))
    }
}
