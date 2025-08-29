📦 Dependencies in `Cargo.toml`

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "cookies", "multipart"] }
serde = { version = "1.0", features = ["derive"] }
```



1. GET request

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://httpbin.org/get")
        .await?
        .text()
        .await?;
    println!("GET Response: {body}");
    Ok(())
}
```




2. GET with Query Parameters

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .get("https://httpbin.org/get")
        .query(&[("search", "rust"), ("page", "2")])
        .send()
        .await?
        .text()
        .await?;

    println!("GET with query: {res}");
    Ok(())
}
```





3. GET with Headers

```rust
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("MyRustClient/1.0"));

    let client = reqwest::Client::new();
    let res = client
        .get("https://httpbin.org/headers")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    println!("GET with headers: {res}");
    Ok(())
}
```





4. POST with JSON

```rust
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    username: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let user = User {
        username: "Ahmed".into(),
        email: "ahmed@example.com".into(),
    };

    let res = client
        .post("https://httpbin.org/post")
        .json(&user)
        .send()
        .await?
        .text()
        .await?;

    println!("POST JSON: {res}");
    Ok(())
}
```





5. POST with Form Data

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let params = [("username", "Ahmed"), ("password", "12345")];

    let res = client
        .post("https://httpbin.org/post")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    println!("POST form: {res}");
    Ok(())
}
```





6. POST with File Upload (Multipart)

```rust
use reqwest::Client;
use reqwest::multipart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let form = multipart::Form::new()
        .text("username", "Ahmed")
        .file("file", "example.txt")?; // make sure file exists

    let res = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    println!("POST multipart: {res}");
    Ok(())
}
```





7. PUT request

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .put("https://httpbin.org/put")
        .body("Updating data...")
        .send()
        .await?
        .text()
        .await?;

    println!("PUT: {res}");
    Ok(())
}
```





8. DELETE request

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .delete("https://httpbin.org/delete")
        .send()
        .await?
        .text()
        .await?;

    println!("DELETE: {res}");
    Ok(())
}
```





9. PATCH request

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .patch("https://httpbin.org/patch")
        .body("patch content")
        .send()
        .await?
        .text()
        .await?;

    println!("PATCH: {res}");
    Ok(())
}
```





🔥 10. Cookies

Enable cookie store with `cookies` feature:

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .cookie_store(true)
        .build()?;

    // First request sets a cookie
    client.get("https://httpbin.org/cookies/set?lang=rust")
        .send()
        .await?;

    // Second request shows stored cookies
    let res = client.get("https://httpbin.org/cookies")
        .send()
        .await?
        .text()
        .await?;

    println!("Cookies: {res}");
    Ok(())
}
```
