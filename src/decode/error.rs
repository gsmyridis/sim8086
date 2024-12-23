#[derive(Debug)]
pub enum DecodeError {
    Mode,
    NumType,
    OpCode(String),
    Displacement,
}

pub type DResult<I, O> = Result<(I, O), DecodeError>;
