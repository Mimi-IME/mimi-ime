use crate::config::settings::{GlobalAppState, ThemeMode, set_app_config};
use eframe::egui;

pub struct SettingsApp {
    state: GlobalAppState,
}

impl SettingsApp {
    pub fn new(state: GlobalAppState, cc: &eframe::CreationContext) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        for path in [
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
            "/run/current-system/sw/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        ] {
            if let Ok(bytes) = std::fs::read(path) {
                fonts.font_data.insert(
                    "system_font".into(),
                    egui::FontData::from_owned(bytes).into(),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "system_font".into());
                break;
            }
        }

        cc.egui_ctx.set_fonts(fonts);
        Self { state }
    }
}

impl eframe::App for SettingsApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut save = false;
        let mut cancel = false;
        let ctx = ui.ctx().clone();

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add_space(8.0);
            ui.label("Giao diện:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.state.theme, ThemeMode::System, "Hệ thống");
                ui.selectable_value(&mut self.state.theme, ThemeMode::Light, "Sáng");
                ui.selectable_value(&mut self.state.theme, ThemeMode::Dark, "Tối");
            });

            ui.add_space(8.0);
            ui.label("Phím tắt chuyển mode:");
            ui.add_space(4.0);
            ui.text_edit_singleline(&mut self.state.hotkey);
            ui.label(
                egui::RichText::new("Ví dụ: ctrl+space, alt+shift")
                    .weak()
                    .small(),
            );

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Lưu").clicked() {
                    save = true;
                }
                if ui.button("Thoát").clicked() {
                    cancel = true;
                }
            });
        });

        if save {
            apply_theme(&ctx, self.state.theme);
            set_app_config(
                self.state.current_mode,
                self.state.theme,
                &self.state.hotkey,
            );
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if cancel {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

fn apply_theme(ctx: &egui::Context, theme: ThemeMode) {
    match theme {
        ThemeMode::Light => ctx.set_visuals(egui::Visuals::light()),
        ThemeMode::Dark => ctx.set_visuals(egui::Visuals::dark()),
        ThemeMode::System => ctx.set_visuals(egui::Visuals::default()),
    }
}
