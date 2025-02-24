use crate::configuration::ConfigProperty;
use mlua::LuaSerdeExt;
use std::path::{Path, PathBuf};

pub fn interpret_configs(
    file_path: &Path,
    config_file_content: String,
) -> Result<ConfigProperty, Box<dyn std::error::Error>> {
    let lua = mlua::Lua::new();
    let base_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
    let watch_list = std::sync::Arc::new(std::sync::Mutex::new(Vec::<PathBuf>::new()));
    let watch_list_clone = watch_list.clone();
    let globals = lua.globals();
    let add_to_watch_list = lua.create_function(move |_, file: String| {
        let file_path = Path::new(&file).canonicalize().map_err(|e| {
            mlua::Error::external(format!("Failed to resolve path {}: {}", file, e))
        })?;
        watch_list_clone
            .lock()
            .map_err(|_| mlua::Error::external("Failed to lock watch list"))?
            .push(file_path);
        Ok(())
    })?;
    globals.set("add_to_watch_list", add_to_watch_list)?;
    // this scans modules around config file
    lua.load(
        r#"
                local orig = package.searchers[2]
                package.searchers[2] = function(module)
                local name, err = package.searchpath(module, package.path)
                if name then
                    add_to_watch_list(name)
                end
                return orig(module)
                end
            "#,
    )
    .exec()?;
    let package = globals.get::<mlua::Value>("package".to_string())?;
    if let mlua::Value::Table(package_table) = package {
        let path = package_table.get::<String>("path".to_string())?;
        let new_path = format!("{}/?.lua;{}", base_dir.display(), path);
        package_table.set("path", new_path)?;
    }
    let val = lua.load(&config_file_content).eval()?;
    let config = lua.from_value(val)?;
    for file in watch_list.lock().unwrap().iter() {
        eprintln!("Watched config file: {}", file.display());
    }
    Ok(config)
}
