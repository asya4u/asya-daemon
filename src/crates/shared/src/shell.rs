use std::process::Command;

use clap::Parser;

/// Executes command. Returns Ok((stdout, stderr)). If command failed to execute, returns Err.
///
/// args: command and args
/// If args are not provided, also returns Err.
pub fn execute_command(args: Vec<&str>) -> Result<(String, String), String> {
    if args.is_empty(){
        return Err("Command not provided".to_string());
    }

    let result = Command::new(args[0])
        .args(&args[1..])
        .output();

    match result {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout.to_vec()).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr.to_vec()).to_string();
            Ok((stdout, stderr))
        }
        Err(err) => {
            let msg = format!("Command execution failed: {err}");
            Err(msg)
        }
    }
}

pub fn shell_args() -> Args {
    Args::parse()
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "AskaConfig.toml")]
    pub config: String,
}
