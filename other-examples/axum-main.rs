use axum::{
    Form, Router,
    response::Html,
    routing::{get, post},
};
use reqwest;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/validate", post(validate))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct UserInput {
    name: String,
}

async fn validate(Form(input): Form<UserInput>) -> Html<String> {
    if input.name.is_empty() {
        Html("<p style=\"color: red;\">Name cannot be empty!</p>".to_string())
    } else {
        Html(format!("<p>Hello, {}!</p>", input.name))
    }
}
