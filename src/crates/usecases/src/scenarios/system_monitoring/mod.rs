use std::sync::Arc;

use shared::event_system;
use tokio::task;

use crate::{
    shared_workers::{self, HardwareStatus},
    AsyaResponse,
};

/// Asya watches ur hardware.
pub async fn start_basic_monitoring(_: String) {
    shared_workers::SystemMonitor::start_basic_monitoring().await;
    event_system::subscribe_once({
        move |event: Arc<HardwareStatus>| {
            task::spawn(async move {
                let response = AsyaResponse::Ok {
                    message: event.to_string(),
                };

                event_system::publish(response).await;
            })
        }
    })
    .await
}
