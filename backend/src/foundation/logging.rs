// Basic structured logging wrapper
// In a real application, this would configure `tracing-subscriber` with JSON formatting.

pub fn init_logging() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}
