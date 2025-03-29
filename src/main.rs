use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use axum::{response::Html, routing::get, Router, extract::{Path,Query}, http::HeaderMap, extract::State};

struct MyConfig {
    counter: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(MyConfig {counter: AtomicUsize::new(0)});

    let app = Router::new()
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract))
        .with_state(counter);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();

    println!("Listening on http://127.0.0.1:3001");
    // Start serving the application
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(config): State<Arc<MyConfig>>) -> Html<String> {
    Html(format!("<h1>{}</h1>", config.counter.fetch_add(1, Ordering::Relaxed)))
}

async fn path_extract( Path(id): Path<u32>) -> Html<String> {
    Html(format!("<h1>Book ID: {}</h1>", id))
}

async fn query_extract (Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(format!("{params:#?}"))
}

async fn header_extract(headers: HeaderMap) -> Html<String> {
    Html(format!("{headers:#?}"))
}
