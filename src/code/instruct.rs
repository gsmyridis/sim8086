use std::fmt;

use super::ops::*;

#[derive(Debug)]
pub enum Instruction {
    Mov(MovOp),
    Push(PushOp),
    Pop(PopOp),
    Num(NumOp),
    Jump(CondJumpOp),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mov(op) => write!(f, "{op}"),
            Self::Push(op) => write!(f, "{op}"),
            Self::Pop(op) => write!(f, "{op}"),
            Self::Num(op) => write!(f, "{op}"),
            Self::Jump(op) => write!(f, "{op}"),
        }
    }
}
