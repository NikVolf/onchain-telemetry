#![no_std]
#![allow(static_mut_refs)]

use gear_test_runtime::ProgressSignal;
use gstd::{exec, msg, prelude::*, ActorId};

mod tests;

static mut HEAP: Vec<ProgressSignal> = vec![];
static mut OWNER: ActorId = ActorId::zero();

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let signal_bytes = msg::load_bytes().expect("Failed to load signal bytes");

    if signal_bytes.len() == 32 {
        if let Ok(valid_actor_id) = ActorId::decode(&mut &signal_bytes[0..32]) {
            if valid_actor_id == OWNER {
                msg::reply(core::mem::replace(&mut HEAP, vec![]), 0).expect("Failed to reply");

                gstd::debug!("Replied to the owner");

                exec::exit(valid_actor_id);

                return;
            }
        }
    }

    // progress signal to add to HEAP
    HEAP.push(
        ProgressSignal::decode(&mut &signal_bytes[..])
            .expect("Failed to decode incoming signal as ProgressSignal"),
    );

    gstd::debug!("pushed progress");
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    OWNER = msg::source();
}
