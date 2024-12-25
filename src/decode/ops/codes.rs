use crate::decode::error::DecodeError;

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
    // MovOps
    (MovRegRM, "100010");
    (MovImRM, "1100011");
    (MovImReg, "1011");
    (MovMemAcc, "101000");
    (MovRMSegReg, "10001110");
    (MovSegRegRM, "10001100");

    // Arithmetic Ops
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

    // Jump Ops
    (JumpEqual, "01110100");        // Jump on Equal / Jump on Zero
    (JumpLess, "01111100");         // Jump on Less
    (JumpLessEq, "01111110");       // Jump on Less or Equal
    (JumpBelow, "01110010");        // Jump on Below
    (JumpBelowEq, "01110110");      // Jump on Below or Equal
    (JumpParityEven, "01111010");   // Jump on Parity Even
    (JumpOverflow, "01110000");     // Jump on Overflow
    (JumpNEqual, "01110101");       // Jump on Not Equal / Jump on Not Zero
    (JumpSign, "01111000");         // Jump on Sign
    (JumpGreaterEq, "01111101");    // Jump on Greater or Equal
    (JumpGreater, "01111111");      // Jump on Greater
    (JumpAboveEq, "01110011");      // Jump on Above or Equal
    (JumpAbove, "01110111");        // Jump on Above
    (JumpParityOdd, "01111011");    // Jump on Odd Parity
    (JumpNOverflow, "01110001");    // Jump on Not Overflow
    (JumpNSign, "01111001");        // Jump on Not Sign
    (JumpCXZero, "11100011");       // Jump on CX Zero

    // Loop Ops
    (Loop, "11100010");             // Loop CX times
    (LoopEqual, "11100001");        // Loop while Equal / Zero
    (LoopNequal, "11100000");       // Loop while Not Equal / Zero

    // Push Pop Ops
    (PushRegRM, "11111111");
    (PushReg, "01010");
    (PopRegRM, "10001111");
    (PopReg, "01011");
    (PushPopSeg, "000");
}
