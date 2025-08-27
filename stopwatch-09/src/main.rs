use std::io::{self, Write};
use std::thread;
use std::time;

fn main() {
    print!("Enter number of seconds: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().expect("Invalid input!");

    println!("Stopwatch started!");

    for i in 0..input {
        println!("{}", input - i);
        thread::sleep(time::Duration::from_secs(1));
    }
    println!("Time is up!");
}
