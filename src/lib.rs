use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use flate2::{Compression, read::DeflateDecoder, write::DeflateEncoder};
use std::io::{Read, Write};

pub fn encode(value: &serde_json::Value) -> Result<String> {
    let minified = serde_json::to_vec(value)?;
    let mut enc = DeflateEncoder::new(Vec::new(), Compression::best());
    enc.write_all(&minified)?;
    Ok(URL_SAFE_NO_PAD.encode(enc.finish()?))
}

pub fn decode(input: &[u8], pretty: bool) -> Result<String> {
    let mut json = String::new();
    DeflateDecoder::new(input)
        .read_to_string(&mut json)
        .context("corrupt compressed data")?;
    let value: serde_json::Value = serde_json::from_str(&json)?;
    Ok(if pretty {
        serde_json::to_string_pretty(&value)
    } else {
        serde_json::to_string(&value)
    }?)
}
