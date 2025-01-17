use std::fmt;

use super::{DResult, DecodeError, Mode, Register};

#[derive(Debug, PartialEq)]
pub enum Displacement {
    None,
    NoneDirect(u16),
    Byte(i8),
    Word(i16),
}

impl Displacement {
    /// It creates a new `Displacement` from specified `Mode` and `RM` fields,
    /// as well as the remaining byte-code stream.
    ///
    /// The method returns the `Displacement` as well as the number of bytes that
    /// were read from the byte-code stream.
    pub fn new(mode: &Mode, rm: u8, bytes: &[u8]) -> DResult<Self> {
        match mode {
            Mode::Memory => {
                if rm == 0b110 {
                    let addr = u16::from_le_bytes([bytes[0], bytes[1]]);
                    Ok((Displacement::NoneDirect(addr), 2))
                } else {
                    Ok((Displacement::None, 0))
                }
            }
            Mode::Memory8 => Ok((Displacement::Byte(bytes[0] as i8), 1)),
            Mode::Memory16 => {
                let addr = i16::from_le_bytes([bytes[0], bytes[1]]);
                Ok((Displacement::Word(addr), 2))
            }
            Mode::Register => Err(DecodeError::Displacement),
        }
    }

    /// Returns the displacement value as 16-bit signed integer value.
    ///
    /// If the displacement is `NoneDirect`, then the displacement is `None`.
    pub fn value(&self) -> Option<i16> {
        match self {
            Self::None => Some(0i16),
            Self::NoneDirect(_) => None,
            Self::Byte(val) => Some(*val as i16),
            Self::Word(val) => Some(*val),
        }
    }
}

impl fmt::Display for Displacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoneDirect(v) => write!(f, " + {v}"),
            Self::Word(v) => {
                let sign = if *v < 0 { '-' } else { '+' };
                write!(f, " {sign} {}", v.abs())
            }
            Self::Byte(v) => {
                let v = (*v as i16).abs();
                let sign = if v < 0 { '-' } else { '+' };
                write!(f, " {sign} {v}")
            }
            Self::None => write!(f, ""),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EffectiveAddr {
    Direct(u16),
    Reg(Register),
    RegDisp {
        base: Register,
        disp: Displacement,
    },
    RegPair {
        base: Register,
        index: Register,
    },
    RegPairDisp {
        base: Register,
        index: Register,
        disp: Displacement,
    },
}

impl EffectiveAddr {
    /// Creates a new direct effective address.
    #[inline]
    fn direct(addr: u16) -> Self {
        Self::Direct(addr)
    }

    /// Creates a new effective address from register and optionally displacement.
    #[inline]
    fn register(base: Register, disp: Displacement) -> Self {
        match disp {
            Displacement::Byte(_) | Displacement::Word(_) => Self::RegDisp { base, disp },
            _ => Self::Reg(base),
        }
    }

    #[inline]
    fn register_pair(base: Register, index: Register, disp: Displacement) -> Self {
        match disp {
            Displacement::Byte(_) | Displacement::Word(_) => {
                Self::RegPairDisp { base, index, disp }
            }
            _ => Self::RegPair { base, index },
        }
    }

    /// Creates a new effective address from specified R/M field and `Displacement`.
    pub fn new(rm: u8, disp: Displacement) -> Self {
        match rm {
            0b000 => Self::register_pair(Register::BX, Register::SI, disp),
            0b001 => Self::register_pair(Register::BX, Register::DI, disp),
            0b010 => Self::register_pair(Register::BP, Register::SI, disp),
            0b011 => Self::register_pair(Register::BP, Register::DI, disp),
            0b100 => Self::register(Register::SI, disp),
            0b101 => Self::register(Register::DI, disp),
            0b110 => match disp {
                Displacement::NoneDirect(addr) => Self::direct(addr),
                Displacement::None => panic!("Direct address must be specified"),
                _ => Self::register(Register::BP, disp),
            },
            0b111 => Self::register(Register::BX, disp),
            _ => panic!("Invalid value for R/M"),
        }
    }
}

impl fmt::Display for EffectiveAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct(val) => write!(f, "[{val}]"),
            Self::Reg(reg) => write!(f, "[{reg}]"),
            Self::RegDisp { base, disp } => write!(f, "[{base}{disp}]"),
            Self::RegPair { base, index } => write!(f, "[{base} + {index}]"),
            Self::RegPairDisp { base, index, disp } => write!(f, "[{base} + {index}{disp}]"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_with_memory_mode_success() {
        assert_eq!(
            EffectiveAddr::new(0b000, Displacement::None),
            EffectiveAddr::RegPair {
                base: Register::BX,
                index: Register::SI
            }
        );

        assert_eq!(
            EffectiveAddr::new(0b001, Displacement::None),
            EffectiveAddr::RegPair {
                base: Register::BX,
                index: Register::DI
            }
        );

        assert_eq!(
            EffectiveAddr::new(0b001, Displacement::NoneDirect(0xFF)),
            EffectiveAddr::RegPair {
                base: Register::BX,
                index: Register::DI
            }
        );

        assert_eq!(
            EffectiveAddr::new(0b110, Displacement::NoneDirect(0xFF)),
            EffectiveAddr::Direct(0xFF)
        );
    }

    #[test]
    fn test_with_memory_mode8_success() {
        assert_eq!(
            EffectiveAddr::new(0b110, Displacement::Byte(0xFFu8 as i8)),
            EffectiveAddr::RegDisp {
                base: Register::BP,
                disp: Displacement::Byte(0xFFu8 as i8)
            }
        );
    }

    #[test]
    fn test_with_memory16_success() {
        assert_eq!(
            EffectiveAddr::new(0b110, Displacement::Word(0xFFi16)),
            EffectiveAddr::RegDisp {
                base: Register::BP,
                disp: Displacement::Word(0xFFi16)
            }
        );

        let val = i16::from_le_bytes([0xFF, 0xFF]);
        assert_eq!(
            EffectiveAddr::new(0b110, Displacement::Word(val)),
            EffectiveAddr::RegDisp {
                base: Register::BP,
                disp: Displacement::Word(val)
            }
        );
    }
}
