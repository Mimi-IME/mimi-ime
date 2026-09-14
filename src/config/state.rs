use serde::{Deserialize, Serialize};

use super::input_mode::InputMode;

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
    pub enable_telex: bool,
    pub enable_vni: bool,
}

impl GlobalAppState {
    pub fn enabled_modes(&self) -> Vec<InputMode> {
        let mut modes = vec![InputMode::English];
        if self.enable_vni {
            modes.push(InputMode::Vni);
        }
        if self.enable_telex {
            modes.push(InputMode::Telex);
        }
        modes
    }

    pub fn toggle_mode(&mut self) {
        let modes = self.enabled_modes();
        let pos = modes
            .iter()
            .position(|m| *m == self.current_mode)
            .unwrap_or(0);
        self.current_mode = modes[(pos + 1) % modes.len()];
    }
}
