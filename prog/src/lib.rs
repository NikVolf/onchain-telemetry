#![no_std]

use telemetry_io::Executor;
use gstd::{ActorId, prelude::*, collections::BTreeSet};

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
}
