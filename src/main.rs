mod webhook;
mod github;
mod command;
mod app;
mod middleware;

use axum::{routing::post, Router, middleware::from_fn_with_state};
use clap::Parser;
use command::Args;
use app::AppState;
use tracing::info;
use tracing_subscriber::{self, EnvFilter};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("smoked_tofu=debug,tower_http=debug"))
                .unwrap(),
        )
        .init();
    
    let args = Args::parse();
    let port = args.port;
    
    info!("Starting smoked-tofu server...");
    info!("Port: {}", port);
    info!("Command: {} {}", args.command, args.args.join(" "));
    
    let app_state = AppState::new(args);
    
    let app = Router::new()
        .route("/webhook", post(webhook::handle_webhook))
        .layer(from_fn_with_state(
            app_state.webhook_secret.clone(),
            middleware::verify_github_signature,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let bind_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    info!("GitHub webhook server running on http://{}", bind_addr);
    
    axum::serve(listener, app).await.unwrap();
}