#[derive(Debug)]
pub enum ExecutionError {
    ImmediateDestination,
    InstructionOffset,
}

pub type EResult<T> = Result<T, ExecutionError>;
