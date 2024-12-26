use std::fmt;


macro_rules! create_cond_jump_ops {
    (
        $(
            ($opname:ident, $mnemonic:expr);
        )+
    ) => {

        #[derive(Debug)]
        pub enum CondJumpOp {
            $($opname(i8),)+
        }

        impl fmt::Display for CondJumpOp {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$opname(inc) => get_increment(f, $mnemonic, *inc),)+
                }
            }
        }
    }
}


create_cond_jump_ops! {
    (Equal, "je");
    (Less, "jl");
    (LessEqual, "jle");
    (Below, "jb");
    (BelowEqual, "jbe");
    (ParityEven, "jp");
    (Overflow, "jo");
    (NotEqual, "jnz");
    (Sign, "js");
    (GreaterEqual, "jnl");
    (Greater, "jg");
    (AboveEqual, "jnb");
    (Above, "ja");
    (ParityOdd, "jnp");
    (NotOverflow, "jno");
    (NotSign, "jns");
    (CXZero, "jcxz");
    (Loop, "loop");
    (LoopEqual, "loopz");
    (LoopNEqual, "loopnz");
}


fn get_increment(w: &mut fmt::Formatter<'_>, mnemonic: &str, inc: i8) -> fmt::Result {
    if inc + 2 > 0 {
        write!(w, "{} $+{}+0", mnemonic, inc + 2)?
    } else if inc + 2 == 0 {
        write!(w, "{} $+0", mnemonic)?
    } else {
        write!(w, "{} ${}+0", mnemonic, inc + 2)?
    }
    Ok(())
}
