use crate::actor::context::{ActorContext, ActorHandlerContext};
use crate::actor::lifecycle::actor_loop;
use crate::actor::message::{Handler, Message, MessageHandler};
use crate::actor::{Actor, ActorId, ActorRef, BoxedActorRef};
use std::collections::HashMap;
use std::marker::PhantomData;

pub mod timer;

pub struct ActorScheduler {
    actors: HashMap<ActorId, BoxedActorRef>,
}

impl ActorScheduler {
    pub fn new() -> ActorRef<ActorScheduler> {
        let (actor_ref, rx) = gen_actor_ref::<ActorScheduler>();
        let actor = ActorScheduler {
            actors: HashMap::new(),
        };
        let context = ActorContext::from(actor_ref.clone());
        tokio::spawn(actor_loop(actor_ref.id.clone(), context, actor, rx, None));
        actor_ref
    }
}

#[async_trait]
impl Actor for ActorScheduler {
    async fn started(&mut self, ctx: &mut ActorHandlerContext) {
        trace!("actor scheduler {} started", ctx.actor_id());
    }

    async fn stopped(&mut self, ctx: &mut ActorHandlerContext) {
        trace!("actor scheduler {} stopped", ctx.actor_id());
    }
}

pub struct RegisterActor<A: Actor>(
    pub ActorContext,
    pub A,
    pub tokio::sync::oneshot::Sender<bool>,
)
where
    A: 'static + Sync + Send;

impl<A: Actor> Message for RegisterActor<A>
where
    A: 'static + Sync + Send,
{
    type Result = ActorRef<A>;
}

pub struct GetActor<A: Actor>
where
    A: 'static + Sync + Send,
{
    id: ActorId,
    _a: PhantomData<A>,
}

impl<A: Actor> Message for GetActor<A>
where
    A: 'static + Sync + Send,
{
    type Result = Option<ActorRef<A>>;
}

impl<A: Actor> GetActor<A>
where
    A: 'static + Sync + Send,
{
    pub fn new(id: ActorId) -> GetActor<A> {
        GetActor {
            id,
            _a: PhantomData,
        }
    }
}

pub struct RemoveActor<A: Actor>
where
    A: 'static + Sync + Send,
{
    id: ActorId,
    _a: PhantomData<A>,
}

impl<A: Actor> Message for RemoveActor<A>
where
    A: 'static + Sync + Send,
{
    type Result = Option<ActorRef<A>>;
}

impl<A: Actor> RemoveActor<A>
where
    A: 'static + Sync + Send,
{
    pub fn new(id: ActorId) -> RemoveActor<A> {
        RemoveActor {
            id,
            _a: PhantomData,
        }
    }
}

#[async_trait]
impl<A: Actor> Handler<RegisterActor<A>> for ActorScheduler
where
    A: 'static + Sync + Send,
{
    async fn handle(
        &mut self,
        message: RegisterActor<A>,
        _ctx: &mut ActorHandlerContext,
    ) -> ActorRef<A> {
        let RegisterActor(context, actor, on_start) = message;
        let actor = start_actor(context, actor, Some(on_start));

        let _ = self
            .actors
            .insert(actor.id, BoxedActorRef::from(actor.clone()));

        actor
    }
}

#[async_trait]
impl<A: Actor> Handler<GetActor<A>> for ActorScheduler
where
    A: 'static + Sync + Send,
{
    async fn handle(
        &mut self,
        message: GetActor<A>,
        _ctx: &mut ActorHandlerContext,
    ) -> Option<ActorRef<A>> {
        match self.actors.get(&message.id) {
            Some(actor) => Some(ActorRef::<A>::from(actor.clone())),
            None => None,
        }
    }
}

#[async_trait]
impl<A: Actor> Handler<RemoveActor<A>> for ActorScheduler
where
    A: 'static + Sync + Send,
{
    async fn handle(
        &mut self,
        message: RemoveActor<A>,
        _ctx: &mut ActorHandlerContext,
    ) -> Option<ActorRef<A>> {
        match self.actors.remove(&message.id) {
            Some(actor) => Some(ActorRef::<A>::from(actor)),
            None => None,
        }
    }
}

fn start_actor<A: Actor>(
    actor_context: ActorContext,
    actor: A,
    on_start: Option<tokio::sync::oneshot::Sender<bool>>,
) -> ActorRef<A>
where
    A: 'static + Send + Sync,
{
    let (actor_ref, rx) = gen_actor_ref();
    tokio::spawn(actor_loop(
        actor_ref.id.clone(),
        actor_context,
        actor,
        rx,
        on_start,
    ));
    actor_ref
}

fn gen_actor_ref<A: Actor>() -> (ActorRef<A>, tokio::sync::mpsc::Receiver<MessageHandler<A>>)
where
    A: 'static + Send + Sync,
{
    let id = ActorId::new_v4();
    let (tx, rx) = tokio::sync::mpsc::channel(128);
    let actor_ref = ActorRef {
        id: id.clone(),
        sender: tx,
    };
    (actor_ref, rx)
}
