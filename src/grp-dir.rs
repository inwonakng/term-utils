use clap::Parser;
use regex::Regex;
use std::{collections::HashMap, env, error::Error, fs, path::Path};

fn get_groups(pattern: &str, dir: &Path) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let children = fs::read_dir(dir)?; // Propagate error if reading the directory fails
    let pattern = Regex::new(pattern)?; // Propagate error if regex compilation fails
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for child in children {
        let child = child?;
        let path = child.path();

        if let Some(name) = path.file_name() {
            if let Some(name_str) = name.to_str() {
                if name_str.starts_with(".") {
                    continue;
                }
                if let Some(captures) = pattern.captures(name_str) {
                    if let Some(group) = captures.name("group") {
                        let group_name = group.as_str().to_string();
                        groups
                            .entry(group_name)
                            .or_default()
                            .push(name_str.to_string());
                    }
                }
            }
        }
    }

    Ok(groups) // Return the hashmap on success
}

fn group_files(dir: &Path, group_name: &str, target: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let group_dir = dir.join(group_name);
    if target.len() < 2 {
        println!(
            "Skipping group {} because it has less than 2 matches",
            group_name
        );
        return Ok(());
    };
    if group_dir.exists() {
        return Err(format!("Directory already exists: {:?}", group_dir).into());
    }
    match fs::create_dir(&group_dir) {
        Ok(_) => {
            // now move the files into the directory
            for name in target {
                let source_dir = dir.join(name);
                let target_dir = group_dir.join(name);
                if let Err(e) = fs::rename(&source_dir, &target_dir) {
                    return Err(Box::new(e));
                }
            }
        }
        Err(e) => return Err(Box::new(e)),
    }
    Ok(())
}

// Driver code

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// String pattern to build groups by
    pattern: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let cwd = env::current_dir()?;

    // make a nice pattern that can capture the specified group pattern
    let mut group_pattern: String = "(?<group>".to_owned();
    group_pattern.push_str(&args.pattern);
    group_pattern.push_str(").*");

    match get_groups(&group_pattern, &cwd) {
        Ok(groups) => {
            for (group_name, names) in groups {
                group_files(&cwd, &group_name, &names)?;
            }
            Ok(())
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }
    }
}
