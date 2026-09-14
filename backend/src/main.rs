use backend::foundation::logging::init_logging;
use backend::foundation::state::AppState;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    init_logging();

    let event_bus = Arc::new(backend::event_bus::EventBus::new());

    let subscription_manager = Arc::new(std::sync::RwLock::new(
        backend::market_data::SubscriptionManager::new(),
    ));
    let gateway = Arc::new(std::sync::Mutex::new(
        backend::market_data::MarketDataGateway::new(event_bus.clone()),
    ));

    let mut generator = backend::market_data::SyntheticProvider::new(gateway.clone());
    generator.start();

    let state = Arc::new(AppState {
        event_bus,
        subscription_manager,
    });

    let app = backend::api::create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
