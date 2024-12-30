use std::fmt;
use std::cmp::Ordering;

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

    /// Executes the instruction.
    pub fn execute(&mut self, instr: Instruction) -> EResult<()> {
        match instr {
            Instruction::Mov(op) => self.execute_mov(op),
            Instruction::Push(op) => todo!(),
            Instruction::Pop(op) => todo!(),
            Instruction::Num(op) => self.execute_numeric(op),
            Instruction::Jump(op) => todo!(),
        }
    }

    /// Returns the value that the operand holds.
    fn get_operand_value(&self, operand: &Operand) -> Value {
        match operand {
            Operand::Register(reg) => self.regs.get(reg),
            Operand::SegmentRegister(segreg) => self.segregs.get(segreg),
            Operand::Immediate(val) => val.clone(),
            _ => todo!(),
        }
    }

    /// Sets the value of the destination operand, and updates the Zero, Sign, 
    /// Parity status flags.
    fn set_value(&mut self, dest: &Operand, val: Value) -> EResult<()> {
        self.flags.set_from_value(&val);
        match dest {
            Operand::Register(reg) => self.regs.set(reg, val),
            Operand::SegmentRegister(segreg) => self.segregs.set(segreg, val),
            Operand::Immediate(_) => return Err(ExecutionError::ImmediateDestination),
            _ => todo!(),
        };
        Ok(())
    }

    /// Returns the value the destination operand holds.
    ///
    /// The destination operand cannot be an immediate value.
    fn get_destination_value(&self, operand: &Operand) -> EResult<Value> {
        match operand {
            Operand::Immediate(_) => Err(ExecutionError::ImmediateDestination),
            _ => Ok(self.get_operand_value(operand)),
        }
    }

    /// Executes a MOV instruction.
    fn execute_mov(&mut self, op: MovOp) -> EResult<()> {
        let val = self.get_operand_value(&op.source);
        println!("mov {}, {} -> {} {val}", &op.destination, &op.source, &op.destination);
        self.set_value(&op.destination, val)
    }

    /// Executes an arithmetic instruction: ADD, ADC, SUB, SBB or CMP.
    fn execute_numeric(&mut self, op: NumOp) -> EResult<()> {
        match op {
            NumOp::Add {
                source,
                destination,
            } => {
                let sval = self.get_operand_value(&source);
                let dval = self.get_destination_value(&destination)?;

                let (val, overflowed, carry, aux_carry) = dval.flagged_add(&sval);
                self.flags.overflow = overflowed;
                self.flags.carry = carry;
                self.flags.aux_carry = aux_carry;

                println!("add {destination} {source} -> {destination} {val}");
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

                let (val, overflowed, carry, aux_carry) = dval.flagged_sub(&sval);
                self.flags.overflow = overflowed;
                self.flags.carry = carry;
                self.flags.aux_carry = aux_carry;

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
                let ord = dval.cmp(&sval);
                match ord {
                    Ordering::Equal => self.flags.set_zero(),
                    Ordering::Greater => self.flags.set_positive(),
                    Ordering::Less => self.flags.set_negative(),
                }
                println!("cmp {destination}, {source}");
                Ok(())
            }
        }
    }

    /// Executes a PUSH instruction.
    fn exec_push(&mut self, _op: PushOp) -> Result<(), ()> {
        // Extract value from source
        // Put it in destination
        todo!()
    }

    /// Executes a POP instruction.
    fn exec_pop(&mut self, _op: PopOp) -> Result<(), ()> {
        // Extract value from source
        // Put it in destination
        todo!()
    }


    /// Executes a JUMP instruction.
    fn exec_jump(&mut self, _op: CondJumpOp) -> Result<(), ()> {
        // Check flags
        // Jump in instruction buffer
        todo!()
    }
}
