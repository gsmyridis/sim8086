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
    /// Parses a byte and extracts the width field from the least 
    /// significant bit.
    #[inline]
    pub fn parse_byte(byte: u8) -> Self {
        Self::from((byte & 0b1) != 0)
    }

    /// Parses a byte and extracts the width field from the fourth
    /// least significant bit.
    #[inline]
    pub fn parse_byte_mid(byte: u8) -> Self {
        Self::from((byte & 0b1000) != 0)
    }

    /// Returns the width as a bit, in a boolean representation.
    ///
    /// If the width is word `true` is returned; otherwise, `false`.
    #[inline]
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Byte => false,
            Self::Word => true,
        }
    }

    /// Returns the width's equivalend in number of bytes.
    #[inline]
    pub fn n_bytes(&self) -> usize {
        self.as_bool() as usize + 1
    }
}

impl From<bool> for Width {
    #[inline]
    fn from(bit: bool) -> Self {
        match bit {
            false => Self::Byte,
            true => Self::Word,
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
    #[inline]
    pub fn parse_byte(byte: u8) -> Self {
        Self::from((byte >> 1) & 1 == 1)
    }
}

impl From<bool> for Direction {
    #[inline]
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
    Memory,
    Memory8,
    Memory16,
    Register,
}

impl Mode {
    /// Parses a byte and extracts the mode fields.
    ///
    /// The mode fields are the 2 most significant bits.
    #[inline]
    pub fn try_parse_byte(byte: u8) -> Result<Self, DecodeError> {
        Self::try_from((byte >> 6) & 0b11)
    }

    /// Calculates how many bytes it needs to take to read the displacements.
    pub fn n_bytes(&self, rm: &RM) ->  u8 {
        match self {
            Mode::Memory => if rm.as_u8() == 0b110 { 2 } else { 0 },
            Mode::Memory8 => 1,
            Mode::Memory16 => 2,
            Mode::Register => 0 
        }
    }
}

impl TryFrom<u8> for Mode {
    type Error = DecodeError;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            0b00 => Ok(Self::Memory),
            0b01 => Ok(Self::Memory8),
            0b10 => Ok(Self::Memory16),
            0b11 => Ok(Self::Register),
            _ => Err(DecodeError::ModeError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Reg(u8);

impl Reg {
    /// Parses a byte and extracts the REG fields from the 4th, 5th
    /// and 6th least significant bits.
    #[inline]
    pub fn parse_byte_mid(byte: u8) -> Self {
        Self((byte >> 3) & 0b111)
    }

    /// Parses a byte and extracts the REG fields from the three least
    /// significant bits
    #[inline]
    pub fn parse_byte_low(byte: u8) -> Self {
        Self(byte & 0b111)
    }
}

impl Into<u8> for Reg {
    #[inline]
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
//TODO: MAYBE Create construction method that checks that its <= 0b111
#[derive(Debug)]
pub struct RM(u8);


impl RM {
    /// Parses a byte and extracts the R/M fields.
    ///
    /// The R/M fields are the three least significant bits.
    #[inline]
    pub fn parse_byte(byte: u8) -> Self {
        Self(byte & 0b111)
    }

    #[inline]
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl Into<u8> for RM {
    #[inline]
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

        assert_eq!(Width::parse_byte_mid(0b10101010).as_bool(), true);
        assert_eq!(Width::parse_byte_mid(0b10100010).as_bool(), false);
    }

    #[test]
    fn test_parse_direction() {
        assert_eq!(Direction::parse_byte(0b10101010), Direction::Destination);
        assert_eq!(Direction::parse_byte(0b10101011), Direction::Destination);
        assert_eq!(Direction::parse_byte(0b10101001), Direction::Source);
    }

    #[test]
    fn test_parse_mode() {
        assert_eq!(Mode::try_parse_byte(0b00011010).unwrap(), Mode::Memory);
        assert_eq!(Mode::try_parse_byte(0b01011010).unwrap(), Mode::Memory8);
        assert_eq!(Mode::try_parse_byte(0b10011010).unwrap(), Mode::Memory16);
        assert_eq!(Mode::try_parse_byte(0b11111010).unwrap(), Mode::Register);
    }

    #[test]
    fn test_parse_reg() {
        assert_eq!(Reg::parse_byte_mid(0b10011110).0, 0b011);
        assert_eq!(Reg::parse_byte_mid(0b11111010).0, 0b111);

        assert_eq!(Reg::parse_byte_low(0b10011110).0, 0b110);
        assert_eq!(Reg::parse_byte_low(0b11111010).0, 0b010);
    }

    #[test]
    fn test_parse_rm() {
        assert_eq!(RM::parse_byte(0b10101010).0, 0b010);
        assert_eq!(RM::parse_byte(0b10101110).0, 0b110);
    }
}
