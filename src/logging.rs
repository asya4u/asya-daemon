use std::{
    fs::{self, File}, io, path::Path
};

use shared::configuration::CONFIG;

pub fn init_logging() {
    tracing_subscriber::fmt()
        .pretty()
        .with_writer(make_writer())
        .with_writer(io::stdout)
        .with_max_level(CONFIG.logging.level.clone())
        .init();
}

fn make_writer() -> File {
    let filename = format!(
        "{}/{}asya_logs.log",
        CONFIG.logging.folder,
        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S_")
    );

    let path = Path::new(&CONFIG.logging.folder);
    if !path.exists() {
        fs::create_dir_all(path).expect("Cannot create folder to store logs.");
    }
    File::create_new(filename).expect("Unable to create log file")
}
