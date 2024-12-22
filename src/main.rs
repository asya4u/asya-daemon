use tokio::join;

#[macro_use]
extern crate log;

mod logging;
mod preview;

#[tokio::main]
async fn main() {
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C, shutting down...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    preview::show_preview();
    logging::init_logging();
    let server_launching = server::start();
    info!("Bootstrapping");

    _ = join!(server_launching);
}

/*
*
*   ⠀⠀⠀⠀⠀⠀⠀⣀⣤⣶⣿⣿⣿⣿⣿⣿⣶⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
*   ⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
*   ⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
*   ⠀⠀⠀⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
*   ⠀⠀⠀⠀⣿⣿⣿⣿⣿⣟⣿⡏⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
*   ⠀⠀⠀⠀⣿⣿⣿⣿⠟⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀   _   _ _                                                    _        ___      _         _
*   ⠀⠀⠀⠀⣿⣿⣿⣇⡀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  | | | (_)                                                  (_)      / _ \    | |       | |
*   ⠀⠀⠀⠀⢿⣿⣿⣟⣃⡤⠤⠿⠿⣿⣿⣿⣿⣿⣿⣿⡿⢿⣴⡶⠦⢤⡀⠀⠀⠀⠀⠀⠀⠀  | |_| |_     _ __ ___  _   _   _ __   __ _ _ __ ___   ___   _ ___  / /_\ \___| | ____ _| |
*   ⠀⠀⠀⠀⢸⣿⣿⠟⠁⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⡆⠀⢿⣿⠁⠀⠀⠈⢦⠀⠀⠀⠀⠀⠀  |  _  | |   | '_ ` _ \| | | | | '_ \ / _` | '_ ` _ \ / _ \ | / __| |  _  / __| |/ / _` | |
*   ⠀⠀⠀⠀⢸⣿⠯⠀⢠⠀⠀⠀⣼⣿⣿⣿⣿⣿⣿⣧⠀⣿⡏⠀⠀⠀⢠⡀⢧⠀⠀⠀⠀⠀  | | | | |_  | | | | | | |_| | | | | | (_| | | | | | |  __/ | \__ \ | | | \__ \   < (_| |_|
*   ⠀⠀⠀⠀⠸⣿⠀⠀⠘⡆⠀⣰⣿⣿⣿⣿⣿⣿⣿⣜⣷⣿⠁⠀⠀⠀⢸⡇⣸⠀⠀⠀⠀⠀⠀⠀\_| |_/_( ) |_| |_| |_|\__, | |_| |_|\__,_|_| |_| |_|\___| |_|___/ \_| |_/___/_|\_\__,_(_) ⠀⠀⠀⠀
*   ⠀⠀⠀⠀⠀⢻⠀⠀⠘⣿⢠⣿⣿⣿⣿⣿⣿⣟⠿⢿⠛⠁⠓⠆⠀⠀⣼⠁⡟⠀⠀⠀⠀⠀⠀⠀        |/              __/ |                                                              ⠀⠀⠀⠀
*   ⠀⠀⠀⠀⠀⠘⡄⠀⠀⣾⣾⣷⣿⣿⣿⠿⢿⣿⣶⣾⣶⣶⣾⣷⣶⣶⣿⠘⡇⠀⠀⠀⠀⠀⠀⠀                       |___/                                                               ⠀⠀⠀⠀
*   ⠀⠀⠀⠀⠀⠀⡇⠀⠀⢹⣇⣾⣿⢿⡟⠀⠸⣿⡄⢹⡁⠀⠀⠀⠀⠈⢹⢰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
*   ⠀⠀⠀⠀⠀⠀⡇⠀⠀⢸⣿⡟⠉⠘⣇⠀⠀⠉⠙⠺⡇⠀⠀⠀⠀⡓⠘⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
*   ⠀⠀⠀⠀⠀⠀⣷⠀⠀⣼⡿⠧⠒⠒⠛⠛⠒⣶⢤⣄⣳⡀⣀⣀⡤⠥⠤⠬⢷⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀        _____               _   _          _              ___
*   ⠀⠀⠀⠀⠀⢀⣾⡤⠞⠉⠀⠀⠀⠀⠀⠀⠀⠀⠁⢹⣿⣿⡿⠑⠀⠀⠀⠀⠀⠈⠙⠓⠢⣤⣄⣀⠀⠀⠀⠀       /  __ \             (_) | |        | |            |__ \
*   ⠀⠀⠀⢰⢋⣷⠊⠀⠀⠴⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠳⣌⡇⠀⠀⠀       | /  \/ __ _ _ __    _  | |__   ___| |_ __    _   _  ) |
*   ⠀⠀⢀⣯⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢻⡤⢄⡀⠀⠀⠀⠀⠀⠀⠀| |    / _` | '_ \  | | | '_ \ / _ \ | '_ \  | | | |/ /
*   ⢠⡴⢻⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣗⡇⠀⠀⠀⠀⠀⠀⠀| \__/\ (_| | | | | | | | | | |  __/ | |_) | | |_| |_|                    ⠀
*   ⠈⣷⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡇⠀⠀⠀⠀⠀⠀⠀ \____/\__,_|_| |_| |_| |_| |_|\___|_| .__/   \__,_(_)                    ⠀
*   ⠈⢿⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀                                          | |
*   ⠀⣾⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀                                          |_|
*   ⠀⢹⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣷⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡿⠀⠀⠀⠀⠀⠀⠀
*   ⠀⠸⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡾⣿⣿⣿⡟⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⠇⠀⠀⠀⠀⠀⠀⠀
*   ⠀⢀⡷⠖⠒⠲⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠚⢁⣢⣿⡿⡿⣇⠈⠙⠢⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢞⡞⠀⠀⠀⣀⣀⡀⠀⠀
*   ⢠⠞⠀⠀⠀⠀⠀⠹⣶⢤⡀⠀⠀⠀⠀⠀⠀⣸⡇⠀⠀⠀⠈⡇⢀⠔⠒⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠀⢀⡴⠊⠁⠀⠈⢦⡀
*   ⡞⠀⠀⠀⠀⠀⠀⠀⠙⠳⣽⣷⡀⠀⠀⠀⠀⣿⠇⠀⠀⠀⠀⡇⢻⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⢷⣶⡿⠀⠀⠀⠀⠀⠀⢧
*   ⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⢦⡀⠀⢠⢿⠀⠀⠀⠀⠀⢧⠈⠀⠀⠀⠀⣀⡤⠤⠤⠤⣴⣻⣳⠋⠀⠀⠀⠀⠀⠀⠀⠀⣾
*   ⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣧⣏⡇⢹⡀⣸⡏⠀⠀⠀⠀⠀⠘⡆⠀⠀⣰⠋⠁⠀⠀⠀⢰⣳⣧⡇⠀⡆⠀⠀⠀⠀⠀⠀⢀⡟
*   ⠘⣆⠀⠀⠀⠀⠀⠀⣄⢧⣾⣿⣿⠁⠀⢷⡿⠀⠀⠀⠀⠀⠀⠀⢱⠀⡴⠃⠀⠀⠀⠀⠀⣾⣿⢧⣧⠀⡇⢰⠀⠀⠀⠀⠀⣼⠃
*   ⠀⠸⡄⠀⠀⠀⠀⠀⠈⣿⣿⡽⠃⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⢸⢸⠁⠀⠀⠀⠀⠀⠀⠻⣿⣮⣿⣷⡇⠀⡄⠀⠀⠀⣔⡏⠀
*   ⠀⢰⠻⡄⠀⠀⠀⠀⠀⢹⣏⡄⠀⠀⢠⣿⠁⠀⠀⠀⠀⠀⠀⠀⠸⡌⠒⠀⠀⠀⠀⠀⠀⠀⢘⣻⣿⡟⠁⡀⠀⠀⠀⠀⠙⡇⠀
*   ⠀⡼⠰⢿⠀⠀⠀⠀⠀⠈⠛⣶⣦⣀⡾⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠓⠲⢤⣀⣠⠤⠴⠚⠉⠀⡼⠁⠀⠁⠀⠀⠀⠀⠀⢹⠀
*   ⠀⡇⠀⠈⠀⠀⡀⠀⠀⠀⠀⠹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠁⠀⠀⠀⠀⢠⠀⠀⠀⢸⠀
*   ⠀⢧⠀⠀⠀⠀⠹⢤⡀⠀⠀⠀⣻⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡴⢲⠇⠀⠀⠀⠀⠀⠞⠀⠀⠀⣸⠀
*   ⠀⠈⠳⠤⣀⣀⣀⣀⡤⠤⠤⠤⠵⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⠯⢤⣀⣀⣀⡤⠤⠤⠤⠤⠴⠏⠀
*
*/

// Did you know? Coffee protects from Alcheimer's.
// Did you know? Coffee protects from Alcheimer's.
// Did you know? Coffee protects from Alcheimer's.
// Did you know? Coffee protects from Alcheimer's.
// Did you know? Coffee protects from Alcheimer's.
// Did you know? Coffee protects from Alcheimer's.
