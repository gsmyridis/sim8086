pub mod error;
pub mod fields;
pub mod operand;
pub mod ops;
pub mod address;

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
