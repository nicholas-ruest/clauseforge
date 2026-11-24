use ruvector_core::advanced_features::filtered_search::FilterExpression;
use serde_json::json;
use std::{collections::HashMap, env};

fn required(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn main() {
    let supplier = required("CLAUSEFORGE_SUPPLIER");
    let amendment = required("CLAUSEFORGE_AMENDMENT");
    let controls: u8 = required("CLAUSEFORGE_AFFECTED_CONTROLS").parse().unwrap();
    let cure_days: u32 = required("CLAUSEFORGE_CURE_DAYS").parse().unwrap();
    let candidates = [
        (supplier.clone(), amendment.clone(), controls, cure_days),
        ("unrelated supplier".into(), "legacy notice".into(), 0, 365),
    ];
    let filter = FilterExpression::And(vec![
        FilterExpression::Eq("supplier".into(), json!(supplier)),
        FilterExpression::Eq("amendment".into(), json!(amendment)),
        FilterExpression::Range("controls".into(), json!(controls.saturating_sub(1)), json!(controls.saturating_add(1))),
    ]);
    let matched = candidates.iter().filter(|(s, a, c, d)| {
        let metadata = HashMap::from([
            ("supplier".into(), json!(s)), ("amendment".into(), json!(a)),
            ("controls".into(), json!(c)), ("cure_days".into(), json!(d)),
        ]);
        filter.evaluate(&metadata)
    }).count();
    assert_eq!(matched, 1, "request-specific metadata filter must select one neighbor");
    println!("similarity_set:{matched}:{controls}:{cure_days}");
}
