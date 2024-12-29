use std::fmt;

use crate::code::ops::*;
use crate::code::{Instruction, Operand, Value};

use super::{EResult, ExecutionError, Flags, Registers, SegmentRegisters};

#[derive(Debug, Default)]
pub struct Cpu {
    regs: Registers,
    segregs: SegmentRegisters,
    flags: Flags,
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}", self.flags)?;
        writeln!(f, "{}", self.regs)?;
        writeln!(f, "{}", self.segregs)?;
        Ok(())
    }
}

impl Cpu {
    pub fn execute(&mut self, instr: Instruction) -> EResult<()> {
        match instr {
            Instruction::Mov(op) => self.exec_mov(op),
            Instruction::Push(op) => todo!(),
            Instruction::Pop(op) => todo!(),
            Instruction::Num(op) => self.exec_numeric(op),
            Instruction::Jump(op) => todo!(),
        }
    }

    fn get_operand_value(&self, operand: &Operand) -> Value {
        match operand {
            Operand::Register(reg) => self.regs.get(reg),
            Operand::SegmentRegister(segreg) => self.segregs.get(segreg),
            Operand::Immediate(val) => val.clone(),
            _ => todo!(),
        }
    }

    fn set_value(&mut self, dest: &Operand, val: Value) -> EResult<()> {
        match dest {
            Operand::Register(reg) => self.regs.set(reg, val),
            Operand::SegmentRegister(segreg) => self.segregs.set(segreg, val),
            Operand::Immediate(_) => return Err(ExecutionError::ImmediateDestination),
            _ => todo!(),
        };
        Ok(())
    }

    fn get_destination_value(&self, operand: &Operand) -> EResult<Value> {
        match operand {
            Operand::Immediate(_) => Err(ExecutionError::ImmediateDestination),
            _ => Ok(self.get_operand_value(operand)),
        }
    }

    fn exec_mov(&mut self, op: MovOp) -> EResult<()> {
        let val = self.get_operand_value(&op.source);
        println!("mov {}, {} -> {} {val}", &op.destination, &op.source, &op.destination);
        self.set_value(&op.destination, val)
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

    fn exec_numeric(&mut self, op: NumOp) -> EResult<()> {
        match op {
            NumOp::Add {
                source,
                destination,
            } => {
                let source_val = self.get_operand_value(&source);
                let dest_val = self.get_destination_value(&destination)?;
                let val = add_values(source_val, dest_val)?;
                println!("add {destination} {source} -> {destination} {val}");
                self.flags.set_from_value(&val);
                self.set_value(&destination, val)
            }
            NumOp::Adc {
                source,
                destination,
            } => {
                todo!()
            }
            NumOp::Sub {
                source,
                destination,
            } => {
                let sval = self.get_operand_value(&source);
                let dval = self.get_destination_value(&destination)?;
                let val = sub_values(sval, dval)?;
                self.flags.set_from_value(&val);
                println!("sub {destination} {source} -> {destination} {val}");
                self.set_value(&destination, val)
            }
            NumOp::Sbb {
                source,
                destination,
            } => {
                todo!()
            }
            NumOp::Cmp {
                source,
                destination,
            } => {
                let sval = self.get_operand_value(&source);
                let dval = self.get_destination_value(&destination)?;
                println!("cmp {destination}, {source}");
                let val = sub_values(sval, dval)?;
                self.flags.set_from_value(&val);
                Ok(())
            }
        }
    }

    fn exec_jump(&mut self, _op: CondJumpOp) -> Result<(), ()> {
        // Check flags
        // Jump in instruction buffer
        todo!()
    }
}

fn add_values(val1: Value, val2: Value) -> EResult<Value> {
    match (val1, val2) {
        (Value::Word(v1), Value::Word(v2)) => Ok(Value::Word(v1 + v2)),
        _ => todo!(),
    }
}

fn sub_values(val1: Value, val2: Value) -> EResult<Value> {
    match (val1, val2) {
        (Value::Word(v1), Value::Word(v2)) => Ok(Value::Word(v2 - v1)),
        _ => todo!(),
    }
}
