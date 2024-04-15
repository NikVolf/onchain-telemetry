#![no_std]

use gstd::{collections::BTreeSet, prelude::*, ActorId};
use telemetry_io::Executor;

struct TelemetryExecutor {
    watching: BTreeSet<ActorId>,
}

impl Executor for TelemetryExecutor {
    async fn watch(&mut self, actor: ActorId) {
        self.watching.insert(actor);
    }

    async fn stop_watch(&mut self, actor: ActorId) {
        self.watching.remove(&actor);
    }

    async fn list_watching(&self) -> Vec<ActorId> {
        self.watching.iter().cloned().collect()
    }

    async fn execute_all(&self) {
        unimplemented!()
    }
}
