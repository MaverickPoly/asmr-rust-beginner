use std::fs;
use std::io;
use std::io::BufRead;


fn read_to_string() {
    let file_contents = fs::read_to_string("./file.txt").expect("Error reading file.");
    println!("{}", file_contents);
}

fn main() {
    let file = fs::File::open("./file.txt").expect("Error opening file.");
    let reader = io::BufReader::new(file);

    let mut total_lines = 0usize;
    let mut total_words = 0usize;
    for line in reader.lines() {
        let line = line.unwrap();

        let words_count = line.trim().split(" ").count();
        total_words += words_count;

        println!("{}", line);
        total_lines += 1;
    }

    println!("\nTotal lines: {}", total_lines);
    println!("Total words: {}", total_words);
}
