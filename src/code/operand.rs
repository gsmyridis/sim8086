use std::fmt;
use std::cmp::Ordering;

use super::{
    DResult, Direction, Displacement, EffectiveAddr, Mode, Reg, Register, SegmentRegister, Width,
    RM,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Byte(i8),
    Word(i16),
}

impl Value {
    pub fn word(bytes: [u8; 2]) -> Self {
        Value::Word(i16::from_le_bytes(bytes))
    }

    pub fn byte(byte: u8) -> Self {
        Value::Byte(byte as i8)
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Byte(v) => *v == 0,
            Self::Word(v) => *v == 0,
        }
    }

    pub fn is_positive(&self) -> bool {
        match self {
            Self::Byte(v) => *v > 0,
            Self::Word(v) => *v > 0,
        }
    }

    pub fn is_even(&self) -> bool {
        match self {
            Self::Byte(v) => *v % 2 == 0,
            Self::Word(v) => *v % 2 == 0,
        }
    }

    /// Adds two values returning the result along with the overflow, carry and 
    /// auxiliary carry flags.
    pub fn flagged_add(&self, other: &Value) -> (Value, bool, bool, bool) {
        match (self, other) {
            (Value::Word(v1), Value::Word(v2)) => {
                let (val, ov) = (*v1).overflowing_add(*v2);
                let (_, carry) = (*v1 as u16).overflowing_add(*v2 as u16);
                let aux_carry = (*v1 & 0xF) + (*v2 & 0xF) > 0xF;
                (Value::Word(val), ov, carry, aux_carry)
            },
            _ => panic!("Overflowing add has been implemented only for word-word."),
        }
    }

    /// Subtracts two values returning the result along with the overflow, carry and 
    /// auxiliary carry flags.
    pub fn flagged_sub(&self, other: &Value) -> (Value, bool, bool, bool) {
        match (self, other) {
            (Value::Word(v1), Value::Word(v2)) => {
                let (val, ov) = (*v1).overflowing_sub(*v2);
                let carry = (*v1 as u16) < (*v2 as u16);
                let aux_carry = (*v1 & 0xF) < (*v2 & 0xF);
                (Value::Word(val), ov, carry, aux_carry)
            },
            _ => panic!("Overflowing sub has been implemented only for word-word."),
        }
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        let val_self = match self {
            Self::Byte(v) => *v as i16,
            Self::Word(v) => *v,
        };

        let val_other = match other {
            Self::Byte(v) => *v as i16,
            Self::Word(v) => *v,
        };

        val_self.cmp(&val_other)
    }
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Byte(val) => write!(f, "{val}"),
            Value::Word(val) => write!(f, "{val}"),
        }
    }
}

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
    pub fn register_or_memory<'a>(
        width: bool,
        mode: &Mode,
        rm: u8,
        bytes: &'a [u8],
    ) -> DResult<Self, &'a [u8]> {
        match mode {
            Mode::Register => Ok((Operand::register(rm, width), bytes)),
            _ => {
                let (disp, remaining) = Displacement::new(mode, rm, bytes)?;
                Ok((Operand::memory(rm, disp), remaining))
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
) -> DResult<(Operand, Operand), &[u8]> {
    let reg_operand = Operand::register(reg.into(), width.as_bool());
    let (rm_operand, remaining) =
        Operand::register_or_memory(width.as_bool(), &mode, rm.as_u8(), bytes)?;
    match direction {
        Direction::Source => Ok(((reg_operand, rm_operand), remaining)),
        Direction::Destination => Ok(((rm_operand, reg_operand), remaining)),
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
