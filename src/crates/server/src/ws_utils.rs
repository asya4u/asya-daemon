use std::sync::Arc;

use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::{AggregatedMessage, Session};
use futures_util::StreamExt;
use tracing::*;
use shared::event_system;
use tokio::{sync::RwLock, task};
use usecases::AsyaResponse;

use crate::{requests::Requests, responses::Responses};

pub async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, stream) = actix_ws::handle(&req, stream)?;
    let session_ptr = Arc::new(RwLock::new(session));
    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    handle_message(session_ptr.clone(), text.to_string()).await;
                }
                Ok(AggregatedMessage::Close(_reason)) => {
                    info!("Closing connection");
                    event_system::unsubscribe_all().await;
                }
                _ => (),
            }
        }
    });

    Ok(res)
}

const DEFAULT_EXPECT_MSG: &str = "The Responses enum should be able to be converted into JSON";

async fn handle_message(session: Arc<RwLock<Session>>, input: String) {
    let request = serde_json::from_str::<Requests>(&input);
    match request {
        Ok(request) => {
            handle_request(request, session).await;
        }
        Err(err) => {
            handle_error(err, session).await;
        }
    }
}

async fn handle_error(err: serde_json::Error, session: Arc<RwLock<Session>>) {
    warn!("Error parsing request: {:?}", err);

    let response = Responses::Base {
        is_err: true,
        message: err.to_string(),
    };
    session
        .write()
        .await
        .text(
            serde_json::to_string(&response)
                .expect(DEFAULT_EXPECT_MSG)
                .to_string(),
        )
        .await
        .unwrap();
}

async fn handle_request(request: Requests, session: Arc<RwLock<Session>>) {
    subscribe_to_asya_response(session).await;
    match request {
        Requests::Command { action } => {
            action.execute("".to_string()).await;
        }
        Requests::Human { message } => {
            usecases::dispatch_by_user_message(message).await;
        }
    }
}

async fn subscribe_to_asya_response(session: Arc<RwLock<Session>>) {
    event_system::subscribe_once({
        let session = session.clone();
        move |event: Arc<AsyaResponse>| {
            let session = session.clone();
            task::spawn(async move {
                let response = Responses::Base {
                    is_err: false,
                    message: event.to_string().replace("\"", ""),
                };

                let res = session
                    .write()
                    .await
                    .text(
                        serde_json::to_string(&response)
                            .expect(DEFAULT_EXPECT_MSG)
                            .to_string(),
                    )
                    .await;

                if let Err(err) = res {
                    warn!("Error sending response: {:?}", err);
                }
            })
        }
    })
    .await;
}
