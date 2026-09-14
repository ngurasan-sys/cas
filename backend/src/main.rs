use backend::foundation::logging::init_logging;
use backend::foundation::state::AppState;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    init_logging();

    let event_bus = Arc::new(backend::event_bus::EventBus::new());

    let generator = backend::market_data::SyntheticDataGenerator::new(event_bus.clone());
    generator.start();

    let state = Arc::new(AppState {
        event_bus: event_bus,
    });

    let app = backend::api::create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
