use ksni::TrayMethods;
use mimi_ime::config::GlobalAppState;
use mimi_ime::config::InputMode;
use mimi_ime::input_method::start_input_method;
use mimi_ime::tray::tray::{APP_NAME, MimiTray, TrayMessage};
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use users::get_current_username;

fn init_dir(local_share_dir: &str) {
    for path in [
        local_share_dir.to_string(),
        format!("{}/{}", local_share_dir, APP_NAME),
        format!("{}/{}/logs", local_share_dir, APP_NAME),
    ] {
        if !Path::new(&path).exists() {
            std::fs::create_dir_all(&path).expect("Failed to create directory");
        }
    }
}

#[tokio::main]
async fn main() {
    if std::env::var("DBUS_SESSION_BUS_ADDRESS").is_err() {
        eprintln!("WARNING: DBUS_SESSION_BUS_ADDRESS not set — systray will not work");
    }

    let username = get_current_username().expect("Failed to get current username");
    let local_share_dir = format!("/home/{}/.local/share", username.to_string_lossy());

    init_dir(&local_share_dir);

    let app_state = Arc::new(Mutex::new(GlobalAppState {
        current_mode: InputMode::Telex,
        is_running: true,
    }));

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
                current_mode: InputMode::Telex,
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
