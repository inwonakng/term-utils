use clap::Parser;
use regex::Regex;
use std::{cmp::max, env, error::Error, fs};

fn build_pattern(pattern: &str) -> Result<String, Box<dyn Error>> {
    // first check if the pattern is following the rules
    // pattern should only conatain one instance of \d+
    if pattern.matches(r"\d+").count() != 1 {
        return Err("Pattern should contain exactly one instance of \\d+".into());
    }
    if pattern.contains(")") || pattern.contains("(") {
        return Err("Pattern should not contain any parentheses".into());
    }

    // This regex matches the pattern with one instance of \d+
    let re = Regex::new(r"^(.*?)(\\d\+)(.*?)$")?;

    // Capture the groups
    let captures = re.captures(pattern).ok_or("Error transforming regex")?;
    let part1 = captures.get(1).map_or("", |m| m.as_str());
    let part2 = captures.get(2).map_or("", |m| m.as_str());
    let part3 = captures.get(3).map_or("", |m| m.as_str());

    let formatted = format!("({})(?<num>{})({})", part1, part2, part3)
        .replace(")()", ")")
        .replace("()(", "(")
        .replace("()", "");
    Ok(formatted)
}

// Driver code

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// String pattern to build match for digits
    pattern: String,
    /// Number of zeros to pad the digits with
    #[arg(short, long, default_value = "1")]
    zeros: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let cwd = env::current_dir()?;

    let dirty_pattern = build_pattern(&args.pattern)?;
    println!("Pattern: {}", dirty_pattern);
    let pattern = Regex::new(&dirty_pattern)?;

    // Now find all file/directories that have a match with the pattern
    let mut children: Vec<String> = Vec::new();
    let mut max_num_len: usize = args.zeros as usize;
    for child in fs::read_dir(cwd.clone())? {
        let child = child?;
        let path = child.path();

        let name_str = path
            .file_name()
            .ok_or("No file name")?
            .to_str()
            .ok_or("Failed to convert name to string")?;
        if name_str.starts_with(".") || !pattern.is_match(name_str) {
            continue;
        }
        let num: &str = &pattern.captures(name_str).unwrap()["num"];
        children.push(name_str.to_string());
        max_num_len = max(num.len(), max_num_len);
    }

    for child in children {
        let captures = pattern.captures(&child).ok_or("No match")?;
        let num: u32 = captures["num"].parse()?;
        let mut parsed_name = String::new();
        for c in captures.iter().skip(1) {
            let capt = c.ok_or("No captuer")?;
            if capt.as_str() == num.to_string() {
                parsed_name.push_str(&format!("{:0max_num_len$}", num));
            } else {
                parsed_name.push_str(capt.as_str());
            }
        }
        let source_dir = cwd.join(child);
        let target_dir = cwd.join(parsed_name.clone());
        if let Err(e) = fs::rename(&source_dir, &target_dir) {
            return Err(Box::new(e));
        }
    }
    Ok(())
}
