use std::fmt;

use crate::code::{DResult, Mode, Operand, Reg, Register, SegmentRegister, RM, SR};

macro_rules! create_push_pop_op {
    (
        $(
           ($op_name:ident, $mnemonic:expr);
        )+
    ) => {
        $(
           #[derive(Debug)]
            pub struct $op_name {
                pub operand: Operand,
            }

            impl $op_name {

                pub fn try_decode_rm(bytes: &[u8]) -> DResult<Self> {
                    let mode = Mode::try_parse_byte(bytes[1])?;
                    let rm = RM::parse_byte(bytes[1]);
                    let (operand, rest) = Operand::register_or_memory(true, &mode, rm.as_u8(), &bytes[2..])?;
                    Ok((Self { operand }, rest))
                }

                pub fn try_decode_reg(bytes: &[u8]) -> DResult<Self> {
                    let reg = Reg::parse_byte_low(bytes[0]);
                    let register = Register::from(reg.into(), true);
                    let operand = Operand::Register(register);
                    Ok((Self { operand }, 1))
                }

                pub fn try_decode_seg_reg(bytes: &[u8]) -> DResult<Self> {
                    let sr = SR::parse_byte(bytes[0]);
                    let segreg = SegmentRegister::try_from(sr.as_u8()).unwrap();
                    let operand = Operand::SegmentRegister(segreg);
                    Ok((Self { operand }, 1))
                }
            }

            impl fmt::Display for $op_name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match &self.operand {
                        Operand::Memory(addr) => write!(f, "{} word {addr}", $mnemonic),
                        Operand::Register(reg) => write!(f, "{} {reg}", $mnemonic),
                        Operand::SegmentRegister(segreg) => write!(f, "{} {segreg}", $mnemonic),
                        Operand::Immediate(_) => panic!("{}ed value cannot be immediate", $mnemonic),
                    }
                }
            }
        )+
    }
}

create_push_pop_op! {
    (PushOp, "push");
    (PopOp, "pop");
}
