mod webhook;
mod github;
mod command;
mod app;

use axum::{routing::post, Router};
use clap::Parser;
use command::Args;
use app::AppState;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    let port = args.port;
    
    info!("Starting smoked-tofu server...");
    info!("Port: {}", port);
    info!("Command: {} {}", args.command, args.args.join(" "));
    
    let app_state = AppState::new(args);
    
    let app = Router::new()
        .route("/webhook", post(webhook::handle_webhook))
        .with_state(app_state);

    let bind_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    info!("GitHub webhook server running on http://{}", bind_addr);
    
    axum::serve(listener, app).await.unwrap();
}