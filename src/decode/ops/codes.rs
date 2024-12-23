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
    (MovMemAcc, "1010000");
    (MovAccMem, "1010001");
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

}
