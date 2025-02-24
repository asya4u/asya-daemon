use crate::AsyaResponse;
use shared::{event_system, shell::execute_command};

pub async fn shutdown() {
    let result = execute_command(vec!["shutdown", "now"]);
    match result {
        Ok(_) => {
            event_system::publish(AsyaResponse::Ok {
                message: "Turning your PC off!".to_string(), // тоби пизда
            })
            .await
        }
        Err(_) => {
            event_system::publish(AsyaResponse::Err {
                message: "Couldn't turn your PC off!".to_string(),
            })
            .await
        }
    }
}

pub async fn reboot() {
    let result = execute_command(vec!["reboot"]);
    match result {
        Ok(_) => {
            event_system::publish(AsyaResponse::Ok {
                message: "Rebooting your PC!".to_string(), // тоби пизда
            })
            .await
        }
        Err(_) => {
            event_system::publish(AsyaResponse::Err {
                message: "Couldn't reboot your PC!".to_string(),
            })
            .await
        }
    }
}
