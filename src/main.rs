use ksni::TrayMethods;
use std::path::Path;
use users::get_current_username;

use mimi_ime::tray::tray::{APP_NAME, MimiTray};

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

    let _handle = MimiTray { is_running: true }
        .spawn()
        .await
        .expect("Failed to start system tray");

    // Giữ process chạy mãi
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
