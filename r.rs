use std::env;
use std::fs;
use std::io::{self, BufRead};
//use std::path::Path;

// Function to echo the input arguments
fn echo(args: Vec<String>) {
    println!("{}", args.join(" "));
}

// Function to concatenate files
fn cat(files: Vec<String>) {
    for file in files {
        // Attempt to read the content of each file
        if let Ok(contents) = fs::read_to_string(&file) {
            // Print the content of each file
            println!("{}", contents);
        } else {
            // Print an error message if reading fails
            eprintln!("Error: Unable to read file '{}'", file);
        }
    }
}

// Function to list directories
fn ls(directory: &str) {
    // Attempt to read the directory
    if let Ok(entries) = fs::read_dir(directory) {
        // Iterate over each entry in the directory
        for entry in entries {
            // Attempt to get the file name of each entry
            if let Ok(entry) = entry {
                // Print the file name
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
    } else {
        // Print an error message if reading fails
        eprintln!("Error: Unable to read directory '{}'", directory);
    }
}

// Function to locate files or directories
fn find(directory: &str, filename: &str) {
    let mut found = false;
    // Attempt to read the directory
    if let Ok(entries) = fs::read_dir(directory) {
        // Iterate over each entry in the directory
        for entry in entries {
            // Attempt to get the file name of each entry
            if let Ok(entry) = entry {
                let path = entry.path();
                // Check if the entry is a file and its name matches the given filename
                if path.is_file() && path.file_name().unwrap() == filename {
                    // Print the path of the found file
                    println!("{}", path.display());
                    found = true;
                }
            }
        }
    }
    // Print a message if the file is not found
    if !found {
        println!("File '{}' not found in directory '{}'", filename, directory);
    }
}

// Function to match text in files
fn grep(filename: &str, pattern: &str) {
    // Attempt to open the file
    if let Ok(file) = fs::File::open(filename) {
        let reader = io::BufReader::new(file);
        // Iterate over each line in the file
        for (line_number, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                // Check if the line contains the pattern
                if line.contains(pattern) {
                    // Print the line number and the line content
                    println!("{}: {}", line_number + 1, line);
                }
            }
        }
    } else {
        // Print an error message if opening the file fails
        eprintln!("Error: Unable to open file '{}'", filename);
    }
}

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are enough arguments
    if args.len() < 2 {
        println!("Usage: {} <command> [arguments]", args[0]);
        return;
    }

    // Extract the command and arguments
    let command = &args[1];
    let arguments = &args[2..];

    // Match the command to decide which function to call
    match command.as_str() {
        "echo" => echo(arguments.to_vec()),
        "cat" => cat(arguments.to_vec()),
        "ls" => ls(&arguments[0]),
        "find" => {
            // Ensure 'find' command has the correct number of arguments
            if arguments.len() != 2 {
                println!("Usage: {} find <directory> <filename>", args[0]);
                return;
            }
            find(&arguments[0], &arguments[1])
        }
        "grep" => {
            // Ensure 'grep' command has the correct number of arguments
            if arguments.len() != 2 {
                println!("Usage: {} grep <filename> <pattern>", args[0]);
                return;
            }
            grep(&arguments[0], &arguments[1])
        }
        _ => println!("Unknown command: {}", command),
    }
}
