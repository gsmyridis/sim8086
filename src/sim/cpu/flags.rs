use std::fmt;

#[derive(Debug, Default)]
pub struct Flags {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    pub aux_carry: bool,
    pub overflow: bool,
    pub trap: bool,
    pub interrupt_enable: bool,
    pub direction: bool,
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FLAGS")?;
        writeln!(f, "-------------------")?;
        writeln!(f, "- Zero: {}", self.zero)?;
        writeln!(f, "- Sign: {}", self.sign)?;
        writeln!(f, "- Parity: {}", self.parity)?;
        writeln!(f, "- Overflow: {}", self.overflow)?;
        writeln!(f, "- Carry: {}", self.carry)?;
        writeln!(f, "- Auxiliary Carry: {}", self.aux_carry)?;
        Ok(())
    }
}
