//! `loom` - the Uldren Loom command-line tool.
//!
//! Licensed under BUSL-1.1 (see the workspace `LICENSE`). © Uldren Technologies LLC.

use std::io::Read;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use loom_core::{Object, VERSION};

#[derive(Parser)]
#[command(
    name = "loom",
    version,
    about = "Uldren Loom - a universal, content-addressed, versioned filesystem",
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print version information.
    Version,
    /// Hash bytes from a file (or `-` for stdin) and print the Blob content address.
    Hash {
        /// Path to a file, or `-` to read from standard input.
        path: String,
    },
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Version => {
            println!("loom {VERSION}");
            ExitCode::SUCCESS
        }
        Command::Hash { path } => match read_input(&path) {
            Ok(bytes) => {
                println!("{}", Object::Blob(bytes).digest());
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("error: {err}");
                ExitCode::FAILURE
            }
        },
    }
}

fn read_input(path: &str) -> std::io::Result<Vec<u8>> {
    if path == "-" {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf)?;
        Ok(buf)
    } else {
        std::fs::read(path)
    }
}
