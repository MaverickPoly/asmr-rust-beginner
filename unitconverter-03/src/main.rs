use std::io;
use std::io::{Write};

fn main() {
    println!("====== Welcome to unit converter! ======");
    let mut input = String::new();

    loop {
        println!("\n1. KM -> Miles");
        println!("2. Miles -> KM");
        println!("3. Exit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        let option = input.trim().parse::<u32>().expect("Invalid input!");

        if option == 3 {
            break;
        }

        input.clear();
        if option == 1 {
            print!("Enter km: ");
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut input).expect("Failed to read line!");
            let kms = input.trim().parse::<f64>().expect("Invalid input!");
            let miles = kms / 1.6;
            println!("{} km = {} miles", kms, miles);
        }
        else if option == 2 {
            print!("Enter km: ");
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut input).expect("Failed to read line!");
            let miles = input.trim().parse::<f64>().expect("Invalid input!");
            let kms = miles * 1.6;
            println!("{} miles = {} km", miles, kms);
        }
        else {
            println!("Invalid option!");
        }
    }

    println!("Thanks for using Unit Converter!");
}
