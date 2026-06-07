use ksni::TrayMethods;
use std::path::Path;
use users::get_current_username;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug)]
struct MimiTray {
    is_running: bool,
}

impl ksni::Tray for MimiTray {
    fn id(&self) -> String {
        APP_NAME.into()
    }

    fn icon_name(&self) -> String {
        "input-keyboard".into()
    }

    fn title(&self) -> String {
        APP_NAME.into()
    }

    fn tool_tip(&self) -> ksni::ToolTip {
        ksni::ToolTip {
            title: format!(
                "{} - {}",
                APP_NAME,
                if self.is_running {
                    "Running"
                } else {
                    "Stopped"
                }
            ),
            ..Default::default()
        }
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: format!(
                    "Status: {}",
                    if self.is_running {
                        "Running ✓"
                    } else {
                        "Stopped ✗"
                    }
                ),
                enabled: false,
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Restart".into(),
                icon_name: "view-refresh".into(),
                activate: Box::new(|this: &mut Self| {
                    this.is_running = true;
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Quit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

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
