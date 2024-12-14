#[repr(u8)]
pub enum OpCode {
    Mov = 0b100010,
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
    }
}
