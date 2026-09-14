pub mod gateway;
pub mod subscription;
pub mod synthetic;

// Export main structs for external usage
pub use gateway::{MarketDataGateway, MarketDataProvider};
pub use subscription::{SubscriptionManager, SubscriptionTier};
pub use synthetic::SyntheticProvider;
