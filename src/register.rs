use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Register {
    AL, // Low byte of A register.
    BL, // Low byte of B register.
    CL, // Low byte of C register.
    DL, // Low byte of D register.
    AH, // High byte of A register.
    BH, // High byte of B register.
    CH, // High byte of C register.
    DH, // High Byte of D register.
    AX, // Word of A register.
    BX, // Word of B register.
    CX, // Word of C register.
    DX, // Word of D register.
    SP, // Stack Pointer
    BP, //
    SI, //
    DI, //
}

impl Register {
    /// Creates a register instance from a combination of code bytes and W bit.
    pub fn from(code: u8, w: bool) -> Self {
        if code == 0b0 && w == false {
            Register::AL
        } else if code == 0b000 && w == true {
            Register::AX
        } else if code == 0b001 && w == false {
            Register::CL
        } else if code == 0b001 && w == true {
            Register::CX
        } else if code == 0b010 && w == false {
            Register::DL
        } else if code == 0b010 && w == true {
            Register::DX
        } else if code == 0b011 && w == false {
            Register::BL
        } else if code == 0b011 && w == true {
            Register::BX
        } else if code == 0b100 && w == false {
            Register::AH
        } else if code == 0b100 && w == true {
            Register::SP
        } else if code == 0b101 && w == false {
            Register::CH
        } else if code == 0b101 && w == true {
            Register::BP
        } else if code == 0b110 && w == false {
            Register::DH
        } else if code == 0b110 && w == true {
            Register::SI
        } else if code == 0b111 && w == false {
            Register::BH
        } else if code == 0b111 && w == true {
            Register::DI
        } else {
            panic!("Invalid combination of code and w");
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::AH => write!(f, "ah"),
            Register::BH => write!(f, "bh"),
            Register::CH => write!(f, "ch"),
            Register::DH => write!(f, "dh"),
            Register::AL => write!(f, "al"),
            Register::BL => write!(f, "bl"),
            Register::CL => write!(f, "cl"),
            Register::DL => write!(f, "dl"),
            Register::AX => write!(f, "ax"),
            Register::BX => write!(f, "bx"),
            Register::CX => write!(f, "cx"),
            Register::DX => write!(f, "dx"),
            Register::SP => write!(f, "sp"),
            Register::BP => write!(f, "bp"),
            Register::SI => write!(f, "si"),
            Register::DI => write!(f, "di"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_registers() {
        assert_eq!(Register::from(0b000, false), Register::AL);
        assert_eq!(Register::from(0b000, true), Register::AX);
        assert_eq!(Register::from(0b001, false), Register::CL);
        assert_eq!(Register::from(0b001, true), Register::CX);
        assert_eq!(Register::from(0b010, false), Register::DL);
        assert_eq!(Register::from(0b010, true), Register::DX);
        assert_eq!(Register::from(0b011, false), Register::BL);
        assert_eq!(Register::from(0b011, true), Register::BX);
        assert_eq!(Register::from(0b100, false), Register::AH);
        assert_eq!(Register::from(0b100, true), Register::SP);
        assert_eq!(Register::from(0b101, false), Register::CH);
        assert_eq!(Register::from(0b101, true), Register::BP);
        assert_eq!(Register::from(0b110, false), Register::DH);
        assert_eq!(Register::from(0b110, true), Register::SI);
        assert_eq!(Register::from(0b111, false), Register::BH);
        assert_eq!(Register::from(0b111, true), Register::DI);
    }
}
