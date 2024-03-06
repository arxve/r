use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::thread;
use std::sync::{Arc, Mutex};
use colored::*;

// Function to echo the input arguments
fn echo(args: Vec<String>) {
    println!("{}", args.join(" "));
}

// Function to concatenate files
fn cat(files: Vec<String>) {
    for file in files {
        match fs::read_to_string(&file) {
            Ok(contents) => println!("{}", contents),
            Err(e) => eprintln!("Error: Unable to read file '{}': {}", file, e),
        }
    }
}

// Function to list directories
fn ls(directory: &str) {
    match fs::read_dir(directory) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => println!("{}", entry.file_name().to_string_lossy()),
                    Err(e) => eprintln!("Error: Unable to read directory entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: Unable to read directory '{}': {}", directory, e),
    }
}

// Function to locate files or directories
fn find(directory: &str, filename: &str) {
    let paths = match fs::read_dir(directory) {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error: Unable to read directory '{}': {}", directory, e);
            return;
        }
    };

    let found_paths: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for path in paths {
        let found_paths_clone = Arc::clone(&found_paths);
        let directory = directory.to_string();
        handles.push(thread::spawn(move || {
            if let Ok(entry) = path {
                let path = entry.path();
                if path.is_file() && path.file_name().unwrap() == filename {
                    let mut found_paths = found_paths_clone.lock().unwrap();
                    found_paths.push(path);
                }
            }
        }));
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let found_paths = found_paths.lock().unwrap();
    if found_paths.is_empty() {
        println!("File '{}' not found in directory '{}'", filename, directory);
    } else {
        for path in found_paths.iter() {
            println!("{}", path.display());
        }
    }
}

// Function to match text in files
fn grep(filename: &str, pattern: &str) {
    match fs::File::open(filename) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for (line_number, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    if line.contains(pattern) {
                        println!("{}: {}", (line_number + 1).to_string().green(), line);
                    }
                } else {
                    eprintln!("Error: Unable to read line from file '{}'", filename);
                    return;
                }
            }
        }
        Err(e) => eprintln!("Error: Unable to open file '{}': {}", filename, e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command> [arguments]", args[0]);
        return;
    }

    let command = &args[1];
    let arguments = &args[2..];

    match command.as_str() {
        "echo" => echo(arguments.to_vec()),
        "cat" => cat(arguments.to_vec()),
        "ls" => {
            if arguments.is_empty() {
                println!("Usage: {} ls <directory>", args[0]);
                return;
            }
            ls(&arguments[0])
        }
        "find" => {
            if arguments.len() != 2 {
                println!("Usage: {} find <directory> <filename>", args[0]);
                return;
            }
            find(&arguments[0], &arguments[1])
        }
        "grep" => {
            if arguments.len() != 2 {
                println!("Usage: {} grep <filename> <pattern>", args[0]);
                return;
            }
            grep(&arguments[0], &arguments[1])
        }
        _ => println!("Unknown command: {}", command),
    }
}
