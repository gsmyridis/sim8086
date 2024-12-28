use std::fmt;

use crate::code::{Register, SegmentRegister, Value};


#[derive(Debug, Default)]
pub struct Flags {
    zero: bool,
    sign: bool,
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FLAGS\n")?;
        write!(f, "-------------------\n")?;
        write!(f, "- Zero: {}\n", self.zero)?;
        write!(f, "- Sign: {}\n", self.sign)?;
        Ok(())
    }
}


#[derive(Debug, Default)]
pub struct Registers {
    ax: [u8; 2],
    bx: [u8; 2],
    cx: [u8; 2],
    dx: [u8; 2],
    sp: [u8; 2],
    bp: [u8; 2],
    si: [u8; 2],
    di: [u8; 2],
}

impl Registers {
    pub fn get(&self, reg: &Register) -> Value {
        match reg {
            Register::AX => Value::word(self.ax),
            Register::AL => Value::byte(self.ax[0]),
            Register::AH => Value::byte(self.ax[1]),
            Register::BX => Value::word(self.bx),
            Register::BL => Value::byte(self.bx[0]),
            Register::BH => Value::byte(self.bx[1]),
            Register::CX => Value::word(self.cx),
            Register::CL => Value::byte(self.cx[0]),
            Register::CH => Value::byte(self.cx[1]),
            Register::DX => Value::word(self.dx),
            Register::DL => Value::byte(self.dx[0]),
            Register::DH => Value::byte(self.dx[1]),
            Register::SP => Value::word(self.sp),
            Register::BP => Value::word(self.bp),
            Register::SI => Value::word(self.si),
            Register::DI => Value::word(self.di),
        }
    }

    pub fn set(&mut self, reg: &Register, val: Value) {
        match val {
            Value::Byte(v) => {
                match reg {
                    Register::AL => self.ax[0] = v as u8,
                    Register::AH => self.ax[1] = v as u8,
                    Register::BL => self.bx[0] = v as u8,
                    Register::BH => self.bx[1] = v as u8,
                    Register::CL => self.cx[0] = v as u8,
                    Register::CH => self.cx[1] = v as u8,
                    Register::DL => self.dx[0] = v as u8,
                    Register::DH => self.dx[1] = v as u8,
                    _ => panic!("Two bytes value for single byte register.")
                }
            },
            Value::Word(v) => {
                let bytes = v.to_le_bytes();
                match reg {
                    Register::AX => self.ax = bytes,
                    Register::BX => self.bx = bytes,
                    Register::CX => self.cx = bytes,
                    Register::DX => self.dx = bytes,
                    Register::SP => self.sp = bytes,
                    Register::BP => self.bp = bytes,
                    Register::SI => self.si = bytes,
                    Register::DI => self.di = bytes,
                    _ => panic!("Single byte for word register.")
                }
            }
        };
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "REGISTERS\n")?;
        write!(f, "-------------------\n")?;
        write!(f, "- AX: {:x}\n", i16::from_le_bytes(self.ax))?;
        write!(f, "- BX: {:x}\n", i16::from_le_bytes(self.bx))?;
        write!(f, "- CX: {:x}\n", i16::from_le_bytes(self.cx))?;
        write!(f, "- DX: {:x}\n", i16::from_le_bytes(self.dx))?;
        write!(f, "- SP: {:x}\n", i16::from_le_bytes(self.sp))?;
        write!(f, "- BP: {:x}\n", i16::from_le_bytes(self.bp))?;
        write!(f, "- SI: {:x}\n", i16::from_le_bytes(self.si))?;
        write!(f, "- DI: {:x}\n", i16::from_le_bytes(self.di))?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct SegmentRegisters {
    es: [u8; 2],
    cs: [u8; 2],
    ss: [u8; 2],
    ds: [u8; 2],
}

impl SegmentRegisters {
    pub fn get(&self, segreg: &SegmentRegister) -> Value {
        let bytes = match segreg {
            SegmentRegister::ES => self.es,
            SegmentRegister::CS => self.cs,
            SegmentRegister::SS => self.ss,
            SegmentRegister::DS => self.ds,
        };
        Value::word(bytes)
    }

    pub fn set(&mut self, segreg: &SegmentRegister, val: Value) {
        match val {
            Value::Word(v) => {
                let bytes = v.to_le_bytes();
                match segreg {
                    SegmentRegister::ES => self.es = bytes,
                    SegmentRegister::CS => self.cs = bytes,
                    SegmentRegister::SS => self.ss = bytes,
                    SegmentRegister::DS => self.ds = bytes,
                };
            },
            _ => panic!("The value of a segment register can be set to a word value.")
        }
    }
}

impl fmt::Display for SegmentRegisters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SEGMENT REGISTERS\n")?;
        write!(f, "-------------------\n")?;
        write!(f, "- ES: {:x}\n", i16::from_le_bytes(self.es))?;
        write!(f, "- CS: {:x}\n", i16::from_le_bytes(self.cs))?;
        write!(f, "- SS: {:x}\n", i16::from_le_bytes(self.ss))?;
        write!(f, "- DS: {:x}\n", i16::from_le_bytes(self.ds))?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    use crate::code::{Register, SegmentRegister, Value};

    #[test]
    fn test_registers_success() {
        let mut regs = Registers::default();
        assert_eq!(regs.get(&Register::AX), Value::Word(0));

        regs.set(&Register::AL, Value::Byte(8));
        assert_eq!(regs.get(&Register::AX), Value::Word(8));

        regs.set(&Register::AH, Value::Byte(1));
        assert_eq!(regs.get(&Register::AL), Value::Byte(8));
        assert_eq!(regs.get(&Register::AH), Value::Byte(1));
        assert_eq!(regs.get(&Register::AX), Value::Word(264));

        regs.set(&Register::AX, Value::Word(1025));
        assert_eq!(regs.get(&Register::AL), Value::Byte(1));
        assert_eq!(regs.get(&Register::AH), Value::Byte(4));
        assert_eq!(regs.get(&Register::AX), Value::Word(1025));
    }

    #[test]
    fn test_segment_registers_success() {
        let mut segregs = SegmentRegisters::default();
        assert_eq!(segregs.get(&SegmentRegister::ES), Value::Word(0));

        segregs.set(&SegmentRegister::ES, Value::Word(10));
        assert_eq!(segregs.get(&SegmentRegister::ES), Value::Word(10));
    }
}
