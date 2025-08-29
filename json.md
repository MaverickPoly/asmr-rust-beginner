```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```






1. ✅ Parse JSON string into a `serde_json::Value`

```rust
use serde_json::Value;

fn main() {
    let data = r#"
        {
            "name": "Alice",
            "age": 25,
            "is_student": false,
            "skills": ["Rust", "Python", "SQL"]
        }
    "#;

    let v: Value = serde_json::from_str(data).unwrap();

    println!("Name: {}", v["name"]);
    println!("First skill: {}", v["skills"][0]);
}
```

👉 `Value` is a flexible type (like Python’s `dict` or JavaScript’s object).





2. ✅ Access fields safely

```rust
if let Some(age) = v["age"].as_i64() {
    println!("Age is {age}");
}
```

Methods:

* `.as_str() -> Option<&str>`
* `.as_i64() -> Option<i64>`
* `.as_bool() -> Option<bool>`
* `.as_array() -> Option<&Vec<Value>>`
* `.as_object() -> Option<&Map<String, Value>>`





3. ✅ Deserialize JSON → Struct

This is **most common** when working with APIs:

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
    is_student: bool,
    skills: Vec<String>,
}

fn main() {
    let data = r#"
        {"name":"Alice","age":25,"is_student":false,"skills":["Rust","Python"]}
    "#;

    let user: User = serde_json::from_str(data).unwrap();
    println!("{:?}", user);
}
```





4. ✅ Serialize Struct → JSON

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

fn main() {
    let product = Product {
        id: 1,
        name: "Laptop".to_string(),
        price: 1299.99,
    };

    let json_str = serde_json::to_string(&product).unwrap();
    println!("Compact JSON: {}", json_str);

    let pretty = serde_json::to_string_pretty(&product).unwrap();
    println!("Pretty JSON:\n{}", pretty);
}
```





5. ✅ Modify JSON dynamically

```rust
use serde_json::{json, Value};

fn main() {
    let mut user = json!({
        "name": "Bob",
        "age": 30
    });

    user["city"] = Value::String("Tashkent".to_string());
    user["skills"] = json!(["Go", "Rust"]);

    println!("{}", user);
}
```

👉 The `json!` macro makes JSON literals super easy.





6. ✅ Query arrays & iterate

```rust
if let Some(skills) = user["skills"].as_array() {
    for skill in skills {
        println!("Skill: {}", skill.as_str().unwrap());
    }
}
```





7. ✅ Read/Write JSON files

```rust
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
    port: u16,
}

fn main() {
    // Write
    let config = Config { host: "localhost".into(), port: 8080 };
    fs::write("config.json", serde_json::to_string_pretty(&config).unwrap()).unwrap();

    // Read
    let content = fs::read_to_string("config.json").unwrap();
    let read_config: Config = serde_json::from_str(&content).unwrap();
    println!("{:?}", read_config);
}
```





8. ✅ Handle unknown/dynamic JSON

When API returns fields you don’t control:

```rust
fn main() {
    let data = r#"{"id": 1, "extra": {"nested": true}}"#;
    let v: Value = serde_json::from_str(data).unwrap();

    if let Some(extra) = v["extra"].as_object() {
        println!("Has extra fields: {:?}", extra);
    }
}
```