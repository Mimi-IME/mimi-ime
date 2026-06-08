use crate::config::InputMode;
use tokio::sync::mpsc::UnboundedSender;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub enum TrayMessage {
    ModeChanged(InputMode),
}

#[derive(Debug)]
pub struct MimiTray {
    pub current_mode: InputMode,
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
                    this.current_mode = new_mode;
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
                label: "Quit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}
