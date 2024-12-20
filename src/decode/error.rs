#[derive(Debug)]
pub enum DecodeError {
    ModeError,
    DisplacementError,
}

pub type DResult<I, O> = Result<(I, O), DecodeError>;
