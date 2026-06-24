use ksni::TrayMethods;
use mimi_ime::config::get_app_config;
use mimi_ime::config::init_dir;
use mimi_ime::config::settings::init_logging;
use mimi_ime::input_method::start_input_method;
use mimi_ime::systray::tray::{MimiTray, TrayMessage};
use std::sync::Arc;
use std::sync::Mutex;

use tracing::{error, info, warn};

#[cfg(feature = "settings-ui")]
use mimi_ime::config::settings::{GlobalAppState, ThemeMode};
#[cfg(feature = "settings-ui")]
use mimi_ime::config::settings_ui::SettingsApp;
#[cfg(feature = "settings-ui")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "settings-ui")]
use winit::platform::wayland::EventLoopBuilderExtWayland;

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

    #[cfg(feature = "settings-ui")]
    let settings_open = Arc::new(AtomicBool::new(false));
    #[cfg(feature = "settings-ui")]
    let settings_open_tray = settings_open.clone();
    #[cfg(feature = "settings-ui")]
    let (settings_tx, settings_rx) = std::sync::mpsc::channel::<GlobalAppState>();

    #[cfg(feature = "settings-ui")]
    std::thread::spawn(move || {
        while let Ok(state) = settings_rx.recv() {
            let theme = state.theme;
            let options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default()
                    .with_title("Mimi IME — Settings")
                    .with_inner_size([320.0, 220.0])
                    .with_resizable(false),
                event_loop_builder: Some(Box::new(|builder| {
                    builder.with_any_thread(true);
                })),
                ..Default::default()
            };
            eframe::run_native(
                "mimi-settings",
                options,
                Box::new(|cc| {
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
                    match theme {
                        ThemeMode::Light => cc.egui_ctx.set_visuals(egui::Visuals::light()),
                        ThemeMode::Dark => cc.egui_ctx.set_visuals(egui::Visuals::dark()),
                        ThemeMode::System => {
                            if std::env::var("GTK_THEME")
                                .map(|t| t.contains("dark"))
                                .unwrap_or(false)
                                || std::env::var("COLORFGBG")
                                    .map(|v| v.ends_with(";0"))
                                    .unwrap_or(false)
                            {
                                cc.egui_ctx.set_visuals(egui::Visuals::dark());
                            } else {
                                cc.egui_ctx.set_visuals(egui::Visuals::light());
                            }
                        }
                    }
                    Ok(Box::new(SettingsApp::new(state, cc)))
                }),
            )
            .ok();
            settings_open.store(false, Ordering::SeqCst);
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
                #[cfg(feature = "settings-ui")]
                TrayMessage::OpenSettings => {
                    info!("Opening settings window");
                    if settings_open_tray
                        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                        .is_ok()
                    {
                        let state_snapshot = get_app_config();
                        settings_tx.send(state_snapshot).ok();
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
