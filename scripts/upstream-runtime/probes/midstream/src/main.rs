use midstream::{
    AggregateFunction, HyprService, HyprServiceImpl, HyprSettings, MetricRecord, TimeWindow,
};
use std::env;

fn required(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let supplier = required("CLAUSEFORGE_SUPPLIER");
    let amendment = required("CLAUSEFORGE_AMENDMENT");
    let controls: f64 = required("CLAUSEFORGE_AFFECTED_CONTROLS").parse().unwrap();
    let cure_days = required("CLAUSEFORGE_CURE_DAYS");
    let settings = HyprSettings::default();
    let service = HyprServiceImpl::new(&settings).await.unwrap();
    service
        .ingest_metric(MetricRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            name: "affected_controls".into(),
            value: controls,
            labels: vec![
                ("supplier".into(), supplier),
                ("amendment".into(), amendment),
                ("cure_days".into(), cure_days.clone()),
            ],
        })
        .await
        .unwrap();
    let sum = service
        .query_aggregate(TimeWindow::Days(365), AggregateFunction::Sum)
        .await
        .unwrap();
    assert_eq!(sum, controls);
    println!("impact_metric:{sum:.0}:{cure_days}");
}
