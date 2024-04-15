use gear_test_runtime::{ProgressSignal, TestInfo, TestUpdate};
use gstd::{exec, msg, prelude::*, ActorId, CodeId};

async fn create_this(code_hash: &CodeId) -> ActorId {
    let (_, actor_id) =
        gstd::prog::ProgramGenerator::create_program_bytes(code_hash.clone(), vec![], 0)
            .expect("Failed to create this/self");

    actor_id
}

#[gear_test_codegen::test]
async fn general(context: &gear_test_runtime::SessionData) {
    let this = create_this(&context.testee()).await;

    msg::send(
        this,
        ProgressSignal {
            test_info: TestInfo {
                index: 5,
                name: "Data".to_string(),
            },
            update: TestUpdate::Start,
        },
        0,
    )
    .expect("failed to send update message");

    let tester_id = exec::program_id();

    let reply: Vec<ProgressSignal> = msg::send_for_reply_as(this, tester_id, 0, 0)
        .expect("failed to send final message")
        .await
        .expect("failed to decode final message");

    gstd::debug!("Finally got a reply");

    assert_eq!(reply.len(), 1)
}
