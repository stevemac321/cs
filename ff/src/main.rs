use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use walkdir::WalkDir;
use regex::Regex;

fn search_in_file(file_path: &Path, search_term: &Regex) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if search_term.is_match(&line) {
            println!("Match found in {}:{}: {}", file_path.display(), line_number + 1, line);
        }
    }

    Ok(())
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file_extension_pattern> <search_term>", args[0]);
        std::process::exit(1);
    }

    // Extract file extension and search term from arguments
    let file_extension_pattern = &args[1];
    let search_term_str = &args[2];

    // Convert search term to a Regex
    let search_term = match Regex::new(search_term_str) {
        Ok(re) => Arc::new(re),
        Err(err) => {
            eprintln!("Invalid regex pattern: {}", err);
            std::process::exit(1);
        }
    };

    let file_paths = Arc::new(Mutex::new(Vec::new())); // Shared, thread-safe vector for storing file paths

    // Manually walk through the directory and find files matching the pattern
    let extension = &file_extension_pattern[2..]; // Strip off "*." from the beginning of the pattern
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
            if ext == extension {
                file_paths.lock().unwrap().push(path.to_path_buf());
            }
        }
    }

    let mut handles = vec![];
    for _ in 0..12 { // Create 12 threads
        let search_term = Arc::clone(&search_term);
        let file_paths = Arc::clone(&file_paths);

        let handle = thread::spawn(move || {
            loop {
                let path: Option<std::path::PathBuf> = {
                    let mut paths = file_paths.lock().unwrap();
                    if paths.is_empty() {
                        break;
                    }
                    paths.pop()
                };

                if let Some(file_path) = path {
                    if let Err(err) = search_in_file(&file_path, &search_term) {
                        eprintln!("Error searching in file {}: {}", file_path.display(), err);
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

