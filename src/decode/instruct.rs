use std::fmt;

use super::error::DecodeError;
use super::ops::OpCode::*;
use super::ops::{CondJumpOp, MovOp, NumOp, NumOpType, OpCode, PushOp, PopOp};

#[derive(Debug)]
pub enum Instruction {
    Mov(MovOp), 
    Push(PushOp),
    Pop(PopOp),
    Num(NumOp),
    Jump(CondJumpOp),
}

impl Instruction {
    /// Tries to decode next instruction from the binary representation of the machine code.
    ///
    /// Takes a reference to an array of bytes that represent the machine code, and attempts
    /// to decode the next instruction. If it succeeds, it returns the instruction and a
    /// reference to the array with the rest of the bytes. Otherwise it returns the parsing
    /// error.
    pub fn try_decode_next(bytes: &[u8]) -> Result<(Instruction, &[u8]), DecodeError> {
        match OpCode::parse(bytes[0])? {
            MovRegRM => {
                let (op, rest) = MovOp::try_parse_reg_rm(bytes)?;
                Ok((Instruction::Mov(op), rest))
            }
            MovImRM => {
                let (op, rest) = MovOp::try_parse_im_rm(bytes)?;
                Ok((Instruction::Mov(op), rest))
            }
            MovImReg => {
                let (op, rest) = MovOp::try_parse_im_reg(bytes)?;
                Ok((Instruction::Mov(op), rest))
            }
            MovMemAcc => {
                let (op, rest) = MovOp::try_decode_mem_acc(bytes)?;
                Ok((Instruction::Mov(op), rest))
            }
            MovRMSegReg | MovSegRegRM => todo!(),
            NumImRM => {
                let (op, rest) = NumOp::try_decode_im_rm(bytes)?;
                Ok((Instruction::Num(op), rest))
            }
            AddRMReg => {
                let (op, rest) = NumOp::try_decode_rm_reg(bytes, NumOpType::Add)?;
                Ok((Instruction::Num(op), rest))
            }
            AdcRMReg => {
                let (op, rest) = NumOp::try_decode_rm_reg(bytes, NumOpType::Adc)?;
                Ok((Instruction::Num(op), rest))
            }
            SubRMReg => {
                let (op, rest) = NumOp::try_decode_rm_reg(bytes, NumOpType::Sub)?;
                Ok((Instruction::Num(op), rest))
            }
            SbbRMReg => {
                let (op, rest) = NumOp::try_decode_rm_reg(bytes, NumOpType::Sbb)?;
                Ok((Instruction::Num(op), rest))
            }
            CmpRMReg => {
                let (op, rest) = NumOp::try_decode_rm_reg(bytes, NumOpType::Cmp)?;
                Ok((Instruction::Num(op), rest))
            }
            AddImAcc => {
                let (op, rest) = NumOp::try_decode_im_acc(bytes, NumOpType::Add)?;
                Ok((Instruction::Num(op), rest))
            }
            AdcImAcc => {
                let (op, rest) = NumOp::try_decode_im_acc(bytes, NumOpType::Adc)?;
                Ok((Instruction::Num(op), rest))
            }
            SubImAcc => {
                let (op, rest) = NumOp::try_decode_im_acc(bytes, NumOpType::Sub)?;
                Ok((Instruction::Num(op), rest))
            }
            SbbImAcc => {
                let (op, rest) = NumOp::try_decode_im_acc(bytes, NumOpType::Sbb)?;
                Ok((Instruction::Num(op), rest))
            }
            CmpImAcc => {
                let (op, rest) = NumOp::try_decode_im_acc(bytes, NumOpType::Cmp)?;
                Ok((Instruction::Num(op), rest))
            }
            JumpEqual => Ok((
                Instruction::Jump(CondJumpOp::Equal(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpLess => Ok((
                Instruction::Jump(CondJumpOp::Less(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpLessEq => Ok((
                Instruction::Jump(CondJumpOp::LessEqual(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpBelow => Ok((
                Instruction::Jump(CondJumpOp::Below(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpBelowEq => Ok((
                Instruction::Jump(CondJumpOp::BelowEqual(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpParityEven => Ok((
                Instruction::Jump(CondJumpOp::ParityEven(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpOverflow => Ok((
                Instruction::Jump(CondJumpOp::Overflow(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpNEqual => Ok((
                Instruction::Jump(CondJumpOp::NotEqual(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpSign => Ok((
                Instruction::Jump(CondJumpOp::Sign(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpGreaterEq => Ok((
                Instruction::Jump(CondJumpOp::GreaterEqual(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpGreater => Ok((
                Instruction::Jump(CondJumpOp::Greater(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpAboveEq => Ok((
                Instruction::Jump(CondJumpOp::AboveEqual(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpAbove => Ok((
                Instruction::Jump(CondJumpOp::Above(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpParityOdd => Ok((
                Instruction::Jump(CondJumpOp::ParityOdd(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpNOverflow => Ok((
                Instruction::Jump(CondJumpOp::NotOverflow(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpNSign => Ok((
                Instruction::Jump(CondJumpOp::NotSign(bytes[1] as i8)),
                &bytes[2..],
            )),
            Loop => Ok((
                Instruction::Jump(CondJumpOp::Loop(bytes[1] as i8)),
                &bytes[2..]
            )),
            LoopEqual => Ok((
                Instruction::Jump(CondJumpOp::LoopEqual(bytes[1] as i8)),
                &bytes[2..]
            )),
            LoopNequal => Ok((
                Instruction::Jump(CondJumpOp::LoopNEqual(bytes[1] as i8)),
                &bytes[2..]
            )),
            JumpCXZero => Ok((
                Instruction::Jump(CondJumpOp::CXZero(bytes[1] as i8)),
                &bytes[2..]
            )),
            PushRegRM => {
                let (op, rest) = PushOp::try_decode_rm(bytes)?;
                Ok((Instruction::Push(op), rest))
            },
            PushReg => {
                let (op, rest) = PushOp::try_decode_reg(bytes)?;
                Ok((Instruction::Push(op), rest))
            },
            PopRegRM => {
                let (op, rest) = PopOp::try_decode_rm(bytes)?;
                Ok((Instruction::Pop(op), rest))
            }
            PopReg => {
                let (op, rest) = PopOp::try_decode_reg(bytes)?;
                Ok((Instruction::Pop(op), rest))
            },
            PushPopSeg => {
                match bytes[0] & 0b111 {
                    0b110 => {
                        let (op, rest) = PushOp::try_decode_seg_reg(bytes)?;
                        Ok((Instruction::Push(op), rest))
                    },
                    0b111 => {
                        let (op, rest) = PopOp::try_decode_seg_reg(bytes)?;
                        Ok((Instruction::Pop(op), rest))
                    },
                    _ => Err(DecodeError::OpCode(format!("{}", bytes[0])))
                }
            }
        }
    }

    /// Tries to decode all instructions from binary representation of the machine code.
    ///
    /// Recursively
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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mov(op) => write!(f, "{op}"),
            Self::Push(op) => write!(f, "{op}"),
            Self::Pop(op) => write!(f, "{op}"),
            Self::Num(op) => write!(f, "{op}"),
            Self::Jump(op) => write!(f, "{op}"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn single_mov_reg_reg() {
        let instruction = Instruction::try_decode_next(&[0x89, 0xd9])
            .expect("Guaranteed to succeed")
            .0;
        assert_eq!(&format!("{instruction}"), "mov cx, bx");
    }

    #[test]
    fn multi_mov_reg_reg() {
        let bin = &[
            0x89, 0xd9, 0x88, 0xe5, 0x89, 0xda, 0x89, 0xde, 0x89, 0xfb, 0x88, 0xc8, 0x88, 0xed,
            0x89, 0xc3, 0x89, 0xf3, 0x89, 0xfc, 0x89, 0xc5,
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

    #[test]
    fn multi_mov_all() {
        let bin = &[
            0x89, 0xde, 0x88, 0xc6, 0xb1, 0x0c, 0xb5, 0xf4, 0xb9, 0x0c, 0x00, 0xb9, 0xf4, 0xff,
            0xba, 0x6c, 0x0f, 0xba, 0x94, 0xf0, 0x8a, 0x00, 0x8b, 0x1b, 0x8b, 0x56, 0x00, 0x8a,
            0x60, 0x04, 0x8a, 0x80, 0x87, 0x13, 0x89, 0x09, 0x88, 0x0a, 0x88, 0x6e, 0x00,
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
            "mov dx, [bp + 0]",
            "mov ah, [bx + si + 4]",
            "mov al, [bx + si + 4999]",
            "mov [bx + di], cx",
            "mov [bp + si], cl",
            "mov [bp + 0], ch",
        ];

        let instructions = Instruction::try_decode(bin).unwrap();
        for (instr, exp) in instructions.into_iter().zip(asm) {
            assert_eq!(&format!("{instr}"), exp);
        }
    }
}
