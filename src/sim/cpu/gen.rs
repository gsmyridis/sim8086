use std::fmt;
use crate::code::Register;
use crate::value::Value;

#[derive(Debug, Default)]
pub struct GeneralRegisters {
    ax: [u8; 2],
    bx: [u8; 2],
    cx: [u8; 2],
    dx: [u8; 2],
    sp: [u8; 2],
    bp: [u8; 2],
    si: [u8; 2],
    di: [u8; 2],
}

impl GeneralRegisters {
    /// Returns the value of the specified general register.
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

    /// Sets the value of the specified general register.
    pub fn set(&mut self, reg: &Register, val: Value) {
        match val {
            Value::Byte(v) => match reg {
                Register::AL => self.ax[0] = v as u8,
                Register::AH => self.ax[1] = v as u8,
                Register::BL => self.bx[0] = v as u8,
                Register::BH => self.bx[1] = v as u8,
                Register::CL => self.cx[0] = v as u8,
                Register::CH => self.cx[1] = v as u8,
                Register::DL => self.dx[0] = v as u8,
                Register::DH => self.dx[1] = v as u8,
                _ => panic!("Two bytes value for single byte register."),
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
                    _ => panic!("Single byte for word register."),
                }
            }
        };
    }
}

impl fmt::Display for GeneralRegisters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GENERAL REGISTERS")?;
        writeln!(f, "-------------------")?;
        writeln!(f, "- AX: 0x{:04x}", i16::from_le_bytes(self.ax))?;
        writeln!(f, "- BX: 0x{:04x}", i16::from_le_bytes(self.bx))?;
        writeln!(f, "- CX: 0x{:04x}", i16::from_le_bytes(self.cx))?;
        writeln!(f, "- DX: 0x{:04x}", i16::from_le_bytes(self.dx))?;
        writeln!(f, "- SP: 0x{:04x}", i16::from_le_bytes(self.sp))?;
        writeln!(f, "- BP: 0x{:04x}", i16::from_le_bytes(self.bp))?;
        writeln!(f, "- SI: 0x{:04x}", i16::from_le_bytes(self.si))?;
        writeln!(f, "- DI: 0x{:04x}", i16::from_le_bytes(self.di))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::code::Register;
    use crate::value::Value;

    #[test]
    fn test_registers_success() {
        let mut gen = GeneralRegisters::default();
        assert_eq!(gen.get(&Register::AX), Value::Word(0));

        gen.set(&Register::AL, Value::Byte(8));
        assert_eq!(gen.get(&Register::AX), Value::Word(8));

        gen.set(&Register::AH, Value::Byte(1));
        assert_eq!(gen.get(&Register::AL), Value::Byte(8));
        assert_eq!(gen.get(&Register::AH), Value::Byte(1));
        assert_eq!(gen.get(&Register::AX), Value::Word(264));

        gen.set(&Register::AX, Value::Word(1025));
        assert_eq!(gen.get(&Register::AL), Value::Byte(1));
        assert_eq!(gen.get(&Register::AH), Value::Byte(4));
        assert_eq!(gen.get(&Register::AX), Value::Word(1025));
    }
}
