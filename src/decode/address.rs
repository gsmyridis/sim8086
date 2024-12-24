use std::fmt;

use super::fields::Mode;
use crate::decode::error::DecodeError;
use crate::register::Register;

#[derive(Debug, PartialEq)]
pub enum Displacement {
    None,
    NoneDirect(u16),
    Byte(u8),
    Word(i16),
}

impl Displacement {
    /// Calculates how many bytes it needs to take to read the displacements.
    pub fn new<'a>(mode: &Mode, rm: u8, bytes: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        match mode {
            Mode::Memory => {
                if rm == 0b110 {
                    assert!(bytes.len() >= 2, "The byte array is too short.");
                    let addr = u16::from_le_bytes([bytes[0], bytes[1]]);
                    Ok((Displacement::NoneDirect(addr), &bytes[2..]))
                } else {
                    Ok((Displacement::None, bytes))
                }
            }
            Mode::Memory8 => {
                assert!(!bytes.is_empty(), "The byte array is too short.");
                Ok((Displacement::Byte(bytes[0]), &bytes[1..]))
            }
            Mode::Memory16 => {
                assert!(bytes.len() >= 2, "The byte array is too short.");
                let addr = i16::from_le_bytes([bytes[0], bytes[1]]);
                Ok((Displacement::Word(addr), &bytes[2..]))
            }
            Mode::Register => Err(DecodeError::Displacement),
        }
    }
}

impl fmt::Display for Displacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoneDirect(val) => write!(f, " + {val}"),
            Self::Word(val) => write!(f, " + {val}"),
            Self::Byte(val) => write!(f, " + {val}"),
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
            EffectiveAddr::new(0b110, Displacement::Byte(0xFF)),
            EffectiveAddr::RegDisp {
                base: Register::BP,
                disp: Displacement::Byte(0xFF)
            }
        );
    }

    #[test]
    fn test_with_memory16_success() {
        assert_eq!(
            EffectiveAddr::new(0b110, Displacement::Word(0xFF as i16)),
            EffectiveAddr::RegDisp {
                base: Register::BP,
                disp: Displacement::Word(0xFF as i16)
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
