use actix_web::HttpServer;
use shared::configuration::CONFIG;

mod requests;
mod responses;
mod routing;
mod ws_utils;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(routing::route_all)
        .disable_signals()
        .bind((CONFIG.net.ws_ip.clone(), CONFIG.net.ws_port))?
        .run()
        .await
}
