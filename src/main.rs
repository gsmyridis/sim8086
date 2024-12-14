use std::fs;
use std::io::prelude::*;

mod decode;
mod register;

use decode::instr::Instruction;

const BIN_PATH: &str = "../computer_enhance/perfaware/part1/listing_0038_many_register_mov";

fn main() {
    let mut file = fs::File::open(BIN_PATH).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read bytes from file.");
    for el in &buffer {
        println!("{el:x}");
    }
    let instr = Instruction::try_decode(buffer.as_ref()).unwrap();
    println!("{instr:?}");
}
