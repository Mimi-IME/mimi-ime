use ksni::TrayMethods;
use mimi_ime::config::get_app_config;
use mimi_ime::config::init_dir;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::{error, info, warn};

use mimi_ime::config::settings::init_logging;
use mimi_ime::input_method::start_input_method;
use mimi_ime::systray::tray::{MimiTray, TrayMessage};

#[tokio::main]
async fn main() {
    init_dir();
    init_logging();

    if std::env::var("DBUS_SESSION_BUS_ADDRESS").is_err() {
        warn!("DBUS_SESSION_BUS_ADDRESS not set — systray will not work");
    }

    info!("mimi-ime starting");

    let app_state = Arc::new(Mutex::new(get_app_config()));
    let app_state_wayland = app_state.clone();
    let app_state_tray = app_state.clone();
    let current_mode = app_state.lock().unwrap().current_mode;
    info!("Loaded config, current mode: {:?}", current_mode);

    std::thread::spawn(|| {
        if let Err(e) = start_input_method(app_state_wayland) {
            error!("Input method error: {}", e);
        }
    });

    let (notifier, mut tray_msgs) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(msg) = tray_msgs.recv().await {
            match msg {
                TrayMessage::ModeChanged(mode) => {
                    info!("Mode changed to: {:?}", mode);
                    app_state_tray.lock().unwrap().current_mode = mode;
                }
            }
        }
    });

    tokio::spawn(async move {
        loop {
            let tray = MimiTray {
                current_mode,
                notifier: notifier.clone(),
            };
            match tray.spawn().await {
                Ok(_) => {
                    info!("Tray started");
                    break;
                }
                Err(e) => {
                    warn!("Tray unavailable, retrying in 3s: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                }
            }
        }
    });

    info!("mimi-ime ready");
    tokio::signal::ctrl_c().await.ok();
    info!("mimi-ime shutting down");

    app_state.lock().unwrap().is_running = false;
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
}
