use serde::Deserialize;

use super::input_mode::InputMode;
use super::paths::config_path;
use super::state::{GlobalAppState, ThemeMode};

#[derive(Deserialize)]
pub struct AppConfig {
    input: InputConfig,
    ui: Option<UiConfig>,
}

#[derive(Deserialize)]
pub struct InputConfig {
    mode: String,
    enable_vni: Option<bool>,
    enable_telex: Option<bool>,
}

#[derive(Deserialize)]
pub struct UiConfig {
    theme: Option<String>,
    hotkey: Option<String>,
}

pub fn get_app_config() -> GlobalAppState {
    let content = std::fs::read_to_string(config_path()).expect("Failed to read config file");
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
    let enable_vni = config.input.enable_vni.unwrap_or(true);
    let enable_telex = config.input.enable_telex.unwrap_or(true);

    let mode =
        if (mode == InputMode::Vni && !enable_vni) || (mode == InputMode::Telex && !enable_telex) {
            InputMode::English
        } else {
            mode
        };

    GlobalAppState {
        current_mode: mode,
        is_running: true,
        theme,
        hotkey,
        enable_telex,
        enable_vni,
    }
}

pub fn set_app_config(
    mode: InputMode,
    theme: ThemeMode,
    hotkey: &str,
    enable_telex: bool,
    enable_vni: bool,
) {
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
        "[input]\nmode = \"{}\"\nenable_vni = {}\nenable_telex = {}\n\n[ui]\ntheme = \"{}\"\nhotkey = \"{}\"\n",
        mode_str, enable_vni, enable_telex, theme_str, hotkey
    );
    std::fs::write(config_path(), config).expect("Failed to write config file");
}
