use std::{
    fs::{self, File},
    path::Path,
};
use tracing::*;
use tracing_subscriber::EnvFilter;

use shared::configuration::{LoggingLevel, CONFIG};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() {
    let console_layer = fmt::layer().with_writer(std::io::stdout).pretty();
    let file_layer = fmt::layer().with_writer(make_writer()).with_ansi(false);

    let env_filter = EnvFilter::from_default_env()
        .add_directive(CONFIG.logging.level.as_str().parse().unwrap())
        .add_directive("other_module=warn".parse().unwrap());

    let sub = tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer);

    if CONFIG.logging.stdout {
        sub.with(console_layer).init();
    } else {
        sub.init();
    };

    if let LoggingLevel::Trace = CONFIG.logging.level {
        trace!("Check logging level.");
        debug!("Check logging level.");
        info!("Check logging level.");
        warn!("Check logging level.");
        error!("Check logging level.");
    }
}

fn make_writer() -> File {
    let filename = format!(
        "{}/{}asya_logs.log",
        CONFIG.logging.folder,
        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S_")
    );

    let path = Path::new(&CONFIG.logging.folder);
    if !path.exists() {
        fs::create_dir_all(path).expect("The application should be able to create folder to store logs.");
    }
    File::create_new(filename).expect("The application should be able to create a log file in specified folder.")
}
