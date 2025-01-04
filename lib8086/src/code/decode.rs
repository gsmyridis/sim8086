use super::ops::*;
use super::{DResult, DecodeError, Instruction, InstructionQueue};
use crate::code::ops::OpCode::*;

pub struct Decoder {
    buffer: Vec<u8>,
}

impl Decoder {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }

    pub fn decode(self) -> Result<InstructionQueue, DecodeError> {
        let mut iqueue = InstructionQueue::default();
        let mut remaining_bytes = self.buffer.as_slice();
        while !remaining_bytes.is_empty() {
            let (instr, size) = Decoder::try_decode_next(remaining_bytes)?;
            iqueue.push(instr, size);
            remaining_bytes = &remaining_bytes[size..];
        }

        iqueue.push(Instruction::Halt, 0);
        Ok(iqueue)
    }

    /// Tries to decode next instruction from the binary representation of the machine code.
    ///
    /// Takes a reference to an array of bytes that represent the machine code, and attempts
    /// to decode the next instruction. If it succeeds, it returns the instruction and a
    /// reference to the array with the rest of the bytes. Otherwise, it returns the decoding.
    /// error.
    pub fn try_decode_next(bytes: &[u8]) -> DResult<Instruction> {
        match OpCode::parse(bytes[0])? {
            MovRegRM => {
                let (op, size) = MovOp::try_parse_reg_rm(bytes)?;
                Ok((Instruction::Mov(op), size))
            }
            MovImRM => {
                let (op, size) = MovOp::try_parse_im_rm(bytes)?;
                Ok((Instruction::Mov(op), size))
            }
            MovImReg => {
                let (op, size) = MovOp::try_parse_im_reg(bytes)?;
                Ok((Instruction::Mov(op), size))
            }
            MovMemAcc => {
                let (op, size) = MovOp::try_decode_mem_acc(bytes)?;
                Ok((Instruction::Mov(op), size))
            }
            MovRMSegReg | MovSegRegRM => {
                let (op, size) = MovOp::try_decode_rm_segreg(bytes)?;
                Ok((Instruction::Mov(op), size))
            }
            NumImRM => {
                let (op, size) = NumOp::try_decode_im_rm(bytes)?;
                Ok((Instruction::Num(op), size))
            }
            AddRMReg => {
                let (op, size) = NumOp::try_decode_rm_reg(bytes, NumOpType::Add)?;
                Ok((Instruction::Num(op), size))
            }
            AdcRMReg => {
                let (op, size) = NumOp::try_decode_rm_reg(bytes, NumOpType::Adc)?;
                Ok((Instruction::Num(op), size))
            }
            SubRMReg => {
                let (op, size) = NumOp::try_decode_rm_reg(bytes, NumOpType::Sub)?;
                Ok((Instruction::Num(op), size))
            }
            SbbRMReg => {
                let (op, size) = NumOp::try_decode_rm_reg(bytes, NumOpType::Sbb)?;
                Ok((Instruction::Num(op), size))
            }
            CmpRMReg => {
                let (op, size) = NumOp::try_decode_rm_reg(bytes, NumOpType::Cmp)?;
                Ok((Instruction::Num(op), size))
            }
            AddImAcc => {
                let (op, size) = NumOp::try_decode_im_acc(bytes, NumOpType::Add)?;
                Ok((Instruction::Num(op), size))
            }
            AdcImAcc => {
                let (op, size) = NumOp::try_decode_im_acc(bytes, NumOpType::Adc)?;
                Ok((Instruction::Num(op), size))
            }
            SubImAcc => {
                let (op, size) = NumOp::try_decode_im_acc(bytes, NumOpType::Sub)?;
                Ok((Instruction::Num(op), size))
            }
            SbbImAcc => {
                let (op, size) = NumOp::try_decode_im_acc(bytes, NumOpType::Sbb)?;
                Ok((Instruction::Num(op), size))
            }
            CmpImAcc => {
                let (op, size) = NumOp::try_decode_im_acc(bytes, NumOpType::Cmp)?;
                Ok((Instruction::Num(op), size))
            }
            JumpEqual => Ok((Instruction::CondJump(CondJumpOp::Equal(bytes[1] as i8)), 2)),
            JumpLess => Ok((Instruction::CondJump(CondJumpOp::Less(bytes[1] as i8)), 2)),
            JumpLessEq => Ok((Instruction::CondJump(CondJumpOp::LessEqual(bytes[1] as i8)), 2)),
            JumpBelow => Ok((Instruction::CondJump(CondJumpOp::Below(bytes[1] as i8)), 2)),
            JumpBelowEq => Ok((Instruction::CondJump(CondJumpOp::BelowEqual(bytes[1] as i8)), 2)),
            JumpParityEven => Ok((Instruction::CondJump(CondJumpOp::ParityEven(bytes[1] as i8)), 2)),
            JumpOverflow => Ok((Instruction::CondJump(CondJumpOp::Overflow(bytes[1] as i8)), 2)),
            JumpNEqual => Ok((Instruction::CondJump(CondJumpOp::NotEqual(bytes[1] as i8)), 2)),
            JumpSign => Ok((Instruction::CondJump(CondJumpOp::Sign(bytes[1] as i8)), 2)),
            JumpGreaterEq => Ok((
                Instruction::CondJump(CondJumpOp::GreaterEqual(bytes[1] as i8)),
                2,
            )),
            JumpGreater => Ok((Instruction::CondJump(CondJumpOp::Greater(bytes[1] as i8)), 2)),
            JumpAboveEq => Ok((Instruction::CondJump(CondJumpOp::AboveEqual(bytes[1] as i8)), 2)),
            JumpAbove => Ok((Instruction::CondJump(CondJumpOp::Above(bytes[1] as i8)), 2)),
            JumpParityOdd => Ok((Instruction::CondJump(CondJumpOp::ParityOdd(bytes[1] as i8)), 2)),
            JumpNOverflow => Ok((
                Instruction::CondJump(CondJumpOp::NotOverflow(bytes[1] as i8)),
                2,
            )),
            JumpNSign => Ok((Instruction::CondJump(CondJumpOp::NotSign(bytes[1] as i8)), 2)),
            Loop => Ok((Instruction::CondJump(CondJumpOp::Loop(bytes[1] as i8)), 2)),
            LoopEqual => Ok((Instruction::CondJump(CondJumpOp::LoopEqual(bytes[1] as i8)), 2)),
            LoopNEqual => Ok((Instruction::CondJump(CondJumpOp::LoopNEqual(bytes[1] as i8)), 2)),
            JumpCXZero => Ok((Instruction::CondJump(CondJumpOp::CXZero(bytes[1] as i8)), 2)),
            PushRegRM => {
                let (op, size) = PushOp::try_decode_rm(bytes)?;
                Ok((Instruction::Push(op), size))
            }
            PushReg => {
                let (op, size) = PushOp::try_decode_reg(bytes)?;
                assert_eq!(size, 1);
                Ok((Instruction::Push(op), size))
            }
            PopRegRM => {
                let (op, size) = PopOp::try_decode_rm(bytes)?;
                Ok((Instruction::Pop(op), size))
            }
            PopReg => {
                let (op, size) = PopOp::try_decode_reg(bytes)?;
                assert_eq!(size, 1);
                Ok((Instruction::Pop(op), size))
            }
            PushPopSeg => match bytes[0] & 0b111 {
                0b110 => {
                    let (op, size) = PushOp::try_decode_seg_reg(bytes)?;
                    assert_eq!(size, 1);
                    Ok((Instruction::Push(op), size))
                }
                0b111 => {
                    let (op, size) = PopOp::try_decode_seg_reg(bytes)?;
                    assert_eq!(size, 1);
                    Ok((Instruction::Pop(op), size))
                }
                _ => Err(DecodeError::OpCode(format!("{}", bytes[0]))),
            },
        }
    }
}
