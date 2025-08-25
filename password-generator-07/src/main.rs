use rand::Rng;
use rand::thread_rng;
use std::io;
use std::io::Write;

fn flush(input: &mut String) {
    input.clear();
    io::stdout().flush().unwrap();
    io::stdin().read_line(input).unwrap();
}

fn main() {
    let mut options = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut input = String::new();
    print!("Include uppercase[y/n]: ");
    flush(&mut input);
    if input.trim().to_lowercase() == "y" {
        options += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }

    print!("Include digits[y/n]: ");
    flush(&mut input);
    if input.trim().to_lowercase() == "y" {
        options += "0123456789";
    }

    print!("Include punctuation[y/n]: ");
    flush(&mut input);
    if input.trim().to_lowercase() == "y" {
        options += "!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~";
    }

    print!("Password size: ");
    flush(&mut input);
    let size: u32 = input.trim().parse().expect("Invalid size!");

    if size > 100 {
        println!("Size is too large!");
        return;
    }

    let len = options.len();
    let mut rng = thread_rng();
    let mut password = String::new();
    for _ in 0..size {
        let i = rng.gen_range(0..len);
        let el = options.chars().nth(i).unwrap();
        password.push(el);
    }
    println!("\nPasssword: {password}")
}
