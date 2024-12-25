use std::fmt;

use crate::decode::fields::{Mode, RM, Reg, SR};
use crate::decode::error::DResult;
use crate::decode::operand::Operand;
use crate::register::{Register, SegmentRegister};


#[derive(Debug)]
pub struct PushOp {
    source: Operand
}


impl PushOp {

    pub fn try_decode_rm(bytes: &[u8]) -> DResult<Self, &[u8]> {
        let mode = Mode::try_parse_byte(bytes[1])?;
        let rm = RM::parse_byte(bytes[1]);
        let (source, rest) = Operand::register_or_memory(true, &mode, rm.as_u8(), &bytes[2..])?;
        Ok((PushOp { source }, rest))
    }

    pub fn try_decode_reg(bytes: &[u8]) -> DResult<Self, &[u8]> {
        let reg = Reg::parse_byte_low(bytes[0]);
        let register = Register::from(reg.into(), true);
        let source = Operand::Register(register);
        Ok((PushOp { source }, &bytes[1..]))
    }

    pub fn try_decode_seg_reg(bytes: &[u8]) -> DResult<Self, &[u8]> {
        let sr = SR::parse_byte(bytes[0]);
        let segreg = SegmentRegister::try_from(sr.as_u8()).unwrap();
        let source = Operand::SegmentRegister(segreg);
        Ok((PushOp { source }, &bytes[1..]))
    }
}


impl fmt::Display for PushOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            Operand::Memory(addr) => write!(f, "push word {addr}"),
            Operand::Register(reg) => write!(f, "push {reg}"),
            Operand::SegmentRegister(segreg) => write!(f, "push {segreg}"),
            Operand::Immediate(_) => panic!("Pushed value cannot be immediate")
        }
    }
}
