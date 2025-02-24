use actix_web::{dev::ServiceFactory, middleware, web, App, Error};

use crate::ws_utils;

pub fn route_all() -> App<
    impl ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    App::new()
        .wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
        .route("/hey", web::get().to(|| async { "bebra" }))
        .route("/sex", web::get().to(|| async { "не было" }))
        .route("/ws", web::get().to(ws_utils::ws_handler))

    // todo: add events handler
    // .route("/ws/events", web::get().to(ws_utils::ws_events_handler))
}
