use crate::actor::message::{Handler, Message};
use crate::actor::{Actor, ActorId, ActorRefError};
use crate::remote::context::RemoteActorContext;
use serde::export::PhantomData;

pub mod actor;
pub mod codec;
pub mod context;
pub mod handler;

pub struct RemoteActorRef<A: Actor>
where
    A: 'static + Sync + Send,
{
    id: ActorId,
    context: RemoteActorContext,
    _a: PhantomData<A>,
}

impl<A: Actor> RemoteActorRef<A>
where
    A: 'static + Sync + Send,
{
    pub fn new(id: ActorId, context: RemoteActorContext) -> RemoteActorRef<A> {
        RemoteActorRef {
            id,
            context,
            _a: PhantomData,
        }
    }

    pub async fn send<Msg: Message>(&mut self, _msg: Msg) -> Result<Msg::Result, ActorRefError>
    where
        Msg: 'static + Send + Sync,
        A: Handler<Msg>,
        Msg::Result: Send + Sync,
    {
        Err(ActorRefError::ActorUnavailable)
    }
}
