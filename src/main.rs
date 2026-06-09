use ksni::TrayMethods;
use mimi_ime::config::get_app_config;
use mimi_ime::config::init_dir;
use std::sync::Arc;
use std::sync::Mutex;

use mimi_ime::input_method::start_input_method;
use mimi_ime::systray::tray::{MimiTray, TrayMessage};

#[tokio::main]
async fn main() {
    if std::env::var("DBUS_SESSION_BUS_ADDRESS").is_err() {
        eprintln!("WARNING: DBUS_SESSION_BUS_ADDRESS not set — systray will not work");
    }

    init_dir();

    let app_state = Arc::new(Mutex::new(get_app_config()));
    let current_mode = app_state.lock().unwrap().current_mode;

    let app_state_for_wayland = app_state.clone();
    std::thread::spawn(|| {
        if let Err(e) = start_input_method(app_state_for_wayland) {
            eprintln!("Input method error: {}", e);
        }
    });

    let (notifier, mut tray_msgs) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(msg) = tray_msgs.recv().await {
            match msg {
                TrayMessage::ModeChanged(mode) => {
                    app_state.lock().unwrap().current_mode = mode;
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
                Ok(_handle) => {
                    eprintln!("Tray started");
                    break;
                }
                Err(e) => {
                    eprintln!("Tray unavailable, retrying in 3s: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                }
            }
        }
    });

    tokio::signal::ctrl_c().await.ok();
}
