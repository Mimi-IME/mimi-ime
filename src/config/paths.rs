use std::path::PathBuf;

use super::APP_NAME;

fn home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .expect("HOME not set")
}

fn config_dir() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| home_dir().join(".config"))
}

fn data_dir() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| home_dir().join(".local/share"))
}

pub fn app_config_dir() -> PathBuf {
    config_dir().join(APP_NAME)
}

pub fn app_data_dir() -> PathBuf {
    data_dir().join(APP_NAME)
}

pub fn log_dir() -> PathBuf {
    app_data_dir().join("logs")
}

pub fn config_path() -> PathBuf {
    app_config_dir().join("config.toml")
}

pub fn init_dir() {
    for path in [app_config_dir(), app_data_dir(), log_dir()] {
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Failed to create directory");
        }
    }
    init_config();
}

fn init_config() {
    let path = config_path();
    if !path.exists() {
        let default_config = r#"[input]
mode = "English" # Options: English, Vni, Telex
enable_vni = true
enable_telex = true

[ui]
theme = "System"
hotkey = "ctrl+space"
"#;
        std::fs::write(&path, default_config).expect("Failed to write config file");
    }
}
