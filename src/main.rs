use std::fs;
use std::io::prelude::*;

mod decode;
use decode::error::DecodeError;

mod register;

use decode::Instruction;

const BIN_PATH: &str = "../computer_enhance/perfaware/part1/listing_0042_completionist_decode";

fn main() -> Result<(), DecodeError> {
    let mut file = fs::File::open(BIN_PATH).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read bytes from file.");
    let _instr = Instruction::try_decode(buffer.as_ref())?;
    Ok(())
}
