pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub mod input_mode;
pub mod logging;
pub mod paths;
pub mod settings;
pub mod state;

pub use input_mode::InputMode;
pub use logging::init_logging;
pub use paths::init_dir;
pub use settings::{get_app_config, set_app_config};
pub use state::{GlobalAppState, ThemeMode};
