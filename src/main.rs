use std::fs;
use std::io::prelude::*;

mod decode;
use decode::error::DecodeError;

mod register;

use decode::instr::Instruction;

const BIN_PATH: &str = "../computer_enhance/perfaware/part1/listing_0039_more_movs";

fn main() -> Result<(), DecodeError> {
    let mut file = fs::File::open(BIN_PATH).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read bytes from file.");
    for n in &buffer {
        println!("{:x}", n);
    }
    let instr = Instruction::try_parse(buffer.as_ref())?;
    for instruction in instr {
        println!("\n\n");
        println!("{instruction:?}");
        println!("{instruction}");
    }
    Ok(())
}
