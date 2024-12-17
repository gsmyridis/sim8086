use std::fmt;

use crate::decode::error::DecodeError;
use crate::decode::fields::*;
use crate::decode::operand::{get_operands, Operand};

#[derive(Debug)]
pub struct MovOp {
    source: Operand,
    destination: Operand,
}

impl MovOp {
    /// Creates a new move operation with specified source and destination operands.
    fn new(source: Operand, destination: Operand) -> Self {
        Self {
            source,
            destination,
        }
    }

    ///
    pub fn try_parse(bytes: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let direction = Direction::parse_byte(bytes[0]);
        let width = Width::parse_byte(bytes[0]);
        let reg = Reg::parse_byte_mid(bytes[1]);
        let rm = RM::parse_byte(bytes[1]);
        let mode = Mode::try_parse_byte(bytes[1])?;

        let (source, dest, bytes) = get_operands(mode, direction, width, reg, rm, &bytes);
        println!("Here");
        Ok((MovOp::new(source, dest), &bytes[2..]))
    }
}

impl fmt::Display for MovOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("mov {}, {}", self.destination, self.source))
    }
}
