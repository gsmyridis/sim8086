#[derive(Debug)]
pub enum ExecutionError {
    ImmediateDestination 
}

pub type EResult<T> = Result<T, ExecutionError>;
