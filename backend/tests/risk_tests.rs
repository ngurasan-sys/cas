use backend::risk::OrderIntent;
use backend::risk::RiskEngine;

// Create a dummy implementation of RiskEngine for tests
struct MockRiskEngine;
impl backend::risk::RiskEngine for MockRiskEngine {
    fn validate_order(&self, intent: &OrderIntent) -> Result<(), String> {
        if intent.quantity > 1000 {
            Err("Quantity exceeds maximum limit".to_string())
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_risk_engine_validation() {
    let engine = MockRiskEngine;

    let valid_intent = OrderIntent {
        instrument_id: "NIFTY25100CE".to_string(),
        quantity: 50,
        order_type: "BUY".to_string(),
    };

    let invalid_intent = OrderIntent {
        instrument_id: "NIFTY25100CE".to_string(),
        quantity: 5000,
        order_type: "BUY".to_string(),
    };

    assert!(engine.validate_order(&valid_intent).is_ok());
    assert!(engine.validate_order(&invalid_intent).is_err());
}
