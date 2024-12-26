use std::fs;
use std::io::prelude::*;

use clap::Parser;

mod decode;
use decode::{DecodeError, Instruction};

#[derive(Parser)]
struct Cli {
    path: String,
}

fn main() -> Result<(), DecodeError> {
    let cli = Cli::parse();
    let mut file = fs::File::open(cli.path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read bytes from file.");
    let _instr = Instruction::try_decode(buffer.as_ref())?;
    Ok(())
}
