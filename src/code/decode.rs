use super::ops::*;
use super::{DecodeError, Instruction};
use crate::code::ops::OpCode::*;

pub struct Decoder<'de> {
    bytes: &'de [u8],
}

impl Decoder<'_> {
    pub fn from(bytes: &[u8]) -> Decoder<'_> {
        Decoder { bytes }
    }

    /// Tries to decode next instruction from the binary representation of the machine code.
    ///
    /// Takes a reference to an array of bytes that represent the machine code, and attempts
    /// to decode the next instruction. If it succeeds, it returns the instruction and a
    /// reference to the array with the rest of the bytes. Otherwise, it returns the decoding.
    /// error.
    fn try_decode_next(bytes: &[u8]) -> Result<(Instruction, &[u8]), DecodeError> {
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
            MovRMSegReg | MovSegRegRM => {
                let (op, rest) = MovOp::try_decode_rm_segreg(bytes)?;
                Ok((Instruction::Mov(op), rest))
            }
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
                &bytes[2..],
            )),
            LoopEqual => Ok((
                Instruction::Jump(CondJumpOp::LoopEqual(bytes[1] as i8)),
                &bytes[2..],
            )),
            LoopNEqual => Ok((
                Instruction::Jump(CondJumpOp::LoopNEqual(bytes[1] as i8)),
                &bytes[2..],
            )),
            JumpCXZero => Ok((
                Instruction::Jump(CondJumpOp::CXZero(bytes[1] as i8)),
                &bytes[2..],
            )),
            PushRegRM => {
                let (op, rest) = PushOp::try_decode_rm(bytes)?;
                Ok((Instruction::Push(op), rest))
            }
            PushReg => {
                let (op, rest) = PushOp::try_decode_reg(bytes)?;
                Ok((Instruction::Push(op), rest))
            }
            PopRegRM => {
                let (op, rest) = PopOp::try_decode_rm(bytes)?;
                Ok((Instruction::Pop(op), rest))
            }
            PopReg => {
                let (op, rest) = PopOp::try_decode_reg(bytes)?;
                Ok((Instruction::Pop(op), rest))
            }
            PushPopSeg => match bytes[0] & 0b111 {
                0b110 => {
                    let (op, rest) = PushOp::try_decode_seg_reg(bytes)?;
                    Ok((Instruction::Push(op), rest))
                }
                0b111 => {
                    let (op, rest) = PopOp::try_decode_seg_reg(bytes)?;
                    Ok((Instruction::Pop(op), rest))
                }
                _ => Err(DecodeError::OpCode(format!("{}", bytes[0]))),
            },
        }
    }
}

impl Iterator for Decoder<'_> {
    type Item = Result<Instruction, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }
        match Self::try_decode_next(self.bytes) {
            Ok((instr, rest)) => {
                self.bytes = rest;
                Some(Ok(instr))
            }
            Err(e) => Some(Err(e)),
        }
    }
}
