use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use shared::TextPayload;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Default)]
struct AppState {
    text: Arc<Mutex<String>>,
}

// Handler for: GET /api/text
async fn get_text(State(state): State<AppState>) -> Json<TextPayload> {
    let current_text = state.text.lock().unwrap().clone();
    Json(TextPayload { text: current_text })
}

// Handler for: POST /api/text
async fn set_text(State(state): State<AppState>, Json(payload): Json<TextPayload>) -> StatusCode {
    let mut current_text = state.text.lock().unwrap();
    *current_text = payload.text;
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {
        text: Arc::new(Mutex::new("Hello from Server!".to_string())),
    };

    // CORS layer to allow local frontend dev server (e.g., trunk serve on :8080)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // API Routes under /api
    let api_routes = Router::new()
        .route("/text", get(get_text).post(set_text))
        .with_state(state)
        .layer(cors);

    // Find the static dist directory
    let dist_dir = if PathBuf::from("dist").exists() {
        PathBuf::from("dist")
    } else if PathBuf::from("crates/frontend/dist").exists() {
        PathBuf::from("crates/frontend/dist")
    } else {
        PathBuf::from("dist")
    };

    let static_service = ServeDir::new(&dist_dir);

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(static_service)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    println!("📁 Serving static files from: {}", dist_dir.display());
    axum::serve(listener, app).await.unwrap();
}
