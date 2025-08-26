use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Clone)]
struct User {
    username: String,
    email: String,
    country: String,
    age: u32,
}

const FILENAME: &str = "users.csv";

// Read all Data
fn read_users() -> Vec<User> {
    let file = fs::File::open(FILENAME).expect("Failed to open file!");
    let reader = io::BufReader::new(file);
    let mut users: Vec<User> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line!");

        let splitted: Vec<&str> = line.split(";").collect();
        let (username, email, country, age): (String, String, String, u32) = (
            String::from(splitted[0]),
            String::from(splitted[1]),
            String::from(splitted[2]),
            splitted[3].parse().unwrap(),
        );
        let user = User {
            username,
            email,
            country,
            age,
        };
        users.push(user);
    }
    users
}

// Add User
fn add_user(user: &User) {
    let mut users = read_users();
    users.push(user.clone());
    write_users(&users);
}

// Write All Data (replace)
fn write_users(users: &Vec<User>) {
    let mut users_content = String::new();
    for user in users {
        users_content += &format!(
            "{};{};{};{}\n",
            user.username, user.email, user.country, user.age
        );
    }
    fs::write(FILENAME, users_content).expect("Failed to write file!");
}

// Clear File
fn clear_users() {
    fs::write(FILENAME, "").expect("Failed to write file!");
}

// Display users to console
fn display_users() {
    println!("Users:");
    for user in read_users() {
        println!(
            "{}, {}, {}, {}",
            user.username, user.email, user.country, user.age
        );
    }
}

fn main() {
    // Create file
    fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(FILENAME)
        .unwrap();

    add_user(&User {
        username: String::from("Ahmed"),
        email: String::from("ahmed@gmail.com"),
        country: String::from("Egypt"),
        age: 19,
    });
    add_user(&User {
        username: String::from("Wang Ming"),
        email: String::from("wang@ming.com"),
        country: String::from("China"),
        age: 24,
    });
    add_user(&User {
        username: String::from("Sarah"),
        email: String::from("sarah@gmail.com"),
        country: String::from("England"),
        age: 22,
    });
    display_users();

    clear_users();
    display_users();

    add_user(&User {
        username: String::from("Carlos"),
        email: String::from("carlos@example.com"),
        country: String::from("Brazil"),
        age: 28,
    });

    add_user(&User {
        username: String::from("Sergey"),
        email: String::from("sergey@yandex.ru"),
        country: String::from("Russia"),
        age: 26,
    });

    add_user(&User {
        username: String::from("Hiroshi"),
        email: String::from("hiroshi@jp.com"),
        country: String::from("Japan"),
        age: 31,
    });
    display_users();
}
