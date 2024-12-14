use crate::decode::error::DecodeError;

/// `Width` is a struct that represents the `W` field.
///
/// The `W` field distinguishes between byte and word operation.
/// If `W = 0` the operation acts on a byte; otherwise on a word.
#[derive(Debug)]
pub enum Width {
    Byte,
    Word,
}

impl Width {
    /// Parses a byte and extracts the width field.
    ///
    /// The width field is the least significant bit.
    pub fn parse_byte(byte: u8) -> Self {
        match (byte & 0b1) != 0 {
            false => Self::Byte,
            true => Self::Word,
        }
    }

    /// Returns the width as a bit, in a boolean representation.
    ///
    /// If the width is word `true` is returned; otherwise, `false`.
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Byte => false,
            Self::Word => true,
        }
    }
}

/// `Direction` is a struct that represents the `D` field.
///
/// The `D` field specifies the "direction" of the operation.
/// For `D = 1` the `REG` field in the following byte identifies the
/// destination operand; when `D = 0` the `REG` field identifies the
/// source operand.
#[derive(Debug, PartialEq)]
pub enum Direction {
    Source,
    Destination,
}

impl Direction {
    /// Parses a byte and extracts the direction field.
    ///
    /// The direction field is the second least significant bit.
    pub fn parse_byte(byte: u8) -> Self {
        Self::from((byte >> 1) & 1 == 1)
    }
}

impl From<bool> for Direction {
    fn from(bit: bool) -> Self {
        match bit {
            true => Self::Destination,
            false => Self::Source,
        }
    }
}

/// `Mode` is a struct that represents the `MOD` fields.
///
/// The `MOD` (mode) field indicates whether one of the operands is
/// in memory or whether both operands are registers. The `REG`
/// (register) field identifies a register that is one of the
/// instruction operands.
///  TODO: Add more.
#[derive(Debug, PartialEq)]
pub enum Mode {
    Mem,
    Mem8,
    Mem16,
    Reg,
}

impl Mode {
    /// Parses a byte and extracts the mode fields.
    ///
    /// The mode fields are the 2 most significant bits.
    pub fn try_parse_byte(byte: u8) -> Result<Self, DecodeError> {
        Self::try_from((byte >> 6) & 0b11)
    }
}

impl TryFrom<u8> for Mode {
    type Error = DecodeError;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            0b00 => Ok(Self::Mem),
            0b01 => Ok(Self::Mem8),
            0b10 => Ok(Self::Mem16),
            0b11 => Ok(Self::Reg),
            _ => Err(DecodeError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Reg(u8);

impl Reg {
    /// Parses a byte and extracts the REG fields.
    ///
    /// The REG fields are the 4th, 5th and 6th least significant bits.
    pub fn parse_byte_mid(byte: u8) -> Self {
        Self((byte >> 3) & 0b111)
    }
}

impl Into<u8> for Reg {
    fn into(self) -> u8 {
        self.0
    }
}

/// `RM` is a struct that represents the `R/M` fields.
///
/// The encoding of the `R/M` (register/memory) field depends on
/// how the mode field is set. If the `MOD` selects memory mode,
/// then `R/M` indicates how the effective address of the memory
/// operand is to be calculated.
#[derive(Debug)]
pub struct RM(u8);

impl RM {
    /// Parses a byte and extracts the R/M fields.
    ///
    /// The R/M fields are the three least significant bits.
    pub fn parse_byte(byte: u8) -> Self {
        Self(byte & 0b111)
    }
}

impl Into<u8> for RM {
    fn into(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_width() {
        assert_eq!(Width::parse_byte(0b10101010).as_bool(), false);
        assert_eq!(Width::parse_byte(0b10101011).as_bool(), true);
    }

    #[test]
    fn test_parse_direction() {
        assert_eq!(Direction::parse_byte(0b10101010), Direction::Destination);
        assert_eq!(Direction::parse_byte(0b10101011), Direction::Destination);
        assert_eq!(Direction::parse_byte(0b10101001), Direction::Source);
    }

    #[test]
    fn test_parse_mode() {
        assert_eq!(Mode::try_parse_byte(0b00011010).unwrap(), Mode::Mem);
        assert_eq!(Mode::try_parse_byte(0b01011010).unwrap(), Mode::Mem8);
        assert_eq!(Mode::try_parse_byte(0b10011010).unwrap(), Mode::Mem16);
        assert_eq!(Mode::try_parse_byte(0b11111010).unwrap(), Mode::Reg);
    }

    #[test]
    fn test_parse_reg_mid() {
        assert_eq!(Reg::parse_byte_mid(0b10011110).0, 0b011);
        assert_eq!(Reg::parse_byte_mid(0b11111010).0, 0b111);
    }

    #[test]
    fn test_parse_rm() {
        assert_eq!(RM::parse_byte(0b10101010).0, 0b010);
        assert_eq!(RM::parse_byte(0b10101110).0, 0b110);
    }
}
