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

fn has_prefix(byte: u8, prefix: u8, prefix_len: u8) -> bool {
    byte >> (8 - prefix_len) == prefix
}

impl Instruction {

    /// Tries to decode next instruction from the binary representation of the machine code.
    ///
    /// Takes a reference to an array of bytes that represent the machine code, and attempts
    /// to decode the next instruction. If it succeeds, it returns the instruction and a 
    /// reference to the array with the rest of the bytes. Otherwise it returns the parsing 
    /// error.
    pub fn try_parse_next(bytes: &[u8]) -> Result<(Instruction, &[u8]), DecodeError> {
        let (movop, rest) = if has_prefix(bytes[0], OpCode::MovRegRM.into(), 6) {
            MovOp::try_parse_reg_rm(bytes)?
        } else if has_prefix(bytes[0], OpCode::MovImRM.into(), 7) {
            MovOp::try_parse_im_rm(bytes)?
        } else if has_prefix(bytes[0], OpCode::MovImReg.into(), 4) {
            MovOp::try_parse_im_reg(bytes)?
        } else {
            todo!();
        };
        Ok((Instruction::Mov(movop), rest))
    }

    /// Tries to decode all instructions from binary representation of the machine code.
    ///
    /// Recursively
    pub fn try_parse(bytes: &[u8]) -> Result<Vec<Instruction>, DecodeError> {
        let mut instructions = Vec::with_capacity(bytes.len());

        let mut bytes_ = bytes;
        while !bytes_.is_empty() {
            let (instr, rest) = Self::try_parse_next(bytes_)?;
            bytes_ = rest;
            instructions.push(instr);
        }
        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn single_mov_reg_reg() {
        let instruction = Instruction::try_parse_next(&[0x89, 0xd9])
            .expect("Guaranteed to succeed")
            .0;
        assert_eq!(&format!("{instruction}"), "mov cx, bx");
    }

    #[test]
    fn multi_mov_reg_reg() {
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

        let instructions = Instruction::try_parse(bin).unwrap();
        for (instr, exp) in instructions.into_iter().zip(asm) {
            assert_eq!(&format!("{instr}"), exp);
        }
    }

    #[test]
    fn multi_mov_all() {

        let bin = &[
            0x89, 0xde, 0x88, 0xc6, 0xb1, 0x0c, 0xb5, 0xf4, 
            0xb9, 0x0c, 0x00, 0xb9, 0xf4, 0xff, 0xba, 0x6c,
            0x0f, 0xba, 0x94, 0xf0, 0x8a, 0x00, 0x8b, 0x1b,
            0x8b, 0x56, 0x00, 0x8a, 0x60, 0x04, 0x8a, 0x80,
            0x87, 0x13, 0x89, 0x09, 0x88, 0x0a, 0x88, 0x6e,
            0x00,
        ];

        let asm = &[
            "mov si, bx",
            "mov dh, al",
            "mov cl, 12",
            "mov ch, 244",
            "mov cx, 12",
            "mov cx, 65524",
            "mov dx, 3948",
            "mov dx, 61588",
            "mov al, [bx + si]",
            "mov bx, [bp + di]",
            "mov dx, [bp]",
            "mov ah, [bx + si + 4]",
            "mov al, [bx + si + 34579]",
            "mov [bx + di], cx",
            "mov [bp + si], cl",
            "mov [bp], ch",
        ];

        let instructions = Instruction::try_parse(bin).unwrap();
        for (instr, exp) in instructions.into_iter().zip(asm) {
            assert_eq!(&format!("{instr}"), exp);
        }
    }
}
