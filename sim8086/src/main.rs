use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use lib8086::{Cpu, DecodeError, Decoder};

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
        output: Option<PathBuf>,
    },

    Execute {
        path: PathBuf,

        #[arg(long, short)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<(), DecodeError> {
    let cli = Cli::parse();
    match cli.command {
        Command::Decode { path, output } => {
            let buffer = fs::read(path).expect("Failed to read input byte-code file.");
            let decoder = Decoder::new(buffer);
            let iqueue = decoder.decode()?;
            let asm = iqueue.to_string();

            match output {
                Some(opath) => {
                    let mut output =
                        fs::File::create(opath).expect("Failed to create new output file.");
                    write!(output, "{asm}").expect("Failed to write output.")
                }
                None => println!("{asm}"),
            }
        }
        Command::Execute { path, output } => {
            let buffer = fs::read(path).expect("Failed to read input byte-code file.");
            let decoder = Decoder::new(buffer);
            let iqueue = decoder.decode()?;

            let mut cpu = Cpu::new();
            cpu.execute(&iqueue).unwrap();

            println!("\nINSTRUCTIONS");
            println!("-------------------");
            println!("{iqueue}");
            println!("{cpu}");

            if let Some(opath) = output {
                let mut file = fs::File::create(opath).expect("Failed to create new output file");
                file.write_all(&cpu.mem).expect("Failed to write memory into output file");
            }
        }
    }
    Ok(())
}
