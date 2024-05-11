use std::io::Write;
use std::{fs, process::Command};
use tempfile::NamedTempFile;

pub fn edit_with_vim(content: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    // Create a temporary file.
    let mut file = NamedTempFile::new()?;
    writeln!(file, "{}", content.unwrap_or_default())?;

    // Get the path of the temporary file.
    let file_path = file.path().to_str().unwrap_or_default().to_string();

    // Launch Vim on the file.
    Command::new("vim").arg(&file_path).status()?;
    // Read the contents of the file back into a String.
    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}
