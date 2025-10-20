use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let mut file_list: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(current_dir) {
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

    Ok(())
}
