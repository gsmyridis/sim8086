#[derive(Debug)]
pub enum ExecutionError {
    ImmediateDestination,
    InstructionOffset,
    MemoryOffset,
}

pub type EResult<T> = Result<T, ExecutionError>;
