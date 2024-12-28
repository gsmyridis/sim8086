pub mod error;
pub use error::{EResult, ExecutionError};

pub mod cpu;
pub use cpu::Cpu;

pub mod registers;
pub use registers::{Flags, Registers, SegmentRegisters};
