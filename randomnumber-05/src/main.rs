use rand::Rng;
use rand::thread_rng;
use std::cmp;
use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to this random number guessing game!");
    println!("You need to guess a number between 0-100\n");

    let mut rng = thread_rng();
    let random_number = rng.gen_range(0..=100);
    let mut guess = 0;

    loop {
        print!("Take a guess: ");
        io::stdout().flush().unwrap();

        guess += 1;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input!");
        let input: i32 = input.trim().parse().expect("Invalid input!");

        match input.cmp(&random_number) {
            cmp::Ordering::Less => {
                println!("Too low!");
            }
            cmp::Ordering::Greater => {
                println!("Too high!");
            }
            cmp::Ordering::Equal => {
                println!("Correct!");
                println!("The number: {}", random_number);
                println!("Guesses: {}", guess);
                break;
            }
        }
    }
}
