use serde::Deserialize;

#[derive(Deserialize)]
struct Post {
    id: u32,
    userId: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() {
    let response = reqwest::get("https://json-placeholder-olive.vercel.app/posts")
        .await
        .unwrap();

    if !response.status().is_success() {
        eprintln!("Request failed: {}", response.status());
        return;
    }

    let body = response.text().await.unwrap();

    println!("{}", body);
    let posts: Vec<Post> = serde_json::from_str(body.as_str()).unwrap();

    for post in posts {
        println!("{}, {}, {}, {}", post.id, post.userId, post.title, post.body)
    }
}
