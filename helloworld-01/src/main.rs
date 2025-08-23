use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for arg in args {
            if arg.starts_with("name=") {
                let name = arg.split("=").nth(1).unwrap_or("World");
                println!("Hello, {}!", name);
            }
        }
    } else {
        println!("Hello, world!");
    }
}
