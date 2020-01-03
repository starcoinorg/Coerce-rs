use coerce_rt::actor::context::{ActorContext, ActorStatus};
use coerce_rt::actor::ActorRefError;
use util::*;

pub mod util;

#[macro_use]
extern crate serde;
extern crate serde_json;

extern crate chrono;

#[macro_use]
extern crate async_trait;

#[tokio::test]
pub async fn test_actor_lifecycle_started() {
    let mut actor_ref = ActorContext::new()
        .new_actor(TestActor::new())
        .await
        .unwrap();

    let status = actor_ref.status().await;

    actor_ref.stop().await;
    assert_eq!(status, Ok(ActorStatus::Started))
}

#[tokio::test]
pub async fn test_actor_lifecycle_stopping() {
    let mut actor_ref = ActorContext::new()
        .new_actor(TestActor::new())
        .await
        .unwrap();

    let status = actor_ref.status().await;
    let stopping = actor_ref.stop().await;
    let msg_send = actor_ref.status().await;
    let stopping_again = actor_ref.stop().await;

    assert_eq!(status, Ok(ActorStatus::Started));
    assert_eq!(stopping, Ok(ActorStatus::Stopping));
    assert_eq!(msg_send, Err(ActorRefError::ActorUnavailable));
    assert_eq!(stopping_again, Err(ActorRefError::ActorUnavailable));
}
