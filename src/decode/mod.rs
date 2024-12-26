pub mod error;
pub use error::{DResult, DecodeError};

pub mod address;
pub use address::{Displacement, EffectiveAddr};

pub mod register;
pub use register::{Register, SegmentRegister};

pub mod fields;
pub use fields::*;

pub mod operand;
pub use operand::{get_operands, Operand, Value};

pub mod ops;

pub mod instruct;
pub use instruct::Instruction;

/// Returns the bit located in position `pos` from the given byte
/// in boolean representation.
#[inline]
pub fn get_bit(byte: u8, pos: u8) -> bool {
    (byte >> pos) & 1 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        let byte = 0b10101010;
        assert!(get_bit(byte, 1));
        assert!(get_bit(byte, 3));
        assert!(get_bit(byte, 5));
        assert!(get_bit(byte, 7));
        assert!(!get_bit(byte, 0));
        assert!(!get_bit(byte, 2));
        assert!(!get_bit(byte, 4));
        assert!(!get_bit(byte, 6));
    }
}
