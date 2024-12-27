use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod code;
use code::{DecodeError, Decoder};

mod sim;

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
            let mut file = fs::File::open(path).expect("Failed to open file.");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .expect("Failed to read bytes from file.");

            let decoder = Decoder::from(&buffer);
            let asm = decoder
                .into_iter()
                .map(|i| i.unwrap().to_string())
                .collect::<Vec<String>>()
                .join("\n");

            let mut output = fs::File::create(output).expect("Failed to create new output file.");
            write!(output, "{asm}").expect("Failed to write output.");
        }
        _ => todo!(),
    }
    Ok(())
}
