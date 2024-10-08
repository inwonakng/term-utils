use clap::Parser;
use regex::Regex;
use std::{
    env,
    error::Error,
    fs::{read_dir, File},
    io::{Read, Write},
    path::Path,
};
use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, ZipWriter};

fn zip_directory(path: &Path, options: SimpleFileOptions) -> Result<(), Box<dyn Error>> {
    let zipfile = File::create(path.with_extension("zip"))?;
    let mut writer = ZipWriter::new(zipfile);
    let mut buf = Vec::new();

    for dir in WalkDir::new(path) {
        let dir = dir?;
        let dir_path = dir.path();
        let dir_name = dir_path.strip_prefix(path).unwrap().to_str().unwrap();

        if dir_path.is_file() {
            let mut f = File::open(dir_path)?;
            f.read_to_end(&mut buf)?;
            writer.start_file(dir_name, options)?;
            writer.write_all(&buf)?;
            buf.clear();
        } else if !dir_name.is_empty() {
            writer.add_directory(dir_name, options)?;
        }
    }
    writer.finish()?;
    Ok(())
}

// Driver code

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// String pattern to build groups by
    #[arg(short, long, default_value = ".*")]
    pattern: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let cwd = env::current_dir()?;
    let children = read_dir(cwd)?; // Propagate error if reading the directory fails

    let pattern = Regex::new(&args.pattern)?;

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for child in children {
        let child = child?;
        let path = child.path();
        let name_str = path
            .file_name()
            .ok_or("No file name")?
            .to_str()
            .ok_or("Failed to convert name to string")?;
        if name_str.starts_with(".") || !pattern.is_match(name_str) || !path.is_dir() {
            continue;
        }
        zip_directory(&path, options)?;
    }
    Ok(())
}
