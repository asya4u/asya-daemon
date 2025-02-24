use serde::Serialize;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::{Mutex, OnceCell, RwLock};
use tokio::task;

type AsyncEventHandler =
    Box<dyn Fn(Arc<dyn Any + Send + Sync>) -> task::JoinHandle<()> + Send + Sync>;

pub struct AsyncEventDispatcher {
    sender: Arc<RwLock<Sender<String>>>,
    listeners: Arc<RwLock<HashMap<String, Vec<AsyncEventHandler>>>>,
}

mod dispatching;

static EVENT_DISPATCHER: OnceCell<Arc<AsyncEventDispatcher>> = OnceCell::const_new();
pub async fn init_event_dispatcher(sender: Sender<String>) {
    EVENT_DISPATCHER
        .get_or_init(|| async { Arc::new(AsyncEventDispatcher::new(sender)) })
        .await;
}

async fn get_event_dispatcher() -> Arc<AsyncEventDispatcher> {
    let dispatcher = EVENT_DISPATCHER.get().unwrap();
    dispatcher.clone()
}

pub async fn unsubscribe_all() {
    get_event_dispatcher().await.unsubscribe_all().await;
}

pub async fn subscribe_once<E, F>(handler: F)
where
    E: 'static + Any + Send + Sync,
    F: Fn(Arc<E>) -> task::JoinHandle<()> + Send + Sync + 'static,
{
    get_event_dispatcher().await.subscribe(handler).await;
}

pub async fn publish<E>(event: E)
where
    E: 'static + Any + Send + Sync + std::fmt::Debug + Serialize,
{
    get_event_dispatcher().await.publish(event).await;
}

pub async fn get_channel() -> &'static (Sender<String>, Mutex<Receiver<String>>) {
    static ONCE: OnceCell<(Sender<String>, Mutex<Receiver<String>>)> = OnceCell::const_new();
    ONCE.get_or_init(|| async {
        let (tx, rx) = mpsc::channel(32);
        (tx, Mutex::const_new(rx))
    })
    .await
}
