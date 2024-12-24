use std::fmt;

#[derive(Debug)]
pub enum CondJumpOp {
    Equal(i8),
    Less(i8),
    LessEqual(i8),
    Below(i8),
    BelowEqual(i8),
    ParityEven(i8),
    Overflow(i8),
    NotEqual(i8),
    Sign(i8),
    GreaterEqual(i8),
    Greater(i8),
    AboveEqual(i8),
    Above(i8),
    ParityOdd(i8),
    NotOverflow(i8),
    NotSign(i8),
    CXZero(i8),

    Loop(i8),
    LoopEqual(i8),
    LoopNEqual(i8),

}

impl fmt::Display for CondJumpOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal(inc) => write!(f, "je {inc}"),
            Self::Less(inc) => write!(f, "jl {inc}"),
            Self::LessEqual(inc) => write!(f, "jle {inc}"),
            Self::Below(inc) => write!(f, "jb {inc}"),
            Self::BelowEqual(inc) => write!(f, "jbe {inc}"),
            Self::ParityEven(inc) => write!(f, "jp {inc}"),
            Self::Overflow(inc) => write!(f, "jo {inc}"),
            Self::NotEqual(inc) => write!(f, "jnz {inc}"),
            Self::Sign(inc) => write!(f, "js {inc}"),
            Self::GreaterEqual(inc) => write!(f, "jnl {inc}"),
            Self::Greater(inc) => write!(f, "jg {inc}"),
            Self::AboveEqual(inc) => write!(f, "jnb {inc}"),
            Self::Above(inc) => write!(f, "ja {inc}"),
            Self::ParityOdd(inc) => write!(f, "jnp {inc}"),
            Self::NotOverflow(inc) => write!(f, "jno {inc}"),
            Self::NotSign(inc) => write!(f, "jns {inc}"),
            Self::CXZero(inc) => write!(f, "jcxz {inc}"),
            Self::Loop(inc) => write!(f, "loop {inc}"),
            Self::LoopEqual(inc) => write!(f, "loopz {inc}"),
            Self::LoopNEqual(inc) => write!(f, "loopnz {inc}"),
        }
    }
}
