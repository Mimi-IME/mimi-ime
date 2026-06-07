pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug)]
pub struct MimiTray {
    pub is_running: bool,
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
