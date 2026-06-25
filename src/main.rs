use ksni::TrayMethods;
use mimi_ime::config::get_app_config;
use mimi_ime::config::init_dir;
use mimi_ime::config::settings::init_logging;
use mimi_ime::input_method::start_input_method;
use mimi_ime::systray::tray::{MimiTray, TrayMessage};
use std::sync::Arc;
use std::sync::Mutex;

use tracing::{error, info, warn};

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
    let current_theme = app_state.lock().unwrap().theme;
    let current_hotkey = app_state.lock().unwrap().hotkey.clone();
    let current_enable_telex = app_state.lock().unwrap().enable_telex;
    let current_enable_vni = app_state.lock().unwrap().enable_vni;
    info!("Loaded config, current mode: {:?}", current_mode);

    let (notifier, mut tray_msgs) = tokio::sync::mpsc::unbounded_channel();
    let notifier_im = notifier.clone();

    std::thread::spawn(|| {
        if let Err(e) = start_input_method(app_state_wayland, notifier_im) {
            error!("Input method error: {}", e);
        }
    });

    let tray_handle: Arc<Mutex<Option<ksni::Handle<MimiTray>>>> = Arc::new(Mutex::new(None));
    let tray_handle_msg = tray_handle.clone();

    tokio::spawn(async move {
        while let Some(msg) = tray_msgs.recv().await {
            match msg {
                TrayMessage::ModeChanged(mode) => {
                    info!("Mode changed to: {:?}", mode);
                    app_state_tray.lock().unwrap().current_mode = mode;
                    let handle = tray_handle_msg.lock().unwrap().clone();
                    if let Some(handle) = handle {
                        handle.update(|tray| tray.current_mode = mode).await;
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        loop {
            let tray = MimiTray {
                current_mode,
                current_theme,
                current_hotkey: current_hotkey.clone(),
                enable_telex: current_enable_telex,
                enable_vni: current_enable_vni,
                notifier: notifier.clone(),
            };
            match tray.spawn().await {
                Ok(handle) => {
                    info!("Tray started");
                    *tray_handle.lock().unwrap() = Some(handle);
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
