use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
#[derive(Serialize)]
struct Course {
    language: String,
    name: String,
    category: String,
}

const FILENAME: &str = "courses.json";

// Manual implementation
fn read_courses() -> Vec<Course> {
    let file = fs::File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let mut courses: Vec<Course> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.is_empty() || line == "[" || line == "]" {
            continue;
        }

        let line = line.trim_end_matches(',');

        let Some(inner) = line.strip_prefix('{').and_then(|s| s.strip_suffix('}')) else {
            continue;
        };

        let mut language = String::new();
        let mut name = String::new();
        let mut category = String::new();

        for field in inner.split(',') {
            let mut kv = field.splitn(2, ':');
            let key = kv.next().map(|s| s.trim().trim_matches('"')).unwrap();
            let val = kv.next().map(|s| s.trim().trim_matches('"')).unwrap();

            match key {
                "language" => language = val.to_string(),
                "name" => name = val.to_string(),
                "category" => category = val.to_string(),
                _ => {}
            }
        }
        courses.push(Course {
            language,
            name,
            category,
        })
    }

    courses
}

fn write_courses(courses: &Vec<Course>) {
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(FILENAME)
        .unwrap();

    let mut writer = BufWriter::new(file);
    writer.write_all("[\n".as_bytes()).unwrap();
    for (i, course) in courses.iter().enumerate() {
        writer
            .write(
                format!(
                    "{{\"language\": \"{}\", \"name\": \"{}\", \"category\": \"{}\"}}",
                    course.language, course.name, course.category
                )
                .as_bytes(),
            )
            .expect("Failed to write to file");
        if i < courses.len() - 1 {
            writer
                .write(",\n".as_bytes())
                .expect("Failed to write to file");
        }
    }
    writer.write_all("\n]".as_bytes()).unwrap();
}


// serde-json
fn write_json(courses: &Vec<Course>) {
    let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(FILENAME).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, courses).unwrap();
}

fn read_json() -> Vec<Course> {
    let file = fs::File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}


fn main() {
    let mut courses: Vec<Course> = vec![];
    courses.push(Course {
        language: String::from("Python"),
        name: String::from("Django"),
        category: String::from("Backend"),
    });
    courses.push(Course {
        language: String::from("JavaScript"),
        name: String::from("React"),
        category: String::from("Frontend"),
    });
    courses.push(Course {
        language: String::from("Java"),
        name: String::from("Swing"),
        category: String::from("Desktop"),
    });
    courses.push(Course {
        language: String::from("Dart"),
        name: String::from("Flutter"),
        category: String::from("Mobile"),
    });
    courses.push(Course {
        language: String::from("C"),
        name: String::from("Raylib"),
        category: String::from("Game"),
    });

    // write_courses(&courses);
    write_json(&courses);

    println!("Courses:\n");
    // let courses = read_courses();
    let courses = read_json();
    for course in courses {
        println!("{}, {}, {}", course.language, course.name, course.category);
    }
}
