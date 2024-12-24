pub mod codes;
pub use codes::OpCode;

pub mod mov;
pub use mov::MovOp;

pub mod num;
pub use num::{NumOp, NumOpType};

pub mod jump;
pub use jump::CondJumpOp;
