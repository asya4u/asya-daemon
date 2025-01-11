//! Config database.

use macros::Property;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

use crate::types::AiRecognizeMethod;
use homedir::my_home;
use lazy_static::lazy_static;
use mlua::{Lua, Table, ToLua};

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_path = vec![format!(
            "{}/.config/asya/config.lua",
            my_home().unwrap().unwrap().to_str().unwrap().to_string()
        )];

        let lua_config = {
            if let Some((_config_path, lua_file_content)) = load_any_file(config_path) {
                let lua = Lua::new();

                let config_lua: Table = lua
                    .load(&lua_file_content)
                    .eval()
                    .expect("Lua configuration file must be correct to evaluate");

                let config: ConfigProperty =
                    mlua_serde::from_value(config_lua.to_lua(&lua).unwrap())
                        .expect("Lua config table must be correct to desiralize into Rust struct");

                config
            } else {
                ConfigProperty::default()
            }
        };

        let merged_config = lua_config.merge(serde_env::from_env().unwrap());
        merged_config.verify().unwrap();
        merged_config.unwrap_or_default()
    };
}

pub fn load_any_file(pathes: Vec<String>) -> Option<(String, String)> {
    pathes.into_iter().find_map(|path| {
        std::fs::read_to_string(&path)
            .map(|content| (path, content))
            .ok()
    })
}

/// Represents the configuration of the server.
///
/// For details see `asya-daemon/src/crates/macros/README.md`
#[derive(Debug, Property)]
#[property(name(ConfigProperty), derive(Deserialize, Default, Clone))]
pub struct Config {
    /// Net config group.
    #[property(default, use_type(NetProperty), mergeable)]
    pub net: Net,

    /// Logging config group.
    #[property(default, use_type(LoggingProperty), mergeable)]
    pub logging: Logging,

    /// Ai config group.
    #[property(default, use_type(AiProperty), mergeable)]
    pub ai: Ai,

    /// Plugins config group.
    #[property(default, use_type(PluginsProperty), mergeable)]
    pub plugins: Plugins,

    /// Open apps (похуй)
    #[property(default, use_type(OpenAppsProperty), mergeable)]
    pub open: OpenApps,

    /// Usecases specific options.
    #[property(default, use_type(UsecasesProperty), mergeable)]
    pub usecases: Usecases,
}


#[derive(Debug, Property)]
#[property(name(UsecasesProperty), derive(Deserialize, Default, Clone))]
pub struct Usecases {
    /// If enable, Asya can does some funny things.
    #[property(default)]
    pub test_dangerous_features: bool,
}

#[derive(Debug, Property)]
#[property(name(PluginsProperty), derive(Deserialize, Default, Clone))]
pub struct Plugins {
    /// Folder which contains plugins.
    #[property(default("plugins".to_string()))]
    pub plugins_folder: String,

    #[property(default)]
    pub config: HashMap<String, HashMap<String, ConfigFieldType>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConfigFieldType {
    #[serde(skip_deserializing)]
    NormalizedArray(Vec<String>),
    Array(HashMap<usize, String>), // hashmap here for compatibility with lua.
    Single(String),
}

#[derive(Debug, Property)]
#[property(name(AiProperty), derive(Deserialize, Default, Clone))]
pub struct Ai {
    /// Path to prompts.
    #[property(default("./ai-prompts.yaml".to_string()))]
    pub prompts_path: String,

    /// Groq API token.
    ///
    /// Groq used for recognizing user input to commands and generating answers.
    #[property(default("NOT".to_string()))]
    pub groq_token: String,

    /// Method used for ai recognizing user input.
    ///
    /// Currently we have:
    ///     * `Groq` - uses `console.groq.com`.
    ///     * `AltaS` - uses own model made by alta_s, currently work in progress.
    ///     * `None` - ai will not be used. This means you will be able to use only commands.
    #[property(default(AiRecognizeMethod::Groq))]
    pub recognize_method: AiRecognizeMethod,

    /// The address where the alta_s model is hosted.
    #[property(default)]
    pub alta_s_addr: String,

    // Maybe remove this? alta_s model always should launch automatically.
    #[property(default)]
    pub autolaunch_alta_s: bool,

    /// Path with alta_s model for automatically launch.
    #[property(default)]
    pub alta_s_path: String,
}

#[derive(Debug, Property)]
#[property(name(NetProperty), derive(Deserialize, Default, Clone))]
pub struct Net {
    #[property(default(3001))]
    pub ws_port: u16,

    #[property(default("127.0.0.1".to_string()))]
    pub ws_ip: String,

    #[property(default)]
    pub proxy_addr: String, // todo
}

#[derive(Debug, Property)]
#[property(name(LoggingProperty), derive(Deserialize, Default, Clone))]
pub struct Logging {
    #[property(default(LoggingLevel::Info))]
    pub level: LoggingLevel,

    #[property(default("./logs".to_string()))]
    pub folder: String,

    #[property(default(5))]
    pub filescount: usize, // todo

    #[property(default(true))]
    pub stdout: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "String")]
pub enum LoggingLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<String> for LoggingLevel {
    fn from(value: String) -> Self {
        match value.as_str().to_lowercase().as_str() {
            "error" => LoggingLevel::Error,
            "warn" => LoggingLevel::Warn,
            "info" => LoggingLevel::Info,
            "debug" => LoggingLevel::Debug,
            "trace" => LoggingLevel::Trace,
            _ => panic!("Cannot recognize logging level: {}", value),
        }
    }
}

impl LoggingLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoggingLevel::Error => "error",
            LoggingLevel::Warn => "warn",
            LoggingLevel::Info => "info",
            LoggingLevel::Debug => "debug",
            LoggingLevel::Trace => "trace",
        }
    }
}

#[derive(Debug, Property)]
#[property(name(OpenAppsProperty), derive(Deserialize, Default, Clone))]
pub struct OpenApps {
    #[property(default)]
    pub terminal: String,

    #[property(default)]
    pub browser: String,
}
