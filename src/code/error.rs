#[derive(Debug)]
pub enum DecodeError {
    Mode,
    NumType,
    SegmentRegister,
    OpCode(String),
    Displacement,
}

pub type DResult<I> = Result<(I, usize), DecodeError>;
