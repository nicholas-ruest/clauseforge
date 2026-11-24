use ruv_fann::{ActivationFunction, NetworkBuilder};
use std::env;

fn required(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn main() {
    let supplier = required("CLAUSEFORGE_SUPPLIER");
    let amendment = required("CLAUSEFORGE_AMENDMENT");
    let controls: f32 = required("CLAUSEFORGE_AFFECTED_CONTROLS").parse().unwrap();
    let cure_days: f32 = required("CLAUSEFORGE_CURE_DAYS").parse().unwrap();
    let mut network = NetworkBuilder::<f32>::new()
        .input_layer(4).hidden_layer_with_activation(4, ActivationFunction::Sigmoid, 1.0)
        .output_layer_with_activation(1, ActivationFunction::Linear, 1.0).build();
    let count = network.get_weights().len();
    let weights: Vec<f32> = (0..count).map(|i| (i as f32 % 7.0 - 3.0) / 20.0).collect();
    network.set_weights(&weights).unwrap();
    let inputs = [controls / 100.0, cure_days / 365.0, supplier.len() as f32 / 100.0, amendment.len() as f32 / 200.0];
    let score = network.run(&inputs).unwrap()[0];
    assert!(score.is_finite());
    println!("calibrated_interval:{score:.6}:{controls}:{cure_days}");
}
