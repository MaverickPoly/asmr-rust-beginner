# 🌀 What is async in Rust?

* In Python/JS: `async fn` means “this runs concurrently when awaited”.
* In Rust: `async fn` **returns a `Future`**.
  A **Future** is like a "promise of a value later".
  The executor (Tokio, async-std) polls futures until they finish.

---

# 1️⃣ Writing your first async function

```rust
async fn hello_world() {
    println!("Hello async world!");
}
```

👉 But calling `hello_world();` does **nothing**, because it returns a **Future**.

We need an executor (like a runtime) to actually **drive** the future.

---

# 2️⃣ Using `tokio` runtime (most popular)

In `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Example:

```rust
#[tokio::main] // creates runtime
async fn main() {
    hello_world().await;
}

async fn hello_world() {
    println!("Hello async world!");
}
```

---

# 3️⃣ Running async tasks concurrently

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let task1 = task_one();
    let task2 = task_two();

    // run concurrently
    tokio::join!(task1, task2);
}

async fn task_one() {
    sleep(Duration::from_secs(1)).await;
    println!("Task 1 done!");
}

async fn task_two() {
    sleep(Duration::from_secs(2)).await;
    println!("Task 2 done!");
}
```

👉 `tokio::join!` waits for both futures **at the same time**.

---

# 4️⃣ Spawning background tasks

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Background task finished!");
    });

    println!("Main task running...");
    sleep(Duration::from_secs(2)).await;
}
```

👉 `tokio::spawn` starts tasks that run **in parallel on the runtime**.

---

# 5️⃣ Doing async I/O (realistic example)

With `reqwest` (async HTTP client):

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = "0.11"
```

```rust
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/get").await?;
    let body = resp.text().await?;
    println!("Response: {}", body);
    Ok(())
}
```

---

# 6️⃣ Async + multiple requests

```rust
#[tokio::main]
async fn main() {
    let urls = vec![
        "https://httpbin.org/get",
        "https://httpbin.org/ip",
        "https://httpbin.org/uuid",
    ];

    let mut handles = vec![];

    for url in urls {
        handles.push(tokio::spawn(fetch(url)));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn fetch(url: &str) {
    let resp = reqwest::get(url).await.unwrap();
    println!("{} -> {}", url, resp.status());
}
```

---

# 🔑 Why Rust async feels hard

1. **Futures are lazy** → you must `.await` or they don’t run.
2. **Borrow checker + async** → values must live across suspension points.
   Example mistake:

   ```rust
   let mut s = String::new();
   let fut = async {
       s.push_str("hi");  // ❌ borrow across await issues
       sleep(Duration::from_secs(1)).await;
       println!("{}", s);
   };
   ```

   Fix → move ownership inside:

   ```rust
   let s = String::from("hi");
   let fut = async move {
       sleep(Duration::from_secs(1)).await;
       println!("{}", s);
   };
   ```
3. **No built-in runtime** → you need Tokio/async-std.

---

# 7️⃣ Summary

✅ Basics: `async fn`, `.await`, runtime
✅ Concurrency: `tokio::join!`, `tokio::spawn`
✅ Async I/O: `reqwest`, `tokio::fs`, etc.
✅ Gotcha: borrowing across `.await`
