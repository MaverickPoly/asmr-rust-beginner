use std::io;
use std::fs;
use std::io::{Write};

struct Todo {
    title: String,
    completed: bool,
}

const FILENAME: &str = "todos.csv";

// Fetch Todos function
fn fetch_todos() -> Vec<Todo> {
    let file_content = fs::read_to_string(FILENAME).expect("Failed to read file!");
    let mut todos: Vec<Todo> = Vec::new();

    for line in file_content.lines() {
        let parts: Vec<&str> = line.split(";").collect();
        let (title, completed): (&str, bool) = (parts[0], parts[1].parse().unwrap());

        let todo = Todo {title: String::from(title), completed};
        todos.push(todo);
    }

    todos
}

fn save_todos(todos: &mut Vec<Todo>) {
    let mut text = String::from("");

    for todo in todos {
        text += format!("{};{}\n", todo.title, todo.completed).as_str();
    }

    fs::write(FILENAME, text).expect("Failed to write file!");
}

fn add_todo(todos: &mut Vec<Todo>) {
    // User Input
    print!("Enter todo title: ");
    io::stdout().flush().unwrap();

    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line!");

    if title.trim() == "" {
        println!("Invalid title!");
        return;
    }

    let todo = Todo {title: title.trim().to_string(), completed: false};

    todos.push(todo);
}

fn toggle_complete(todos: &mut Vec<Todo>) {
    print!("Enter todo id: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    let mut id = input.trim().parse::<u32>().unwrap();

    if id == 0 || id - 1 >= todos.len() as u32 {
        println!("Invalid id!");
        return;
    }
    id -= 1;

    let todo = &mut todos[id as usize];

    todo.completed = !todo.completed;
}

fn delete_todo(todos: &mut Vec<Todo>) {
    print!("Enter todo id: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    let mut id = input.trim().parse::<u32>().unwrap();

    if id == 0 || id - 1 >= todos.len() as u32 {
        println!("Invalid id!");
        return
    }
    id -= 1;

    todos.remove(id as usize);
}

fn display_todos(todos: &mut Vec<Todo>) {
    println!("\n== Todos:");
    if todos.len() == 0 {
        println!("Nothing to display...");
    }

    for (index, todo) in todos.iter().enumerate() {
        println!("{}. {} - {}", index + 1, todo.title, todo.completed);
    }
}



fn main() {
    fs::OpenOptions::new().write(true).create(true).open(FILENAME).expect("Failed to create file!");

    loop {
        println!();
        println!("1.Display todos");
        println!("2.Add todo");
        println!("3.Toggle Completed");
        println!("4.Delete todo");
        println!("5.Quit");
        print!("Enter command: ");

        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line!");
        let command: u32 = command.trim().parse().expect("Invalid command!");

        let mut todos = fetch_todos();

        match command {
            1 => {
                display_todos(&mut todos);
            },
            2 => {
                add_todo(&mut todos);
                save_todos(&mut todos);
            },
            3 => {
                toggle_complete(&mut todos);
                save_todos(&mut todos);
            },
            4 => {
                delete_todo(&mut todos);
                save_todos(&mut todos);
            },
            5 => {
                println!("Quitting...");
                break;
            },
            _ => {
                println!("Invalid command!");
            },
        }
    }
}
