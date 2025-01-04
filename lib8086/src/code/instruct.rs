use std::fmt;

use super::ops::*;

#[derive(Debug)]
pub enum Instruction {
    Mov(MovOp),
    Push(PushOp),
    Pop(PopOp),
    Num(NumOp),
    CondJump(CondJumpOp),
    Halt,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mov(op) => write!(f, "{op}"),
            Self::Push(op) => write!(f, "{op}"),
            Self::Pop(op) => write!(f, "{op}"),
            Self::Num(op) => write!(f, "{op}"),
            Self::CondJump(op) => write!(f, "{op}"),
            Self::Halt => write!(f, ""),
        }
    }
}

#[derive(Debug, Default)]
pub struct InstructionQueue {
    inner: Vec<Instruction>,
    sizes: Vec<usize>,
    byte_offsets: Vec<usize>,
    next_offset: usize,
}

impl InstructionQueue {
    /// Gets the instruction for the specified instruction pointer.
    pub fn get(&self, ip: usize) -> Option<(&Instruction, &usize)> {
        let idx = self.byte_offsets.iter().position(|&offset| offset == ip)?;
        Some((self.inner.get(idx)?, self.sizes.get(idx)?))
    }

    /// Appends and instruction to the queue specifying its size.
    pub fn push(&mut self, instr: Instruction, size: usize) {
        self.inner.push(instr);
        self.sizes.push(size);
        self.byte_offsets.push(self.next_offset);
        self.next_offset += size;
    }
}

impl fmt::Display for InstructionQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let asm = self.inner
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{asm}")
    }
}
