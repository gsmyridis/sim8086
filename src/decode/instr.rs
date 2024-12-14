use std::fmt;

use super::error::DecodeError;
use super::ops::{MovOp, OpCode};

#[derive(Debug)]
pub enum Instruction {
    Mov(MovOp),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mov(op) => write!(f, "{op}"),
        }
    }
}

fn byte_starts_with(byte: u8, prefix: u8, prefix_len: u8) -> bool {
    byte >> (8 - prefix_len) == prefix
}

impl Instruction {

    /// Tries to decode next instruction from the binary representation of the code.
    ///
    /// Takes a reference to an array of bytes that represent the machine code, and attempts
    /// to decode the next instruction. If it succeeds, it returns the instruction and a 
    /// reference to the array with the rest of the bytes. Otherwise it returns the parsing 
    /// error.
    pub fn try_decode_next(bytes: &[u8]) -> Result<(Instruction, &[u8]), DecodeError> {
        if byte_starts_with(bytes[0], OpCode::Mov.into(), 6) {
            match MovOp::try_parse(bytes) {
                Ok((movop, rest)) => Ok((Instruction::Mov(movop), rest)),
                _ => Err(DecodeError),
            }
        } else {
            Err(DecodeError)
        }
    }

    pub fn try_decode(bytes: &[u8]) -> Result<Vec<Instruction>, DecodeError> {
        let mut instructions = Vec::with_capacity(bytes.len());

        let mut bytes_ = bytes;
        while !bytes_.is_empty() {
            let (instr, rest) = Self::try_decode_next(bytes_)?;
            bytes_ = rest;
            println!("{instr}");
            instructions.push(instr);
        }
        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decode_single_mov() {
        let instruction = Instruction::try_decode_next(&[0x89, 0xd9])
            .expect("Guaranteed to succeed")
            .0;
        assert_eq!(&format!("{instruction}"), "mov cx, bx");
    }

    #[test]
    fn test_decode_multi_mov() {
        let bin = &[
            0x89, 0xd9, 0x88, 0xe5, 0x89, 0xda, 0x89, 0xde, 
            0x89, 0xfb, 0x88, 0xc8, 0x88, 0xed, 0x89, 0xc3, 
            0x89, 0xf3, 0x89, 0xfc, 0x89, 0xc5,
        ];

        let asm = &[
            "mov cx, bx",
            "mov ch, ah",
            "mov dx, bx",
            "mov si, bx",
            "mov bx, di",
            "mov al, cl",
            "mov ch, ch",
            "mov bx, ax",
            "mov bx, si",
            "mov sp, di",
            "mov bp, ax",
        ];

        let instructions = Instruction::try_decode(bin).unwrap();
        for (instr, exp) in instructions.into_iter().zip(asm) {
            assert_eq!(&format!("{instr}"), exp);
        }
    }
}
