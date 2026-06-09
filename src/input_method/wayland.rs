use polling::{Event, Events, Poller};
use std::os::fd::AsFd;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::{debug, error, info, warn};
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    globals::{GlobalListContents, registry_queue_init},
    protocol::{wl_registry, wl_seat},
};
use wayland_protocols_misc::zwp_input_method_v2::client::{
    zwp_input_method_keyboard_grab_v2::{self, ZwpInputMethodKeyboardGrabV2},
    zwp_input_method_manager_v2::{self, ZwpInputMethodManagerV2},
    zwp_input_method_v2::{self, ZwpInputMethodV2},
};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};
use xkbcommon::xkb;

use crate::config::GlobalAppState;
use crate::input_method::keyboard::handle_keyboard_event;

pub struct InputMethodState {
    pub seat: Option<wl_seat::WlSeat>,
    pub im_manager: Option<ZwpInputMethodManagerV2>,
    pub input_method: Option<ZwpInputMethodV2>,
    pub keyboard_grab: Option<ZwpInputMethodKeyboardGrabV2>,
    pub serial: u32,
    pub pending_chars: Vec<char>,
    pub xkb_context: xkb::Context,
    pub xkb_state: Option<xkb::State>,
    pub virtual_keyboard: Option<ZwpVirtualKeyboardV1>,
    pub queue_handle: Option<QueueHandle<InputMethodState>>,
    pub suppress_until_modifiers_sync: bool,
    pub app_state: Arc<Mutex<GlobalAppState>>,
}

impl InputMethodState {
    pub fn new(app_state: Arc<Mutex<GlobalAppState>>) -> Self {
        Self {
            seat: None,
            im_manager: None,
            input_method: None,
            keyboard_grab: None,
            serial: 0,
            pending_chars: Vec::new(),
            xkb_context: xkb::Context::new(xkb::CONTEXT_NO_FLAGS),
            xkb_state: None,
            virtual_keyboard: None,
            queue_handle: None,
            suppress_until_modifiers_sync: false,
            app_state,
        }
    }

    pub fn get_preedit(&self) -> String {
        let mode = self.app_state.lock().unwrap().current_mode;

        let mut result = String::new();
        let transformer = mode.get_transformer();
        transformer.transform(self.pending_chars.clone(), &mut result);

        result
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for InputMethodState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "wl_seat" => {
                    state.seat =
                        Some(registry.bind::<wl_seat::WlSeat, _, _>(name, version.min(7), qh, ()));
                }
                "zwp_input_method_manager_v2" => {
                    state.im_manager =
                        Some(registry.bind::<ZwpInputMethodManagerV2, _, _>(name, 1, qh, ()));
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for InputMethodState {
    fn event(
        _: &mut Self,
        _: &wl_seat::WlSeat,
        _: wl_seat::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwpInputMethodManagerV2, ()> for InputMethodState {
    fn event(
        _: &mut Self,
        _: &ZwpInputMethodManagerV2,
        _: zwp_input_method_manager_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwpInputMethodV2, ()> for InputMethodState {
    fn event(
        state: &mut Self,
        im: &ZwpInputMethodV2,
        event: zwp_input_method_v2::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            zwp_input_method_v2::Event::Activate => {
                debug!("IME Activated");
                state.keyboard_grab = Some(im.grab_keyboard(qh, ()));
                state.pending_chars.clear();
            }
            zwp_input_method_v2::Event::Deactivate => {
                debug!("IME Deactivated");
                if let Some(kb) = state.keyboard_grab.take() {
                    kb.release();
                }
                if !state.pending_chars.is_empty() {
                    let text = state.get_preedit();
                    im.set_preedit_string(String::new(), 0, 0);
                    im.commit_string(text);
                    im.commit(state.serial);
                    state.pending_chars.clear();
                }
            }
            zwp_input_method_v2::Event::Done => {
                state.serial += 1;
            }
            _ => {}
        }
    }
}

impl Dispatch<ZwpInputMethodKeyboardGrabV2, ()> for InputMethodState {
    fn event(
        state: &mut Self,
        _: &ZwpInputMethodKeyboardGrabV2,
        event: zwp_input_method_keyboard_grab_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        handle_keyboard_event(state, event);
    }
}

impl Dispatch<ZwpVirtualKeyboardManagerV1, ()> for InputMethodState {
    fn event(
        _: &mut Self,
        _: &ZwpVirtualKeyboardManagerV1,
        _: wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_manager_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwpVirtualKeyboardV1, ()> for InputMethodState {
    fn event(
        _: &mut Self,
        _: &ZwpVirtualKeyboardV1,
        _: wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

pub fn start_input_method(
    app_state: Arc<Mutex<GlobalAppState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect_to_env()?;
    let (globals, mut event_queue) = registry_queue_init::<InputMethodState>(&conn)?;
    let qh = event_queue.handle();
    let mut state = InputMethodState::new(app_state);

    state.queue_handle = Some(qh.clone());

    let mut vk_manager: Option<ZwpVirtualKeyboardManagerV1> = None;

    globals.contents().with_list(|list| {
        for global in list {
            match global.interface.as_str() {
                "wl_seat" => {
                    state.seat = Some(globals.registry().bind::<wl_seat::WlSeat, _, _>(
                        global.name,
                        global.version.min(7),
                        &qh,
                        (),
                    ));
                }
                "zwp_input_method_manager_v2" => {
                    state.im_manager =
                        Some(globals.registry().bind::<ZwpInputMethodManagerV2, _, _>(
                            global.name,
                            1,
                            &qh,
                            (),
                        ));
                }
                "zwp_virtual_keyboard_manager_v1" => {
                    vk_manager = Some(
                        globals
                            .registry()
                            .bind::<ZwpVirtualKeyboardManagerV1, _, _>(global.name, 1, &qh, ()),
                    );
                }
                _ => {}
            }
        }
    });

    if let (Some(vk_mgr), Some(seat)) = (vk_manager, &state.seat) {
        state.virtual_keyboard = Some(vk_mgr.create_virtual_keyboard(seat, &qh, ()));
        debug!("virtual_keyboard created OK");
    } else {
        warn!("virtual_keyboard NOT created — missing vk_manager or seat");
    }

    if let (Some(mgr), Some(seat)) = (&state.im_manager, &state.seat) {
        state.input_method = Some(mgr.get_input_method(seat, &qh, ()));
    } else {
        error!("Compositor doesn't support zwp_input_method_v2");
        return Ok(());
    }

    // Setup poller watch Wayland fd
    let poller = Poller::new()?;
    let wayland_fd = conn.as_fd();
    unsafe {
        poller.add(&wayland_fd, Event::readable(0))?;
    }
    let mut events = Events::new();
    info!("Wayland event loop starting");

    loop {
        event_queue.dispatch_pending(&mut state)?;
        event_queue.flush()?;

        if !state.app_state.lock().unwrap().is_running {
            info!("Shutdown signal received, exiting event loop");
            break;
        }

        events.clear();
        poller.wait(&mut events, Some(std::time::Duration::from_millis(100)))?;

        if !events.is_empty() {
            debug!("Wayland fd readable, reading events");
            poller.modify(wayland_fd, Event::readable(0))?;
            if let Some(guard) = event_queue.prepare_read() {
                guard.read()?;
            }
        }
    }

    // Cleanup
    info!("Cleaning up Wayland objects");
    if let Some(kb) = state.keyboard_grab.take() {
        debug!("Releasing keyboard grab");
        kb.release();
    }
    if let Some(im) = state.input_method.take() {
        debug!("Destroying input method");
        im.destroy();
    }
    if let Some(vk) = state.virtual_keyboard.take() {
        debug!("Destroying virtual keyboard");
        vk.destroy();
    }
    event_queue.flush()?;
    info!("Wayland cleanup complete");

    Ok(())
}
