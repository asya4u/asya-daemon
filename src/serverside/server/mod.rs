use tokio::join;

//use crate::service::fetchservice::{ infobyfilter, memoryinfo};

mod grpc;
mod http;
mod middlewares;

pub async fn launch_server() {
    _ = join!(http::start(), grpc::start());
}
