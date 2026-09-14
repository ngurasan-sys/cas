use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubscriptionTier {
    P0ActivePositions,
    P1StrategyCandidates,
    P2Scanner,
    P3Background,
}

pub struct SubscriptionManager {
    // Maps an instrument ID to its highest active subscription tier
    subscriptions: HashMap<String, SubscriptionTier>,
    // Tracks the exact sources requesting the subscription to handle deduplication correctly
    requests: HashMap<String, HashSet<String>>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
            requests: HashMap::new(),
        }
    }

    /// Add a subscription requirement for an instrument from a specific source (e.g. strategy ID).
    /// Returns true if this results in a NEW subscription or an UPGRADE to the priority tier.
    pub fn subscribe(&mut self, instrument_id: &str, source: &str, tier: SubscriptionTier) -> bool {
        let entry = self
            .requests
            .entry(instrument_id.to_string())
            .or_insert_with(HashSet::new);
        entry.insert(source.to_string());

        let current_tier = self.subscriptions.get(instrument_id);

        let needs_update = match current_tier {
            None => true,
            Some(existing) => tier < *existing, // Lower enum value means higher priority (P0 < P1)
        };

        if needs_update {
            self.subscriptions.insert(instrument_id.to_string(), tier);
            true
        } else {
            false
        }
    }

    /// Remove a subscription requirement.
    /// Returns true if the instrument has NO remaining requests and should be unsubscribed at the gateway.
    pub fn unsubscribe(&mut self, instrument_id: &str, source: &str) -> bool {
        if let Some(sources) = self.requests.get_mut(instrument_id) {
            sources.remove(source);
            if sources.is_empty() {
                self.requests.remove(instrument_id);
                self.subscriptions.remove(instrument_id);
                return true;
            }
        }
        false
    }

    pub fn get_active_subscriptions(&self) -> Vec<String> {
        self.subscriptions.keys().cloned().collect()
    }
}
