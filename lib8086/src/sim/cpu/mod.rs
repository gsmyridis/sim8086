use std::cmp::Ordering;
use std::fmt;

mod seg;
use seg::SegmentRegisters;

mod gen;
use gen::GeneralRegisters;

mod flags;
use flags::Flags;

use crate::code::{ops::*, Instruction, InstructionQueue, Operand, EffectiveAddr};
use crate::value::Value;

use super::{EResult, ExecutionError};


static MEM_SIZE: usize = 1024;


#[derive(Debug)]
pub struct Cpu {
    // Execution Unit (EU)
    pub gen_regs: GeneralRegisters,
    pub flags: Flags,

    // Bus Interface Unit (BIU)
    pub seg_regs: SegmentRegisters,
    pub ip: u16,
    pub mem: [u8; MEM_SIZE],
}

impl Cpu {
    /// Creates a new default CPU.
    pub fn new() -> Self {
        Self {
            gen_regs: GeneralRegisters::default(),
            flags: Flags::default(),
            seg_regs: SegmentRegisters::default(),
            ip: 0,
            mem: [0u8; MEM_SIZE]
        }
    }

    /// Executes a stream of instructions.
    pub fn execute(&mut self, iqueue: &InstructionQueue) -> EResult<()> {
        loop {
            // Fetch next instruction
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
            Operand::Immediate(val) => val.clone(),
            Operand::Memory(addr) => {
                let idx = self.get_effective_address(addr)
                    .expect("Failed to get memory value") as usize; 
                Value::word([self.mem[idx], self.mem[idx + 1]])
            },
        }
    }


    /// Returns the byte offset equivalend of the effective address.
    fn get_effective_address(&self, addr: &EffectiveAddr) -> EResult<u16> {
        match addr {
            EffectiveAddr::Direct(val) => Ok(*val),
            EffectiveAddr::Reg(reg) => Ok(self.gen_regs.get(reg).inner() as u16),
            EffectiveAddr::RegDisp { base, disp } => {
                let base = self.gen_regs.get(base).inner() as u16;
                let disp = disp.value().expect("");
                let sum = base.checked_add_signed(disp)
                    .ok_or(ExecutionError::MemoryOffset)?;
                Ok(sum)
            },
            EffectiveAddr::RegPair { base, index } => {
                let base = self.gen_regs.get(base).inner() as u16;
                let index = self.gen_regs.get(index).inner() as u16;
                Ok(base + index)
            },
            EffectiveAddr::RegPairDisp { base, index, disp } => {
                let base = self.gen_regs.get(base).inner() as u16;
                let index = self.gen_regs.get(index).inner() as u16;
                let disp = disp.value().expect("");
                let sum = base + index;
                let sum = sum.checked_add_signed(disp)
                    .ok_or(ExecutionError::MemoryOffset)?;
                Ok(sum)
            }
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
            Operand::Memory(addr) => {
                let idx = self.get_effective_address(addr)? as usize;
                match val {
                    Value::Byte(v) => self.mem[idx] = v as u8,
                    Value::Word(v) => {
                        let val_bytes = v.to_le_bytes();
                        self.mem[idx] = val_bytes[0];
                        self.mem[idx+1] = val_bytes[1];
                    },
                }
            },
            Operand::Immediate(_) => return Err(ExecutionError::ImmediateDestination),
        };
        Ok(())
    }

    /// Executes a MOV instruction.
    fn exec_mov(&mut self, op: &MovOp) -> EResult<()> {
        let val = self.get_operand_value(&op.source);
        println!("{op} -> {} {val}", op.destination);
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

                let (val, overflowed, carry, aux_carry) = dval.flagged_add(&sval);
                self.flags.overflow = overflowed;
                self.flags.carry = carry;
                self.flags.aux_carry = aux_carry;

                println!("add {destination} {source} -> {destination} {val}");
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

                let (val, overflowed, carry, aux_carry) = dval.flagged_sub(&sval);
                self.flags.overflow = overflowed;
                self.flags.carry = carry;
                self.flags.aux_carry = aux_carry;

                println!("sub {destination} {source} -> {destination} {val}");
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
                let ord = dval.cmp(&sval);
                match ord {
                    Ordering::Equal => {
                        self.flags.zero = true;
                        self.flags.parity = true;
                        self.flags.sign = false;
                    }
                    Ordering::Greater => {
                        self.flags.zero = false;
                        self.flags.sign = false;
                    }
                    Ordering::Less => {
                        self.flags.zero = false;
                        self.flags.sign = true;
                    }
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
    fn exec_cond_jump(&mut self, op: &CondJumpOp) -> EResult<()> {
        match op {
            CondJumpOp::Equal(n) => {
                if self.flags.zero {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::Less(n) => {
                if self.flags.sign {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::LessEqual(n) => {
                if self.flags.sign | self.flags.zero {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::Below(n) => todo!(),
            CondJumpOp::BelowEqual(n) => todo!(),
            CondJumpOp::ParityEven(n) => {
                if self.flags.parity {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::Overflow(n) => {
                if self.flags.overflow {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::NotEqual(n) => {
                if !self.flags.zero {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::Sign(n) => {
                if self.flags.sign {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::GreaterEqual(n) => {
                if !self.flags.sign | self.flags.zero {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::Greater(n) => {
                if !self.flags.sign {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::AboveEqual(n) => todo!(),
            CondJumpOp::Above(n) => todo!(),
            CondJumpOp::ParityOdd(n) => {
                if !self.flags.parity {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::NotOverflow(n) => {
                if !self.flags.overflow {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::NotSign(n) => {
                if !self.flags.sign {
                    self.ip = increment_ip(self.ip, *n)?
                }
            }
            CondJumpOp::CXZero(n) => todo!(),
            CondJumpOp::Loop(n) => todo!(),
            CondJumpOp::LoopEqual(n) => todo!(),
            CondJumpOp::LoopNEqual(n) => todo!(),
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

fn increment_ip(ip: u16, offset: i8) -> EResult<u16> {
    match ip.checked_add_signed(offset.into()) {
        Some(new_ip) => Ok(new_ip),
        None => Err(ExecutionError::InstructionOffset),
    }
}
