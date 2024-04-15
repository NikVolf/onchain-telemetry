#![no_std]
#![allow(async_fn_in_trait)] // no multi-threading

mod test_client;

use gstd::{prelude::*, ActorId};
use sails_macros::gservice;

pub struct TelemetryService<E> {
    executor: E,
}

pub trait Executor {
    async fn watch(&mut self, actor: ActorId);
    async fn stop_watch(&mut self, actor: ActorId);
    async fn list_watching(&self) -> Vec<ActorId>;
    async fn execute_all(&self);
}

impl<E> TelemetryService<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

#[gservice]
impl<E: Executor> TelemetryService<E> {
    pub async fn watch(&mut self, actor: ActorId) -> () {
        self.executor.watch(actor).await
    }

    pub async fn stop_watch(&mut self, actor: ActorId) -> () {
        self.executor.stop_watch(actor).await
    }
}
