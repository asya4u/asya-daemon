use serde::Serialize;
use services::services::info::hardware::{cpu, ram};
use shared::event_system;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::task;

/// Represents the status of the hardware components.
///
/// This enum is used to convey the current status of the system's hardware,
/// specifically the CPU and memory usage. It can be used to monitor and log
/// the performance of the system ot somethings else.
///
/// # Variants
///
/// - `Ok`: Indicates that the hardware is functioning within normal parameters.
///   Contains the current CPU and memory usage.
///
#[derive(Debug, parse_display::Display, Serialize)]
pub enum HardwareStatus {
    /// Indicates that the hardware is functioning within normal parameters.
    ///
    /// # Fields
    ///
    /// - `cpu_usage`: The current CPU usage as a percentage.
    /// - `mem_usage`: The current memory usage in megabytes.
    #[display("CPU: {cpu_usage}%, MEM: {mem_usage}%")]
    Ok { cpu_usage: f32, mem_usage: u64 },
}

/// System monitor for basic monitoring of the system.
pub struct SystemMonitor;

impl SystemMonitor {
    /// Starts basic monitoring of the system. If monitoring is already running, does nothing.
    ///
    /// This method starts an asynchronous task that periodically checks the system's CPU and memory usage.
    /// The results are published as `HardwareStatus::Ok` events every 5 seconds.
    pub async fn start_basic_monitoring() {
        static IS_RUNNING: AtomicBool = AtomicBool::new(false);
        if IS_RUNNING
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            task::spawn(async {
                loop {
                    let cpu_usage = cpu::get_global_usage("".to_string());
                    let mem_usage = ram::get_used_memory("".to_string());
                    event_system::publish(HardwareStatus::Ok {
                        cpu_usage: cpu_usage.unwrap_or(0.0),
                        mem_usage: mem_usage.unwrap_or(0),
                    })
                    .await;
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            });
        }
    }
}
