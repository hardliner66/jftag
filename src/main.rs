use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use std::io::{IsTerminal, Read};
use std::path::PathBuf;
use std::{fs, io};

use jftag::{decode_str, encode_str};

#[derive(Parser)]
#[command(version, about = "Compress JSON into encoded text and back")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// JSON -> compressed encoded text
    #[command(aliases=["enc", "e"])]
    Encode {
        /// Input file; omit or use `-` to read from stdin
        file: Option<PathBuf>,
    },
    /// Encoded text -> JSON
    #[command(aliases=["dec", "d"])]
    Decode {
        #[arg(short, long)]
        pretty: bool,
        /// Input file; omit or use `-` to read from stdin
        file: Option<PathBuf>,
    },
}

fn read_input(file: Option<PathBuf>) -> Result<String> {
    let mut input = String::new();
    match file {
        Some(path) if path.as_os_str() != "-" => {
            input =
                fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        }
        _ => {
            if io::stdin().is_terminal() {
                bail!("no input: pass a file or pipe data via stdin");
            }
            io::stdin().read_to_string(&mut input)?;
        }
    }
    Ok(input)
}

fn main() -> Result<()> {
    let out = match Cli::parse().command {
        Command::Encode { file } => encode_str(&read_input(file)?)?,
        Command::Decode { file, pretty } => decode_str(&read_input(file)?, pretty)?,
    };
    println!("{out}");
    Ok(())
}
