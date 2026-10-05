use anyhow::{Context, Result, bail};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use clap::{Parser, Subcommand};
use std::io::{IsTerminal, Read};
use std::path::PathBuf;
use std::{fs, io};

use jftag::{decode, encode};

fn encode_str(input: &str) -> Result<String> {
    let value: serde_json::Value = serde_json::from_str(input).context("invalid JSON")?;
    encode(&value)
}

fn decode_str(input: &str, pretty: bool) -> Result<String> {
    let bytes = URL_SAFE_NO_PAD
        .decode(input.trim())
        .context("invalid encoded text")?;
    decode(&bytes, pretty)
}

#[derive(Parser)]
#[command(version, about = "Compress JSON into encoded text and back")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// JSON -> compressed encoded text
    Encode {
        /// Input file; omit or use `-` to read from stdin
        file: Option<PathBuf>,
    },
    /// Encoded text -> JSON
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
