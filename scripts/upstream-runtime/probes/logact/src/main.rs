use agent_bus_proto_rust::agent_bus::DeciderPolicy;
use futures::executor::block_on;
use logact_commit_service_api::{PolicyProvider, PolicyState};
use logact_commit_service_engine::InMemoryStorage;
use logact_commit_service_policy_register::{
    PolicyRegister, StoragePolicyRegister, SynchronousRegisterProvider,
};
use std::{env, rc::Rc};

fn required(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn main() {
    let supplier = required("CLAUSEFORGE_SUPPLIER");
    let amendment = required("CLAUSEFORGE_AMENDMENT");
    let controls: u8 = required("CLAUSEFORGE_AFFECTED_CONTROLS").parse().unwrap();
    let cure_days = required("CLAUSEFORGE_CURE_DAYS");
    let key = format!("{}:{}:{}", supplier, amendment.len(), cure_days);
    let register = StoragePolicyRegister::new(Rc::new(InMemoryStorage::new()), &key);
    let provider = SynchronousRegisterProvider::new(register.clone(), |_| Ok(()));
    let decider = if controls > 25 {
        DeciderPolicy::FirstBooleanWins
    } else {
        DeciderPolicy::OnByDefault
    };
    let state = PolicyState {
        decider_policy: Some(decider as i32),
        ..Default::default()
    };
    let generation = block_on(register.set_policy(&state, None)).unwrap();
    let policy = block_on(provider.read(&supplier)).unwrap();
    println!("logact_policy:{generation}:{}:{}", policy.version, controls);
}
