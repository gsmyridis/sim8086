use crate::code::DecodeError;

macro_rules! create_opcodes {
    (
        $(
            ($name:ident, $code_str:expr);
        )+
    ) => {

        #[derive(Debug)]
        pub enum OpCode {
            $($name,)+
        }

        impl OpCode {
            /// Parses a byte and extracts an instruction OPCODE.
            pub fn parse(byte: u8) -> Result<Self, DecodeError> {
                let bstr = format!("{:08b}", byte);

                match bstr {
                    $(s if s.starts_with($code_str) => Ok(Self::$name),)+
                    _ => Err(DecodeError::OpCode(bstr))
                }
            }
        }
    }
}

create_opcodes! {
    (MovRegRM, "100010");
    (MovImRM, "1100011");
    (MovImReg, "1011");
    (MovMemAcc, "101000");
    (MovRMSegReg, "10001110");
    (MovSegRegRM, "10001100");

    (NumImRM, "100000");

    (AddRMReg, "000000");
    (AdcRMReg, "000100");
    (SubRMReg, "001010");
    (SbbRMReg, "000110");
    (CmpRMReg, "001110");

    (AddImAcc, "0000010");
    (AdcImAcc, "0000010");
    (SubImAcc, "0010110");
    (SbbImAcc, "0001110");
    (CmpImAcc, "0011110");

    (JumpEqual, "01110100");
    (JumpLess, "01111100");
    (JumpLessEq, "01111110");
    (JumpBelow, "01110010");
    (JumpBelowEq, "01110110");
    (JumpParityEven, "01111010");
    (JumpOverflow, "01110000");
    (JumpNEqual, "01110101");
    (JumpSign, "01111000");
    (JumpGreaterEq, "01111101");
    (JumpGreater, "01111111");
    (JumpAboveEq, "01110011");
    (JumpAbove, "01110111");
    (JumpParityOdd, "01111011");
    (JumpNOverflow, "01110001");
    (JumpNSign, "01111001");
    (JumpCXZero, "11100011");
    (Loop, "11100010");
    (LoopEqual, "11100001");
    (LoopNEqual, "11100000");

    (PushRegRM, "11111111");
    (PushReg, "01010");
    (PopRegRM, "10001111");
    (PopReg, "01011");
    (PushPopSeg, "000");

    (Halt, "11110100");
}
