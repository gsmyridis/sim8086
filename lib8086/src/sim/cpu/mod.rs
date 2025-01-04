use std::cmp::Ordering;
use std::fmt;

mod seg;
use seg::SegmentRegisters;

mod gen;
use gen::GeneralRegisters;

mod flags;
use flags::Flags;

use super::{EResult, ExecutionError};
use crate::code::{ops::*, EffectiveAddr, Instruction, InstructionQueue, Operand};
use crate::value::Value;

const MEM_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Cpu {
    pub gen_regs: GeneralRegisters,
    pub seg_regs: SegmentRegisters,
    pub flags: Flags,
    pub mem: [u8; MEM_SIZE],
    pub ip: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            gen_regs: GeneralRegisters::default(),
            seg_regs: SegmentRegisters::default(),
            flags: Flags::default(),
            mem: [0u8; MEM_SIZE],
            ip: 0,
        }
    }
}

impl Cpu {
    /// Creates a new CPU with default initial state.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes a stream of instructions.
    pub fn execute(&mut self, iqueue: &InstructionQueue) -> EResult<()> {
        loop {
            // Fetch next instruction and then increment the instruction pointer.
            let (instr, offset) = iqueue
                .get(self.ip as usize)
                .ok_or(ExecutionError::InstructionOffset)?;
            self.ip += *offset as u16;

            // Execute instruction
            match instr {
                Instruction::Mov(op) => self.exec_mov(op),
                Instruction::Push(_op) => todo!(),
                Instruction::Pop(_op) => todo!(),
                Instruction::Num(op) => self.exec_numeric(op),
                Instruction::CondJump(op) => self.exec_cond_jump(op),
                Instruction::Halt => break,
            }?
        }
        Ok(())
    }

    /// Returns the value that the operand holds.
    ///
    /// If the operand is a general register, the value is fetched from the Execution Unit (EU);
    /// otherwise, from the Bus Interface Unit (BIU).
    fn get_operand_value(&self, operand: &Operand) -> Value {
        match operand {
            Operand::Register(reg) => self.gen_regs.get(reg),
            Operand::SegmentRegister(reg) => self.seg_regs.get(reg),
            Operand::Immediate(val) => val.clone(),
            Operand::Memory(addr) => {
                let idx = self
                    .get_memory_index(addr)
                    .expect("Failed to get memory value") as usize;
                Value::word([self.mem[idx], self.mem[idx + 1]])
            }
        }
    }

    /// Returns the byte offset equivalend of the effective address.
    fn get_memory_index(&self, addr: &EffectiveAddr) -> EResult<u16> {
        let idx = match addr {
            EffectiveAddr::Direct(idx) => *idx,
            EffectiveAddr::Reg(reg) => self.gen_regs.get(reg).as_u16(),
            EffectiveAddr::RegDisp { base, disp } => {
                let base = self.gen_regs.get(base).as_u16();
                let disp = disp.value().expect("");
                base.checked_add_signed(disp)
                    .ok_or(ExecutionError::MemoryOffset)?
            }
            EffectiveAddr::RegPair { base, index } => {
                let base = self.gen_regs.get(base).as_u16();
                let index = self.gen_regs.get(index).as_u16();
                base + index
            }
            EffectiveAddr::RegPairDisp { base, index, disp } => {
                let base = self.gen_regs.get(base).as_u16();
                let index = self.gen_regs.get(index).as_u16();
                let disp = disp.value().expect("");
                let sum = base + index;
                sum.checked_add_signed(disp)
                    .ok_or(ExecutionError::MemoryOffset)?
            }
        };
        Ok(idx)
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

    /// Sets the value of the destination operand, and updates the zero, sign,
    /// parity status flags.
    fn set_operand_value(&mut self, dest: &Operand, val: Value) -> EResult<()> {
        self.flags
            .set_zero_sign_parity(val.is_zero(), val.is_negative(), val.is_even());
        match dest {
            Operand::Register(reg) => self.gen_regs.set(reg, val),
            Operand::SegmentRegister(reg) => self.seg_regs.set(reg, val),
            Operand::Memory(addr) => {
                let idx = self.get_memory_index(addr)? as usize;
                match val {
                    Value::Byte(v) => self.mem[idx] = v as u8,
                    Value::Word(v) => self.mem[idx..idx + 2].copy_from_slice(&v.to_le_bytes()),
                }
            }
            Operand::Immediate(_) => Err(ExecutionError::ImmediateDestination)?,
        };
        Ok(())
    }

    /// Executes a MOV instruction.
    fn exec_mov(&mut self, op: &MovOp) -> EResult<()> {
        let val = self.get_operand_value(&op.source);
        self.set_operand_value(&op.destination, val)
    }

    /// Executes an arithmetic instruction: ADD, ADC, SUB, SBB or CMP.
    fn exec_numeric(&mut self, op: &NumOp) -> EResult<()> {
        match op {
            NumOp::Add {
                source,
                destination,
            } => {
                let sval = self.get_operand_value(source);
                let dval = self.get_destination_value(destination)?;

                let (val, overflow, carry, aux_carry) = dval.flagged_add(&sval);
                self.flags
                    .set_overflow_aux_carry(overflow, carry, aux_carry);
                self.set_operand_value(destination, val)
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
                let sval = self.get_operand_value(source);
                let dval = self.get_destination_value(destination)?;

                let (val, overflow, carry, aux_carry) = dval.flagged_sub(&sval);
                self.flags
                    .set_overflow_aux_carry(overflow, carry, aux_carry);
                self.set_operand_value(destination, val)
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
                let sval = self.get_operand_value(source);
                let dval = self.get_destination_value(destination)?;
                match dval.cmp(&sval) {
                    Ordering::Equal => self.flags.set_zero_sign_parity(true, false, true),
                    Ordering::Greater => self.flags.set_zero_sign(false, false),
                    Ordering::Less => self.flags.set_zero_sign(false, true),
                }
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

    fn jump(&mut self, condition: bool, offset: i8) -> EResult<()> {
        if condition {
            self.ip = self
                .ip
                .checked_add_signed(offset.into())
                .ok_or(ExecutionError::InstructionOffset)?;
        }
        Ok(())
    }

    /// Executes a JUMP instruction.
    fn exec_cond_jump(&mut self, op: &CondJumpOp) -> EResult<()> {
        match op {
            CondJumpOp::Equal(n) => self.jump(self.flags.zero, *n)?,
            CondJumpOp::Less(n) => self.jump(self.flags.sign, *n)?,
            CondJumpOp::LessEqual(n) => self.jump(self.flags.sign | self.flags.zero, *n)?,
            CondJumpOp::Below(_n) => todo!(),
            CondJumpOp::BelowEqual(_n) => todo!(),
            CondJumpOp::ParityEven(n) => self.jump(self.flags.parity, *n)?,
            CondJumpOp::Overflow(n) => self.jump(self.flags.overflow, *n)?,
            CondJumpOp::NotEqual(n) => self.jump(!self.flags.zero, *n)?,
            CondJumpOp::Sign(n) => self.jump(self.flags.sign, *n)?,
            CondJumpOp::GreaterEqual(n) => self.jump(self.flags.sign | self.flags.zero, *n)?,
            CondJumpOp::Greater(n) => self.jump(!self.flags.sign, *n)?,
            CondJumpOp::AboveEqual(_n) => todo!(),
            CondJumpOp::Above(_n) => todo!(),
            CondJumpOp::ParityOdd(n) => self.jump(!self.flags.parity, *n)?,
            CondJumpOp::NotOverflow(n) => self.jump(!self.flags.overflow, *n)?,
            CondJumpOp::NotSign(n) => self.jump(!self.flags.sign, *n)?,
            CondJumpOp::CXZero(_n) => todo!(),
            CondJumpOp::Loop(_n) => todo!(),
            CondJumpOp::LoopEqual(_n) => todo!(),
            CondJumpOp::LoopNEqual(_n) => todo!(),
        }
        Ok(())
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}", self.flags)?;
        writeln!(f, "{}", self.gen_regs)?;
        writeln!(f, "{}", self.seg_regs)?;
        writeln!(f, "- IP: 0x{:04x}", self.ip)?;
        Ok(())
    }
}
