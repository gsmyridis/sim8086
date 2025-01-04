use std::fmt;

use crate::code::SegmentRegister;
use crate::value::Value;

#[derive(Debug, Default)]
pub struct SegmentRegisters {
    pub es: [u8; 2],
    pub cs: [u8; 2],
    pub ss: [u8; 2],
    pub ds: [u8; 2],
}

impl SegmentRegisters {
    /// Returns the value of the specified segment register.
    pub fn get(&self, seg_reg: &SegmentRegister) -> Value {
        let bytes = match seg_reg {
            SegmentRegister::ES => self.es,
            SegmentRegister::CS => self.cs,
            SegmentRegister::SS => self.ss,
            SegmentRegister::DS => self.ds,
        };
        Value::word(bytes)
    }

    /// Sets the value of the specified segment register.
    pub fn set(&mut self, seg_reg: &SegmentRegister, val: Value) {
        match val {
            Value::Word(v) => {
                let bytes = v.to_le_bytes();
                match seg_reg {
                    SegmentRegister::ES => self.es = bytes,
                    SegmentRegister::CS => self.cs = bytes,
                    SegmentRegister::SS => self.ss = bytes,
                    SegmentRegister::DS => self.ds = bytes,
                };
            }
            _ => panic!("The value of a segment register can be set to a word value."),
        }
    }
}

impl fmt::Display for SegmentRegisters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SEGMENT REGISTERS")?;
        writeln!(f, "-------------------")?;
        writeln!(f, "- ES: 0x{:04x}", i16::from_le_bytes(self.es))?;
        writeln!(f, "- CS: 0x{:04x}", i16::from_le_bytes(self.cs))?;
        writeln!(f, "- SS: 0x{:04x}", i16::from_le_bytes(self.ss))?;
        writeln!(f, "- DS: 0x{:04x}", i16::from_le_bytes(self.ds))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::code::SegmentRegister;
    use crate::value::Value;

    #[test]
    fn test_segment_registers_success() {
        let mut segs = SegmentRegisters::default();
        assert_eq!(segs.get(&SegmentRegister::ES), Value::Word(0));

        segs.set(&SegmentRegister::ES, Value::Word(10));
        assert_eq!(segs.get(&SegmentRegister::ES), Value::Word(10));
    }
}
