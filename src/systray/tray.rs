use crate::config::{
    APP_NAME, InputMode,
    settings::{ThemeMode, set_app_config},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

pub enum TrayMessage {
    ModeChanged(InputMode),
}

#[derive(Debug)]
pub struct MimiTray {
    pub current_mode: InputMode,
    pub current_theme: ThemeMode,
    pub current_hotkey: String,
    pub enable_telex: bool,
    pub enable_vni: bool,
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

        let mut modes = vec![InputMode::English];
        if self.enable_vni {
            modes.push(InputMode::Vni);
        }
        if self.enable_telex {
            modes.push(InputMode::Telex);
        }
        let selected = modes
            .iter()
            .position(|m| *m == self.current_mode)
            .unwrap_or(0);

        let options = modes
            .iter()
            .map(|m| RadioItem {
                label: match m {
                    InputMode::English => "English",
                    InputMode::Vni => "VNI",
                    InputMode::Telex => "Telex",
                }
                .into(),
                ..Default::default()
            })
            .collect();

        vec![
            RadioGroup {
                selected,
                select: Box::new(move |this: &mut Self, idx| {
                    let mut modes = vec![InputMode::English];
                    if this.enable_vni {
                        modes.push(InputMode::Vni);
                    }
                    if this.enable_telex {
                        modes.push(InputMode::Telex);
                    }
                    let new_mode = modes[idx];
                    info!("Tray: mode changed to {:?}", new_mode);
                    this.current_mode = new_mode;
                    set_app_config(
                        new_mode,
                        this.current_theme,
                        &this.current_hotkey,
                        this.enable_telex,
                        this.enable_vni,
                    );
                    this.notifier.send(TrayMessage::ModeChanged(new_mode)).ok();
                }),
                options,
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
