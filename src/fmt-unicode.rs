use std::{env, error::Error, fs};
use unicode_normalization::UnicodeNormalization;

// Driver code

fn main() -> Result<(), Box<dyn Error>> {
    let cwd = env::current_dir()?;
    let children = fs::read_dir(cwd.clone())?;

    for child in children {
        let child = child?;
        let path = child.path();
        if let Some(name) = path.file_name() {
            if let Some(name_str) = name.to_str() {
                if name_str.starts_with(".") {
                    continue;
                }
                let normalized: String = name_str.nfc().collect();
                if normalized != name_str {
                    let normalized_path = cwd.join(&normalized);
                    if let Err(e) = fs::rename(&path, &normalized_path) {
                        return Err(Box::new(e));
                    }
                }
            }
        }
    }
    Ok(())
}
