use pulldown_cmark::{Options, Parser, html};
use std::env;
use std::fs;
use std::io::Write;

fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run <markdown_file> <html_file>");
        return;
    }

    let markdown_file = &args[1];
    let html_file = &args[2];

    // Read markdown content
    let markdown_content = fs::read_to_string(markdown_file).expect("Invalid markdown file!");

    // Convert markdown to html
    let options = Options::empty();
    let parser = Parser::new_ext(&markdown_content, options);

    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    // Open HTML File
    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(html_file)
        .expect("Failed to open html file!");

    // Write html File
    file.write_all(html_content.as_bytes())
        .expect("Failed to write html to file!");
}
