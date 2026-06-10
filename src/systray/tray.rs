use crate::config::{
    APP_NAME, InputMode,
    settings::{ThemeMode, set_app_config},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

pub enum TrayMessage {
    ModeChanged(InputMode),
    OpenSettings,
}

#[derive(Debug)]
pub struct MimiTray {
    pub current_mode: InputMode,
    pub current_theme: ThemeMode,
    pub current_hotkey: String,
    pub notifier: UnboundedSender<TrayMessage>,
}

impl ksni::Tray for MimiTray {
    fn id(&self) -> String {
        APP_NAME.into()
    }
    fn icon_name(&self) -> String {
        "mimi-ime".into()
    }
    fn title(&self) -> String {
        APP_NAME.into()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;

        let modes = [InputMode::English, InputMode::Vni, InputMode::Telex];
        let selected = modes
            .iter()
            .position(|m| *m == self.current_mode)
            .unwrap_or(0);

        vec![
            RadioGroup {
                selected,
                select: Box::new(|this: &mut Self, idx| {
                    let modes = [InputMode::English, InputMode::Vni, InputMode::Telex];
                    let new_mode = modes[idx];
                    info!("Tray: mode changed to {:?}", new_mode);
                    this.current_mode = new_mode;
                    set_app_config(new_mode, this.current_theme, &this.current_hotkey);
                    this.notifier.send(TrayMessage::ModeChanged(new_mode)).ok();
                }),
                options: vec![
                    RadioItem {
                        label: "English".into(),
                        ..Default::default()
                    },
                    RadioItem {
                        label: "VNI".into(),
                        ..Default::default()
                    },
                    RadioItem {
                        label: "Telex".into(),
                        ..Default::default()
                    },
                ],
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Thiết lập".into(),
                icon_name: "preferences-system".into(),
                activate: Box::new(|this: &mut Self| {
                    info!("Tray: settings requested");
                    this.notifier.send(TrayMessage::OpenSettings).ok();
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Thoát".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| {
                    info!("Tray: quit requested");
                    std::process::exit(0);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}
