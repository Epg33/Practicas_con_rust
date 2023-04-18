use clap::Parser;
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Couldn´t read file `{}`", path))?;
    println!("File content: {}", content);
    Ok(())
}
