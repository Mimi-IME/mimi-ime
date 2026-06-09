use serde::Deserialize;
use std::path::Path;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};
use users::get_current_username;

use super::{APP_NAME, input_mode::InputMode};

#[derive(Debug)]
pub struct GlobalAppState {
    pub current_mode: InputMode,
    pub is_running: bool,
}

#[derive(Deserialize)]
struct AppConfig {
    input: InputConfig,
}

#[derive(Deserialize)]
struct InputConfig {
    mode: String,
}

pub fn init_dir() {
    let username = get_current_username().expect("Failed to get current username");
    let config_dir = format!("/home/{}/.config", username.to_string_lossy());
    let local_share_dir = format!("/home/{}/.local/share", username.to_string_lossy());
    let paths = [
        &config_dir,
        &format!("{}/{}", config_dir, APP_NAME),
        &local_share_dir,
        &format!("{}/{}", local_share_dir, APP_NAME),
        &format!("{}/{}/logs", local_share_dir, APP_NAME),
    ];
    for path in paths {
        if !Path::new(path).exists() {
            std::fs::create_dir_all(path).expect("Failed to create directory");
        }
    }
    init_config();
}

fn init_config() {
    let username = get_current_username().expect("Failed to get current username");
    let config_path = format!(
        "/home/{}/.config/{}/config.toml",
        username.to_string_lossy(),
        APP_NAME
    );
    if !Path::new(&config_path).exists() {
        let default_config = r#"[input]
mode = "English" # Options: English, Vni, Telex
"#;
        std::fs::write(&config_path, default_config).expect("Failed to write config file");
    }
}

pub fn init_logging() {
    let username = get_current_username().expect("Failed to get current username");
    let log_dir = format!(
        "/home/{}/.local/share/{}/logs",
        username.to_string_lossy(),
        APP_NAME
    );
    let file_appender = rolling::daily(&log_dir, "mimi-ime.log");
    fmt()
        .with_writer(file_appender)
        .with_env_filter(EnvFilter::new("debug"))
        .with_target(true)
        .with_line_number(true)
        .init();
}

pub fn get_app_config() -> GlobalAppState {
    let username = get_current_username().expect("Failed to get current username");
    let config_path = format!(
        "/home/{}/.config/{}/config.toml",
        username.to_string_lossy(),
        APP_NAME
    );
    let content = std::fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: AppConfig = toml::from_str(&content).expect("Failed to parse config file");
    let mode = match config.input.mode.as_str() {
        "Vni" => InputMode::Vni,
        "Telex" => InputMode::Telex,
        _ => InputMode::English,
    };
    GlobalAppState {
        current_mode: mode,
        is_running: true,
    }
}

pub fn set_app_config(mode: InputMode) {
    let username = get_current_username().expect("Failed to get current username");
    let config_path = format!(
        "/home/{}/.config/{}/config.toml",
        username.to_string_lossy(),
        APP_NAME
    );
    let mode_str = match mode {
        InputMode::English => "English",
        InputMode::Vni => "Vni",
        InputMode::Telex => "Telex",
    };
    let config = format!(
        "[input]\nmode = \"{}\" # Options: English, Vni, Telex\n",
        mode_str
    );
    std::fs::write(&config_path, config).expect("Failed to write config file");
}
