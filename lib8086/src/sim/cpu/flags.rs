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

impl Flags {
    /// Sets the zero and sign flags.
    pub fn set_zero_sign(&mut self, zero: bool, sign: bool) {
        self.zero = zero;
        self.sign = sign;
    }

    /// Sets the zero, sign and parity flags.
    pub fn set_zero_sign_parity(&mut self, zero: bool, sign: bool, parity: bool) {
        self.set_zero_sign(zero, sign);
        self.parity = parity;
    }

    /// Sets the overlow, carry and auxilliary carry flags.
    pub fn set_overflow_aux_carry(&mut self, overflow: bool, carry: bool, aux_carry: bool) {
        self.overflow = overflow;
        self.carry = carry;
        self.aux_carry = aux_carry;
    }
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
