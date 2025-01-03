pub mod error;
pub use error::{EResult, ExecutionError};

pub mod cpu;
pub use cpu::Cpu;

mod mem;
