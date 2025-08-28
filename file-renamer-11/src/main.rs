use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut old = String::new();
    print!("Enter old filename: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut old).unwrap();

    if old.trim().is_empty() {
        eprintln!("Invalid input!");
        return;
    }
    let old = Path::new(old.trim());
    if !old.is_file() {
        eprintln!("Not a file!");
        return;
    }
    if !old.exists() {
        eprintln!("File not found!");
        return;
    }

    let mut new = String::new();
    print!("Enter new filename: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new).unwrap();

    if new.trim().is_empty() {
        eprintln!("Invalid input!");
        return;
    }
    let new = Path::new(new.trim());
    if new.exists() {
        eprintln!("Already exists!");
        return;
    }

    if let Err(err) = fs::rename(old, new) {
        eprintln!("Failed to rename file: {}", err);
    } else {
        eprintln!("File renamed successfully!");
    }
}
