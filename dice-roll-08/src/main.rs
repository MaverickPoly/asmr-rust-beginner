use rand::Rng;
use rand::thread_rng;
use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to dice roller!\n");
    let mut rnd = thread_rng();

    loop {
        print!("Loop? ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == "n" {
            break;
        }

        let dice = rnd.gen_range(1..=6);
        println!("Dice: {dice}");
    }
}
