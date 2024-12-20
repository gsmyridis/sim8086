#[repr(u8)]
pub enum OpCode {
    /// MOV
    MovRegRM = 0b100010,
    MovImRM = 0b1100011,
    MovImReg = 0b1011,
    MovMemAcc = 0b1010000,
    MovAccMem = 0b1010001,
    MovRMSegReg = 0b10001110, 
    MovSegRegRM = 0b10001100, 
}

impl OpCode {
    fn len(&self) -> u8 {
        todo!()
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
    }
}
