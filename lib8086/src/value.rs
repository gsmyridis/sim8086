use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Byte(i8),
    Word(i16),
}

impl Value {
    /// Creates a 16-bit value from 2 bytes.
    #[inline]
    pub fn word(bytes: [u8; 2]) -> Self {
        Value::Word(i16::from_le_bytes(bytes))
    }

    /// Creates an 8-bit value from a byte.
    #[inline]
    pub fn byte(byte: u8) -> Self {
        Value::Byte(byte as i8)
    }

    /// Returns whether the value is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Byte(v) => *v == 0,
            Self::Word(v) => *v == 0,
        }
    }

    /// Returns whether the value is positive.
    #[inline]
    pub fn is_negative(&self) -> bool {
        match self {
            Self::Byte(v) => *v < 0,
            Self::Word(v) => *v < 0,
        }
    }

    /// Returns whether the value is positive.
    #[inline]
    pub fn is_even(&self) -> bool {
        match self {
            Self::Byte(v) => *v % 2 == 0,
            Self::Word(v) => *v % 2 == 0,
        }
    }

    /// Returns the inner value as a 16-bit integer.
    #[inline]
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::Byte(v) => *v as u16,
            Self::Word(v) => *v as u16,
        }
    }

    /// Adds two values returning the result along with the overflow, carry and
    /// auxiliary carry flags.
    pub fn flagged_add(&self, other: &Value) -> (Value, bool, bool, bool) {
        match (self, other) {
            (Self::Word(v1), Self::Word(v2)) => {
                let (val, ov) = (*v1).overflowing_add(*v2);
                let (_, carry) = (*v1 as u16).overflowing_add(*v2 as u16);
                let aux_carry = (*v1 & 0xF) + (*v2 & 0xF) > 0xF;
                (Self::Word(val), ov, carry, aux_carry)
            }
            _ => panic!("Flagged add has been implemented only for word-word."),
        }
    }

    /// Subtracts two values returning the result along with the overflow, carry and
    /// auxiliary carry flags.
    pub fn flagged_sub(&self, other: &Value) -> (Value, bool, bool, bool) {
        match (self, other) {
            (Self::Word(v1), Self::Word(v2)) => {
                let (val, ov) = (*v1).overflowing_sub(*v2);
                let carry = (*v1 as u16) < (*v2 as u16);
                let aux_carry = (*v1 & 0xF) < (*v2 & 0xF);
                (Self::Word(val), ov, carry, aux_carry)
            }
            _ => panic!("Flagged sub has been implemented only for word-word."),
        }
    }

    /// Performes a carrying add between two values returning the result and
    /// the oveflow, carry and auxiliary flags.
    pub fn flagged_carrying_add(&self, other: &Value, carry: bool) -> (Value, bool, bool, bool) {
        let carry = if carry {
            Value::Word(1i16)
        } else {
            Value::Word(0i16)
        };
        match (self, other) {
            (Self::Word(v1), Self::Word(v2)) => {
                let (v1, ov1, cf1, acf1) = self.flagged_add(other);
                let (v2, ov2, cf2, acf2) = v1.flagged_add(&carry);
                todo!()
            }
            _ => panic!("Flagged add has been implemented only for word-word."),
        }
    }

    /// Performes a carrying sub between two values returning the result and
    /// the oveflow, carry and auxiliary flags.
    pub fn flagged_carrying_sub(&self, other: &Value, carry: bool) -> (Value, bool, bool, bool) {
        let carry = if carry {
            Value::Word(1i16)
        } else {
            Value::Word(0i16)
        };
        match (self, other) {
            (Self::Word(v1), Self::Word(v2)) => {
                let (v1, ov1, cf1, acf1) = self.flagged_sub(other);
                let (v2, ov2, cf2, acf2) = v1.flagged_sub(&carry);
                todo!()
            }
            _ => panic!("Flagged carrying add has been implemented only for word-word."),
        }
    }

    /// Compares two values.
    pub fn cmp(&self, other: &Self) -> Ordering {
        let val_self = match self {
            Self::Byte(v) => *v as i16,
            Self::Word(v) => *v,
        };

        let val_other = match other {
            Self::Byte(v) => *v as i16,
            Self::Word(v) => *v,
        };

        val_self.cmp(&val_other)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Byte(val) => write!(f, "{val}"),
            Value::Word(val) => write!(f, "{val}"),
        }
    }
}
