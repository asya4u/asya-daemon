use shared::configuration::CONFIG;

pub fn init_logging() {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(CONFIG.logging.level.clone())
        .init();
}

// fn build_config(config: log4rs::config::runtime::ConfigBuilder, logfile: FileAppender) -> Config {
//     config
//         .appender(Appender::builder().build("file", Box::new(logfile)))
//         .logger(log4rs::config::Logger::builder().build("teloxide", log::LevelFilter::Off))
//         .logger(log4rs::config::Logger::builder().build("hyper", log::LevelFilter::Off))
//         .logger(log4rs::config::Logger::builder().build("reqwest", log::LevelFilter::Off))
//         .build(
//             Root::builder()
//                 .appender("console")
//                 .appender("file")
//                 .build(CONFIG.logging.level),
//         )
//         .unwrap() // Если stdout не включать, то паника
// }
//
// fn enable_file() -> FileAppender {
//     FileAppender::builder()
//         .encoder(Box::new(PatternEncoder::new(
//             "{f}:{L}: {d(%Y-%m-%d %H:%M:%S)} {h(SERVER)} - {l} > {m}\n",
//         )))
//         .build(format!(
//             "{}/{}aska_logs.log",
//             CONFIG.logging.folder,
//             chrono::Local::now().format("%Y-%m-%d_%H-%M-%S_")
//         ))
//         .unwrap()
// }
//
// fn enable_console(console_pattern: &str) -> ConsoleAppender {
//     ConsoleAppender::builder()
//         .encoder(Box::new(PatternEncoder::new(console_pattern)))
//         .build()
// }
