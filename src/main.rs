use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, Read};

/// Search for the pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for.
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(args.path.as_path()).context("could not read file")?;
    let mut content = BufReader::new(file);
    let mut content_str = String::new();

    content.read_to_string(&mut content_str)?;

    grrs::find_matches(&args.pattern, &content_str, &mut io::stdout())?;

    Ok(())
}
