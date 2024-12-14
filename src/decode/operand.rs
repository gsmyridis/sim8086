use std::fmt;

use super::fields::{Direction, Mode, Reg, Width, RM};
use crate::register::Register;

///
#[derive(Debug)]
pub enum Operand {
    Register(Register),
    Memory,
}

impl Operand {
    /// Creates a new register operand from code and width field.
    #[inline]
    pub fn new_register(code: u8, width: &Width) -> Self {
        Self::Register(Register::from(code, width.as_bool()))
    }

    /// Creates a new register or memory operand, depending on the mode, for
    /// specified code and width.
    pub fn register_or_memory(code: u8, width: &Width, mode: &Mode) -> Self {
        match mode {
            Mode::Mem => todo!(),
            Mode::Mem8 => todo!(),
            Mode::Mem16 => todo!(),
            Mode::Reg => Self::new_register(code, width),
        }
    }
}

/// Returns the source and destination operands.
pub fn get_operands(
    direction: Direction,
    mode: Mode,
    width: Width,
    reg: Reg,
    rm: RM,
) -> (Operand, Operand) {
    let reg_operand = Operand::new_register(reg.into(), &width);
    let rm_operand = Operand::register_or_memory(rm.into(), &width, &mode);

    match direction {
        Direction::Source => (reg_operand, rm_operand),
        Direction::Destination => (rm_operand, reg_operand),
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register(reg) => write!(f, "{reg}"),
            _ => f.write_str("ADDRESS"),
        }
    }
}
