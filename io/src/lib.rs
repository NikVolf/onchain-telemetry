#![no_std]
#![allow(async_fn_in_trait)] // no multi-threading

use gstd::{ActorId, prelude::*};
use sails_macros::gservice;

pub struct TelemetryService<E> {
    executor: E,
}

pub trait Executor {
    async fn watch(&mut self, actor: ActorId);
    async fn stop_watch(&mut self, actor: ActorId);
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
