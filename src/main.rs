use axum::{response::Html, routing::get, Router};
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::init;
use crate::utils::ModuleTrait;

mod auth;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(Level::INFO) // Set the max log level
        .init();

    let auth_module = auth::Auth::initialize();

    let app = Router::new().merge(
        auth_module.router
    );

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!dddd</h1>")
}