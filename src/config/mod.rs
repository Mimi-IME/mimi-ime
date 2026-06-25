pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub mod input_mode;
pub mod settings;

pub use input_mode::InputMode;
pub use settings::{GlobalAppState, get_app_config, init_dir};
