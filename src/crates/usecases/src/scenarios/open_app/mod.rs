use std::{
    env,
    fs::{self, DirEntry, File},
    io::Read,
    process::Command,
};

use shared::{configuration::CONFIG, event_system};

use crate::AsyaResponse;

// NOTE: linux only
// NOTE: я не тестил случаи, когда .desktop файл прописан неправильно
// FIXME: заспавненные процессы пишут мусор в логи демона и я не знаю от чего это происходит и как
// это фиксить

pub async fn open(app: String) {
    let paths = path_vecs();

    let desktop_file: Vec<String> = match find_desktop_file(paths, app.clone()) {
        Some(res) => res,
        None => {
            event_system::publish(AsyaResponse::Err {
                message: format!("No app '{}' found!", app),
            })
            .await;
            return;
        }
    };

    let (is_terminal, mut exec) = match parse_desktop_file(desktop_file) {
        Ok((is_terminal, exec)) => (is_terminal, exec),
        Err(err) => {
            event_system::publish(AsyaResponse::Err { message: err }).await;
            return;
        }
    };

    let res = spawn_process(&mut exec, is_terminal);

    match res {
        Ok(_) => {
            event_system::publish(AsyaResponse::Ok {
                message: format!("Opened '{}'", app)
            })
            .await;
        }
        Err(err) => {
            event_system::publish(AsyaResponse::Err { message: err }).await;
        }
    }
}

/// Returns a vec of dir entries from all application paths
///
/// It includes:
/// /usr/share/applications
/// /usr/local/share/applications
/// ~/.local/share/applications
fn path_vecs() -> Vec<Result<DirEntry, std::io::Error>> {
    let mut local = env::var("HOME").unwrap();
    local.push_str("/.local/share/applications");
    let paths = match fs::read_dir(local) {
        Ok(res) => Vec::from_iter(res),
        Err(_) => Vec::new(),
    };
    let mut paths1 = match fs::read_dir("/usr/local/share/applications") {
        Ok(res) => Vec::from_iter(res),
        Err(_) => Vec::new(),
    };
    let mut paths2 = match fs::read_dir("/usr/share/applications") {
        Ok(res) => Vec::from_iter(res),
        Err(_) => Vec::new(),
    };

    let mut paths = paths;
    paths.append(&mut paths1);
    paths.append(&mut paths2);
    paths
}

/// Spawns a process that is independent from the daemon
fn spawn_process(exec: &mut String, is_terminal: bool) -> Result<(), String> {
    exec.push_str(" &");
    let shell_process = if is_terminal {
        // Execute TUI apps
        let mut config_terminal = CONFIG.open.terminal.clone();
        if config_terminal.is_empty(){
            config_terminal.push_str(" -e");
            config_terminal.push_str(exec.as_str());
            let _ = Command::new("/bin/sh")
                .arg("-c")
                .arg(config_terminal)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn();
        }else{
            let fallback = ["kitty", "alacritty", "xfce4-terminal", "konsole", "gnome-terminal"].map(|x| x.to_string());
            for mut terminal in fallback {
                terminal.push_str(" -e");
                terminal.push_str(exec.as_str());
                let cmd = Command::new("/bin/sh")
                    .arg("-c")
                    .arg(terminal)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
                if cmd.is_ok() {
                    break;
                }
            }
            return Err("Terminal not found!".to_string());
        };
        let mut terminal = if !CONFIG.open.terminal.is_empty() {
            CONFIG.open.terminal.clone()
        } else {
            env::var("TERM").unwrap_or("kitty".to_string())
        };
        terminal.push_str(" -e");
        terminal.push_str(exec.as_str());
        Command::new("/bin/sh")
            .arg("-c")
            .arg(terminal)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
    } else {
        // Execute GUI apps
        Command::new("/bin/sh")
            .arg("-c")
            .arg(exec)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
    };

    match shell_process {
        Ok(mut process) => {
            process.wait().unwrap();
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
    Ok(())
}

/// Returns Terminal and Exec parameters from .desktop file.
///
/// Returns error if Name is not provided
/// If Terminal is not provided, the first output value defaults to false
fn parse_desktop_file(desktop_file: Vec<String>) -> Result<(bool, String), String> {
    if !desktop_file.is_empty() {
        let exec = desktop_file
            .iter()
            .filter(|line| line.contains("Exec"))
            .collect::<Vec<_>>();
        let exec = if exec.is_empty() {
            return Err("App's EXEC parameter not provided".to_string());
        } else {
            exec[0].split("=").collect::<Vec<_>>()[1]
        };

        let is_terminal = desktop_file
            .iter()
            .filter(|line| line.contains("Terminal"))
            .collect::<Vec<_>>();
        let is_terminal = if is_terminal.is_empty() {
            "false".to_string()
        } else {
            is_terminal[0].split("=").collect::<Vec<_>>()[1].to_string()
        };
        let is_terminal = matches!(is_terminal.as_str(), "true");

        Ok((is_terminal, exec.to_string()))
    } else {
        Err("App not found".to_string())
    }
}

/// Finds a .desktop file in path by its name. If none is found, returns None.
/// Search is case insensitive.
fn find_desktop_file(
    paths: Vec<Result<DirEntry, std::io::Error>>,
    app: String,
) -> Option<Vec<String>> {
    let mut desktop_file: Vec<String> = vec![];
    for path in paths {
        let path = path.unwrap().path().display().to_string();
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let bind = contents
            .split("\n")
            .filter(|el| el.contains("Name=") && el.to_lowercase().contains(&app.to_lowercase()))
            .collect::<Vec<_>>();
        if !bind.is_empty() {
            desktop_file = contents
                .split("\n")
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            break;
        }
    }
    if desktop_file.is_empty() {
        return None;
    }
    Some(desktop_file)
}
