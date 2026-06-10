use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};
use users::get_current_username;

use super::{APP_NAME, input_mode::InputMode};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub struct GlobalAppState {
    pub current_mode: InputMode,
    pub is_running: bool,
    pub theme: ThemeMode,
    pub hotkey: String,
}

impl GlobalAppState {
    pub fn toggle_mode(&mut self) {
        self.current_mode = match self.current_mode {
            InputMode::English => InputMode::Vni,
            InputMode::Vni => InputMode::Telex,
            InputMode::Telex => InputMode::English,
        };
    }
}

#[derive(Deserialize)]
pub struct AppConfig {
    input: InputConfig,
    ui: Option<UiConfig>,
}

#[derive(Deserialize)]
pub struct InputConfig {
    mode: String,
}

#[derive(Deserialize)]
pub struct UiConfig {
    theme: Option<String>,
    hotkey: Option<String>,
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
    let theme = match config.ui.as_ref().and_then(|u| u.theme.as_deref()) {
        Some("Light") => ThemeMode::Light,
        Some("Dark") => ThemeMode::Dark,
        _ => ThemeMode::System,
    };
    let hotkey = config
        .ui
        .and_then(|u| u.hotkey)
        .unwrap_or_else(|| "ctrl+space".to_string());
    GlobalAppState {
        current_mode: mode,
        is_running: true,
        theme,
        hotkey,
    }
}

pub fn set_app_config(mode: InputMode, theme: ThemeMode, hotkey: &str) {
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
    let theme_str = match theme {
        ThemeMode::Light => "Light",
        ThemeMode::Dark => "Dark",
        ThemeMode::System => "System",
    };
    let config = format!(
        "[input]\nmode = \"{}\"\n\n[ui]\ntheme = \"{}\"\nhotkey = \"{}\"\n",
        mode_str, theme_str, hotkey
    );
    std::fs::write(&config_path, config).expect("Failed to write config file");
}
