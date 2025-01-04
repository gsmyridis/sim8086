use std::fmt;

use super::{
    DResult, Direction, Displacement, EffectiveAddr, Mode, Reg, Register, SegmentRegister, Width,
    RM,
};
use crate::value::Value;

/// A struct representing an instruction operand.
///
/// Instructions have one or two operands, each one being either, a register,
/// a memory address or an immediate value.
#[derive(Debug)]
pub enum Operand {
    Register(Register),
    SegmentRegister(SegmentRegister),
    Memory(EffectiveAddr),
    Immediate(Value),
}

impl Operand {
    /// Creates a new register operand from code and width field.
    #[inline]
    pub fn register(rm: u8, width: bool) -> Self {
        Self::Register(Register::from(rm, width))
    }

    /// Creates a new memory operand from R/M field and `Displacement`.
    #[inline]
    pub fn memory(rm: u8, disp: Displacement) -> Operand {
        Self::Memory(EffectiveAddr::new(rm, disp))
    }

    /// Creates an immediate value operand from a `Value`.
    #[inline]
    pub fn immediate(value: Value) -> Operand {
        Self::Immediate(value)
    }

    /// Creates a register or memory operand.
    pub fn register_or_memory(width: bool, mode: &Mode, rm: u8, bytes: &[u8]) -> DResult<Self> {
        match mode {
            Mode::Register => Ok((Operand::register(rm, width), 0)),
            _ => {
                let (disp, bytes_read) = Displacement::new(mode, rm, bytes)?;
                Ok((Operand::memory(rm, disp), bytes_read))
            }
        }
    }
}

/// Returns the source and destination register operands.
pub fn get_operands(
    mode: Mode,
    direction: Direction,
    width: Width,
    reg: Reg,
    rm: RM,
    bytes: &[u8],
) -> DResult<(Operand, Operand)> {
    let reg_operand = Operand::register(reg.into(), width.as_bool());
    let (rm_operand, bytes_read) =
        Operand::register_or_memory(width.as_bool(), &mode, rm.as_u8(), bytes)?;
    match direction {
        Direction::Source => Ok(((reg_operand, rm_operand), bytes_read)),
        Direction::Destination => Ok(((rm_operand, reg_operand), bytes_read)),
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register(reg) => write!(f, "{reg}"),
            Self::SegmentRegister(segreg) => write!(f, "{segreg}"),
            Self::Immediate(val) => write!(f, "{val}"),
            Self::Memory(addr) => write!(f, "{addr}"),
        }
    }
}

pub fn get_prefix<'a>(source: &'a Operand, dest: &'a Operand) -> &'a str {
    match (source, dest) {
        (Operand::Immediate(Value::Byte(_)), Operand::Memory(_)) => "byte ",
        (Operand::Immediate(Value::Word(val)), Operand::Memory(_)) => {
            if (*val).abs() <= i8::MAX as i16 {
                "word "
            } else {
                ""
            }
        }
        _ => "",
    }
}
