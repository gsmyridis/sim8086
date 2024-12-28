use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod code;
use code::{DecodeError, Decoder};

mod sim;
use sim::Cpu;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Decode {
        path: PathBuf,

        #[arg(long, short)]
        output: PathBuf,
    },

    Execute {
        path: PathBuf,

        #[arg(long, short)]
        output: PathBuf,
    },
}

fn main() -> Result<(), DecodeError> {
    let cli = Cli::parse();
    match cli.command {
        Command::Decode { path, output } => {
            let buffer = std::fs::read(path).expect("Failed to read input byte-code file.");
            let decoder = Decoder::from(&buffer);
            let asm = decoder
                .into_iter()
                .map(|i| i.unwrap().to_string())
                .collect::<Vec<_>>()
                .join("\n");

            let mut output = fs::File::create(output).expect("Failed to create new output file.");
            write!(output, "{asm}").expect("Failed to write output.");
        }
        Command::Execute { path, output } => {
            let buffer = std::fs::read(path).expect("Failed to read input byte-code file.");
            let decoder = Decoder::from(&buffer);
            let instr = decoder.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

            let mut cpu = Cpu::default();

            for i in instr {
                cpu.execute(i).unwrap();
            }

            println!("{cpu}");
        }
    }
    Ok(())
}
