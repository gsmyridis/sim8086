use std::cmp::Ordering;
use std::fmt;

mod seg;
use seg::SegmentRegisters;

mod gen;
use gen::GeneralRegisters;

mod flags;
use flags::Flags;

use super::{EResult, ExecutionError};
use crate::code::{ops::*, EffectiveAddr, Instruction, Operand, Decoder};
use crate::value::Value;

const MEM_SIZE: usize = 64 * 1024;
const HALT: u8 = 0xF4;


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

    /// Returns the next instruction's index in memory.
    ///
    /// It is calculated by starting at the code segment memory, `CS`,
    /// and offsetting by the instruction pointer, `IP`.
    #[inline]
    pub fn ip_abs(&self) -> usize {
         self.seg_regs.cs() + (self. ip as usize)
    }

    /// Loads the byte code in memory.
    ///
    /// The loaded bytes are saved in the code segment which begins at the address
    /// stored in the `CS` segment register. To signal the end of the program we 
    /// must either store its offset or add a halt instruction. I chose the latter.
    #[inline]
    pub fn load_instructions(&mut self, buffer: &[u8]) {
        self.mem[self.seg_regs.cs()..self.seg_regs.cs() + buffer.len()]
                    .copy_from_slice(&buffer);
        self.mem[self.seg_regs.cs() + buffer.len()] = HALT;
    }

    /// Executes all the loaded instructions.
    ///
    /// To signal the end of the program we  must either store its offset or add a halt 
    /// instruction. I chose the latter. When the halt instruction is met, the execution 
    /// halts and the istruction pointer is decremented by one, the size of the halt 
    /// instruction.
    pub fn execute(&mut self) -> EResult<()> {
        loop {
            let rem_bytes = &self.mem[self.ip_abs()..];
            let (instruction, size) = Decoder::try_decode_next(rem_bytes)
                .expect("Failed to decode next instruction");
            self.ip += size as u16;

            match instruction {
                Instruction::Mov(ref op) => self.exec_mov(op),
                Instruction::Push(ref op) => self.exec_push(op),
                Instruction::Pop(ref op) => self.exec_pop(op),
                Instruction::Num(ref op) => self.exec_numeric(op),
                Instruction::CondJump(ref op) => self.exec_conditional_jump(op),
                Instruction::Halt => {
                    self.ip -= 1;
                    break
                },
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
        use NumOp::*;

        // Closure that extracts values from operands, executes the specified operation
        // and sets the flags and destination operand.
        let mut exop = |source: &Operand,
                        destination: &Operand,
                        f: fn(&Value, &Value) -> (Value, bool, bool, bool)|
         -> EResult<()> {
            let sval = self.get_operand_value(source);
            let dval = self.get_destination_value(destination)?;

            let (val, overflow, carry, aux_carry) = f(&dval, &sval);
            self.flags
                .set_overflow_aux_carry(overflow, carry, aux_carry);
            self.set_operand_value(destination, val)
        };

        match op {
            Add {
                source,
                destination,
            } => exop(source, destination, |dval, sval| dval.flagged_add(sval)),
            Adc {
                source: _,
                destination: _,
            } => {
                todo!()
            }
            Sub {
                source,
                destination,
            } => exop(source, destination, |dval, sval| dval.flagged_sub(sval)),
            Sbb {
                source: _,
                destination: _,
            } => {
                todo!()
            }
            Cmp {
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
    ///
    /// Decrements the stack-pointer by 2 and then transfers a word from the source
    /// operand to the top of the stack now pointed to by the stack-pointer.
    fn exec_push(&mut self, op: &PushOp) -> EResult<()> {
        self.gen_regs.sp[1] += 1;

        let val = self.get_operand_value(&op.operand).as_u16();
        let sp = u16::from_le_bytes(self.gen_regs.sp) as usize;
        self.mem[sp..sp + 2].copy_from_slice(&val.to_le_bytes());

        Ok(())
    }

    /// Executes a POP instruction.
    ///
    /// Copies the word at the top of the stack to the destination operand
    /// and then increments the stack-pointer by 2.
    fn exec_pop(&mut self, op: &PopOp) -> EResult<()> {
        let sp = u16::from_le_bytes(self.gen_regs.sp) as usize;
        let popped = Value::word([self.mem[sp], self.mem[sp + 1]]);
        self.set_operand_value(&op.operand, popped)?;

        self.gen_regs.sp[1] += 2;
        Ok(())
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

    /// Executes a CONDITIONAAL JUMP instruction.
    ///
    /// For each kind of conditional jump, a condition is checked, and if it's met,
    /// then the instruction pointer is offset by the appropriate amount.
    fn exec_conditional_jump(&mut self, op: &CondJumpOp) -> EResult<()> {
        // Decrements the CX register by 1 and returns if the updated
        // value it is non-zero.
        let decrement_cx = |cx: &mut [u8; 2]| -> bool {
            let updated = u16::from_le_bytes(*cx) - 1;
            *cx = updated.to_le_bytes();
            updated != 0
        };

        let (condition, &offset) = match op {
            // Equality and parity conditions
            CondJumpOp::Equal(n) => (self.flags.zero, n),
            CondJumpOp::NotEqual(n) => (!self.flags.zero, n),
            CondJumpOp::ParityEven(n) => (self.flags.parity, n),
            CondJumpOp::ParityOdd(n) => (!self.flags.parity, n),
            CondJumpOp::Overflow(n) => (self.flags.overflow, n),
            CondJumpOp::NotOverflow(n) => (!self.flags.overflow, n),
            CondJumpOp::Sign(n) => (self.flags.sign, n),
            CondJumpOp::NotSign(n) => (!self.flags.sign, n),

            // Relational conditions
            CondJumpOp::Greater(n) => (
                !self.flags.zero & (self.flags.sign == self.flags.overflow),
                n,
            ),
            CondJumpOp::GreaterEqual(n) => (self.flags.sign == self.flags.overflow, n),
            CondJumpOp::Less(n) => (self.flags.sign != self.flags.overflow, n),
            CondJumpOp::LessEqual(n) => (
                (self.flags.sign != self.flags.overflow) | self.flags.zero,
                n,
            ),
            CondJumpOp::Above(n) => (!self.flags.zero & !self.flags.overflow, n),
            CondJumpOp::AboveEqual(n) => (!self.flags.carry, n),
            CondJumpOp::Below(n) => (self.flags.carry, n),
            CondJumpOp::BelowEqual(n) => (self.flags.carry | self.flags.zero, n),

            // Loop and count-based conditions
            CondJumpOp::CXZero(n) => (u16::from_le_bytes(self.gen_regs.cx) == 0, n),
            CondJumpOp::Loop(n) => (decrement_cx(&mut self.gen_regs.cx), n),
            CondJumpOp::LoopEqual(n) => (decrement_cx(&mut self.gen_regs.cx) & self.flags.zero, n),
            CondJumpOp::LoopNEqual(n) => {
                (decrement_cx(&mut self.gen_regs.cx) & !self.flags.zero, n)
            }
        };
        self.jump(condition, offset)
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
