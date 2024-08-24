use fetches::{hardware, software};
use tokio::join;
use tonic::transport::Server;

use crate::configuration;

pub mod fetches;

pub async fn start() {
    _ = join!(server())
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", configuration::get().net().grpc_port()).parse().unwrap();

    info!("GRPC Server listening on {}", addr);

    Server::builder()
        .add_service(software::mnt::mnt_server::MntServer::new(
            software::mnt::MntRealization::default(),
        ))
        .add_service(software::sys::sys_server::SysServer::new(
            software::sys::SysRealisation::default(),
        ))
        .add_service(hardware::cpu::cpu_server::CpuServer::new(
            hardware::cpu::CpuRealisation::default(),
        ))
        .serve(addr)
        .await?;

    Ok(())
}
