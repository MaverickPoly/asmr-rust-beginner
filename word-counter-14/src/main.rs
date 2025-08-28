use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("text.txt").unwrap();
    let reader = BufReader::new(file);
    let mut map: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();

        for word in words {
            let word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            *map.entry(word).or_insert(0) += 1;
        }
    }

    println!("Word occurrences:");
    for word in map.keys() {
        println!("{}: {}", word, map[word]);
    }
}
