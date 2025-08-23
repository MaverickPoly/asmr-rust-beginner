use std::io::{stdin, stdout, Write};

fn main() {
    println!("====== Welcome to Calculator ======");
    let (mut num1, mut num2, mut result): (f64, f64, f64) = (0.0, 0.0, 0.0);

    loop {
        println!("\n1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Modula");
        println!("6. Quit");
        print!("Select option (1-6): ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Could not read line!");
        let option: u32 = input.trim().parse().expect("Invalid input!");

        if option == 6 {
            println!("\nGoodbye!");
            break;
        }
        if option <= 0 || option > 6 {
            println!("Invalid option! Please try again!");
            continue;
        }

        input.clear();
        print!("Enter number 1: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        num1 = input.trim().parse().unwrap();

        input.clear();
        print!("Enter number 2: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        num2 = input.trim().parse().unwrap();

        match option {
            1 => result = num1 + num2,
            2 => result = num1 - num2,
            3 => result = num1 * num2,
            4 => result = num1 / num2,
            5 => result = num1 % num2,
            _ => {},
        }

        println!("Result: {}", result);
    }
}
