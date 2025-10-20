use std::{fs, path::PathBuf};
use clap::Parser;

#[derive(Parser)]
struct Args {
    paths: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut first = true;

    for path in &args.paths {
        if path.is_file() {
            println!("{}", path.display());
            continue;
        }

        if args.paths.len() > 1 {
            println!("{}{}:", if first {""} else {"\n"}, path.display());
        }
        first = false;

        let mut file_list: Vec<String> = Vec::new();

        // Iterate through files and folders
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let filename = match entry.file_name().into_string() {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Error while processing file \"{:?}\": {}", entry.file_name(), e.display());
                            continue;
                        }
                    };
                    if filename.starts_with(".") {
                        continue;
                    }
                    file_list.push(filename);
                }
            }
        }

        // Sort
        file_list.sort();

        // Print
        for file in file_list {
            println!("{file}");
        }
    }

    Ok(())
}
