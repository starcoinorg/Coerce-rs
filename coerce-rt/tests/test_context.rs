use coerce_rt::actor::context::ActorContext;
use util::*;

pub mod util;

#[macro_use]
extern crate serde;
extern crate serde_json;

extern crate chrono;

#[macro_use]
extern crate async_trait;

#[tokio::test]
pub async fn test_context_get_actor() {
    let mut ctx = ActorContext::new();

    let mut actor_ref = ctx.new_actor(TestActor::new()).await.unwrap();

    let _ = actor_ref
        .exec(|mut actor| {
            actor.counter = 1337;
        })
        .await;

    let mut actor = ctx.get_actor::<TestActor>(actor_ref.id).await.unwrap();

    let counter = actor.exec(|actor| actor.counter).await;

    assert_eq!(counter, Ok(1337));
}
