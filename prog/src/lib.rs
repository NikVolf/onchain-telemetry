#![no_std]

use gstd::{debug, prelude::*};
use sails_macros::gservice;

struct MyService;

#[gservice]
impl MyService {
    pub const fn new() -> Self {
        Self
    }

    pub async fn ping(&mut self) -> bool {
        debug!("Ping called");
        true
    }
}


