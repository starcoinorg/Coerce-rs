#[macro_use]
extern crate serde;
extern crate serde_json;

extern crate chrono;

#[macro_use]
extern crate async_trait;

use coerce_remote::cluster::node::RemoteNode;
use coerce_remote::cluster::workers::{
    ClusterWorker, ClusterWorkers, GetActiveWorkers, WorkerStore, WorkerStoreErr,
};
use coerce_rt::actor::context::ActorContext;
use uuid::Uuid;

pub mod util;

pub struct TestWorkerStore;

#[async_trait]
impl WorkerStore for TestWorkerStore {
    async fn get_active(&mut self) -> Result<Vec<ClusterWorker>, WorkerStoreErr> {
        let node_1 = Uuid::new_v4();
        let node_2 = Uuid::new_v4();

        Ok(vec![
            ClusterWorker::new(node_1, RemoteNode::new(node_1, "127.0.0.1", 1024), None),
            ClusterWorker::new(node_2, RemoteNode::new(node_2, "127.0.0.1", 1025), None),
        ])
    }

    async fn put(&mut self, worker: &ClusterWorker) -> Result<(), WorkerStoreErr> {
        unimplemented!()
    }

    async fn remove(&mut self, worker: &ClusterWorker) -> Result<(), WorkerStoreErr> {
        unimplemented!()
    }
}

#[tokio::test]
pub async fn test_remote_worker_store() {
    let mut context = ActorContext::new();
    let mut workers = ClusterWorkers::new(TestWorkerStore, &mut context)
        .await
        .unwrap();

    let active_workers = workers.send(GetActiveWorkers).await.unwrap().unwrap();

    assert_eq!(active_workers.len(), 2);
}