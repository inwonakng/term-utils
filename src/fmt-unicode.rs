use std::{env, error::Error, fs};
use unicode_normalization::UnicodeNormalization;

// Driver code

fn main() -> Result<(), Box<dyn Error>> {
    let cwd = env::current_dir()?;
    let children = fs::read_dir(cwd.clone())?;

    for child in children {
        let path = child?.path();
        let name_str = path
            .file_name()
            .ok_or("No file name")?
            .to_str()
            .ok_or("Failed to convert name to string")?;
        if name_str.starts_with(".") {
            continue;
        }
        let normalized: String = name_str.nfc().collect();
        if normalized != name_str {
            let normalized_path = cwd.join(&normalized);
            fs::rename(&path, &normalized_path)?;
        }
    }
    Ok(())
}
