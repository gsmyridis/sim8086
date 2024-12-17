use std::fmt;

use super::fields::{Direction, Mode, Reg, Width, RM};
use crate::register::Register;
use crate::decode::address::{Displacement, EffectiveAddr};

///
#[derive(Debug)]
pub enum Operand {
    Register(Register),
    Memory(EffectiveAddr),

}

impl Operand {
    /// Creates a new register operand from code and width field.
    #[inline]
    pub fn new_register(rm: u8, width: &Width) -> Self {
        Self::Register(Register::from(rm, width.as_bool()))
    }

    pub fn new_memory(rm: u8, disp: Displacement) -> Operand {
        Self::Memory(EffectiveAddr::new(rm, disp))
    }

}


/// Returns the source and destination register operands.
pub fn get_operands<'a>(
    mode: Mode,
    direction: Direction,
    width: Width,
    reg: Reg,
    rm: RM,
    bytes: &'a[u8],
) -> (Operand, Operand, &'a[u8]) {
    let mut bytes_rest = bytes;
    let reg_operand = Operand::new_register(reg.into(), &width);
    let rm_operand = match mode {
        Mode::Register => Operand::new_register(rm.into(), &width),
        _ => {
            let (disp, b) = Displacement::new(&mode, &rm, &bytes).expect("Failed to create a displacement");
            bytes_rest = b;
            println!("{:?}", disp);
            Operand::new_memory(rm.into(), disp)
        }
    };

    match direction {
        Direction::Source => (reg_operand, rm_operand, &bytes_rest),
        Direction::Destination => (rm_operand, reg_operand, &bytes_rest),
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
