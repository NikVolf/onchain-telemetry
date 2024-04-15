use gear_test_runtime::ControlSignal;
use gstd::{prelude::*, ActorId, CodeId};

pub struct TestActor {
    actor_id: ActorId,
    code_hash: CodeId,
    control_bus: ActorId,
    gas: u64,
}

impl TestActor {
    pub fn new(actor_id: ActorId, code_hash: CodeId, control_bus: ActorId, gas: u64) -> Self {
        Self {
            actor_id,
            code_hash,
            control_bus,
            gas,
        }
    }

    pub async fn run(&self) {
        gstd::msg::send_with_gas_for_reply(
            self.actor_id,
            ControlSignal::Test {
                code_hash: self.code_hash.clone(),
                control_bus: self.control_bus.clone(),
            },
            self.gas,
            0,
            0,
        )
        .expect("Failed to send message")
        .await
        .expect("failed to run all actors");
    }
}
