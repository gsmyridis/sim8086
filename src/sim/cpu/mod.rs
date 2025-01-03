use std::fmt;
use std::cmp::Ordering;

mod seg;
use seg::SegmentRegisters;

mod gen;
use gen::GeneralRegisters;

mod flags;
use flags::Flags;

use crate::code::{Operand, ops::*, Instruction, InstructionQueue};
use crate::value::Value;

use super::{EResult, ExecutionError, mem::Memory};



#[derive(Debug, Default)]
pub struct Cpu {
    // Execution Unit (EU)
    gen_regs: GeneralRegisters,
    flags: Flags,

    // Bus Interface Unit (BIU)
    seg_regs: SegmentRegisters,
    ip: usize,
    mem: Memory,
}

impl Cpu {

    /// Creates a new default CPU.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&mut self, iqueue: &InstructionQueue) -> EResult<()> {
        loop {
            // Fetch next instruction
            let (instr, offset) = iqueue.get(self.ip).ok_or(ExecutionError::InstructionOffset)?;
            self.ip += offset;

            // Execute instruction
            match instr {
                Instruction::Mov(op) => self.execute_mov(op),
                Instruction::Push(_op) => todo!(),
                Instruction::Pop(_op) => todo!(),
                Instruction::Num(op) => self.execute_numeric(op),
                Instruction::Jump(_op) => todo!(),
                Instruction::Halt => break,
            }?
        }
        Ok(())
    }

    /// Sets the flags from value.
    ///
    /// The flags that can be set from a value are the Zero, Sign and Parity.
    pub fn set_flags_from_val(&mut self, val: &Value) {
        self.flags.zero = val.is_zero();
        self.flags.sign = val.is_negative();
        self.flags.parity = val.is_even();
    }

    /// Returns the value that the operand holds.
    ///
    /// If the operand is a general register, the value is fetched from the Execution Unit (EU);
    /// otherwise, from the Bus Interface Unit (BIU).
    fn get_operand_value(&self, operand: &Operand) -> Value {
        match operand {
            Operand::Register(reg) => self.gen_regs.get(reg),
            Operand::SegmentRegister(reg) => self.seg_regs.get(reg),
            Operand::Memory(addr) => self.mem.get(addr),
            Operand::Immediate(val) => val.clone(),
        }
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

    /// Sets the value of the destination operand, and updates the Zero, Sign, 
    /// Parity status flags.
    ///
    /// If the 
    fn set_operand_value(&mut self, dest: &Operand, val: Value) -> EResult<()> {
        self.set_flags_from_val(&val);
        match dest {
            Operand::Register(reg) => self.gen_regs.set(reg, val),
            Operand::SegmentRegister(reg) => self.seg_regs.set(reg, val),
            Operand::Memory(addr) => self.mem.set(addr, val),
            Operand::Immediate(_) => return Err(ExecutionError::ImmediateDestination),
        };
        Ok(())
    }

    /// Executes a MOV instruction.
    fn execute_mov(&mut self, op: &MovOp) -> EResult<()> {
        let val = self.get_operand_value(&op.source);
        println!("{op} -> {} {val}", op.destination);
        self.set_operand_value(&op.destination, val)
    }

    /// Executes an arithmetic instruction: ADD, ADC, SUB, SBB or CMP.
    fn execute_numeric(&mut self, op: &NumOp) -> EResult<()> {
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
                self.set_operand_value(&destination, val)
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
                self.set_operand_value(&destination, val)
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
                    Ordering::Equal => {
                        self.flags.zero = true;
                        self.flags.parity = true;
                        self.flags.sign = false;
                    },
                    Ordering::Greater => {
                        self.flags.zero = false;
                        self.flags.sign = false;
                    },
                    Ordering::Less => {
                        self.flags.zero = false;
                        self.flags.sign = true;
                    },
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




impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}", self.flags)?;
        writeln!(f, "{}", self.gen_regs)?;
        writeln!(f, "{}", self.seg_regs)?;
        Ok(())
    }
}
