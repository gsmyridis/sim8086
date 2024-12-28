use std::fmt;

use crate::code::{Instruction, Value, Operand};
use crate::code::ops::*;
use super::registers::{Flags, Registers, SegmentRegisters};



#[derive(Debug, Default)]
pub struct Cpu {
    regs: Registers,
    segregs: SegmentRegisters,
    flags: Flags,

}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}\n", self.flags)?;
        write!(f, "{}\n", self.regs)?;
        write!(f, "{}\n", self.segregs)?;
        Ok(())
    }
}


impl Cpu {
    
    pub fn execute(&mut self, instr: Instruction) -> Result<(), ()> {
        match instr {
            Instruction::Mov(op) => self.exec_mov(op),
            Instruction::Push(op) => todo!(),
            Instruction::Pop(op) => todo!(),
            Instruction::Num(op) => todo!(),
            Instruction::Jump(op) => todo!(),
        }
    }

    fn get_source_value(&self, source: Operand) -> Value {
        match source {
            Operand::Register(reg) => self.regs.get(reg),
            Operand::SegmentRegister(segreg) => self.segregs.get(segreg),
            Operand::Immediate(val) => val,
            _ => todo!()
        }
    }

    fn exec_mov(&mut self, op: MovOp) -> Result<(), ()> {
        let val = self.get_source_value(op.source);

        match op.destination {
            Operand::Register(reg) => self.regs.set(reg, val),
            Operand::SegmentRegister(segreg) => self.segregs.set(segreg, val),
            Operand::Immediate(_) => panic!("Destination cannot be immediate value"),
            _ => todo!()
        };
        Ok(())
    }

    fn exec_push(&mut self, _op: PushOp) -> Result<(), ()> {
        // Extract value from source
        // Put it in destination
        todo!()
    }

    fn exec_pop(&mut self, _op: PopOp) -> Result<(), ()> {
        // Extract value from source
        // Put it in destination
        todo!()
    }

    fn exec_numeric(&mut self, _op: NumOp) -> Result<(), ()> {
        // Extract values
        // Perform numeric operation
        // Set flags
        todo!()
    }

    fn exec_jump(&mut self, _op: CondJumpOp) -> Result<(), ()> {
        // Check flags
        // Jump in instruction buffer
        todo!()
    }

}


