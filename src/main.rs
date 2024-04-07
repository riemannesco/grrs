use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};

/// Search for the pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for.
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn find_matches<O: Write>(pattern: &str, content: &str, o: &mut O) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(o, "{}", line).context("could not write to specified output")?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {}", args.path.display()))?;

    find_matches(&args.pattern, &content, &mut io::stdout())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        let _ = find_matches("lorem", "lorem ipsum\ndolor sit amet", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}
