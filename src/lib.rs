use anyhow::{Context, Result};
use std::io::Write;

pub fn find_matches<O: Write>(pattern: &str, content: &str, o: &mut O) -> Result<()> {
    let mut is_match: bool = false;

    if pattern.is_empty() {
        return Err(std::fmt::Error).context("empty pattern");
    }

    for line in content.lines() {
        if line.contains(pattern) {
            is_match = true;
            writeln!(o, "{}", line).context("could not write to specified output")?;
            o.flush().context("couldn't flush the output properly")?;
        }
    }

    if is_match.eq(&false) {
        writeln!(o, "no match found").context("counld not write to specified output")?;
    }

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
