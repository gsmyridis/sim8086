#[derive(Debug)]
pub enum DecodeError {
    Mode,
    NumType,
    SegmentRegister,
    OpCode(String),
    Displacement,
}

pub type DResult<I, O> = Result<(I, O), DecodeError>;
