use std::sync::{Arc, Mutex};
use tracing::{debug, warn};
use wayland_client::protocol::wl_keyboard::KeyState;
use wayland_client::{QueueHandle, WEnum, protocol::wl_seat};
use wayland_protocols::wp::text_input::zv3::client::zwp_text_input_v3::{
    ChangeCause, ContentHint, ContentPurpose,
};
use wayland_protocols_misc::zwp_input_method_v2::client::{
    zwp_input_method_keyboard_grab_v2::ZwpInputMethodKeyboardGrabV2,
    zwp_input_method_manager_v2::ZwpInputMethodManagerV2, zwp_input_method_v2::ZwpInputMethodV2,
};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1;
use xkbcommon::xkb;

use crate::{config::GlobalAppState, systray::tray::TrayMessage};

#[derive(Debug)]
pub enum PendingOp<'a> {
    Push(&'a str),
    Pop(char),
    Clear(&'a str),
    Commit(&'a str),
    Observe(&'a str),
}

pub fn log_pending(state: &InputMethodState, op: PendingOp) {
    let chars: String = state.pending_chars.iter().collect();
    match op {
        PendingOp::Push(s) => debug!(
            "CHAR push {:?} | pending={:?} | own={} serial={} cause={:?}",
            s, chars, state.pending_own_commits, state.serial, state.pending_text_change_cause
        ),
        PendingOp::Pop(c) => debug!(
            "CHAR pop {:?} | pending={:?} | own={}",
            c, chars, state.pending_own_commits
        ),
        PendingOp::Clear(reason) => {
            if !state.pending_chars.is_empty() {
                warn!(
                    "CHAR CLEAR ({}) | lost={:?} | own={} serial={} cause={:?} last_commit_age={:?}",
                    reason,
                    chars,
                    state.pending_own_commits,
                    state.serial,
                    state.pending_text_change_cause,
                    state.last_own_commit_at.map(|t| t.elapsed()),
                );
            }
        }
        PendingOp::Commit(t) => debug!(
            "CHAR commit | text={:?} | pending={:?} | own={} serial={}",
            t, chars, state.pending_own_commits, state.serial
        ),
        PendingOp::Observe(tag) => debug!(
            "CHAR observe ({}) | pending={:?} | own={} serial={} cause={:?}",
            tag, chars, state.pending_own_commits, state.serial, state.pending_text_change_cause
        ),
    }
}

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
    pub repeat_rate: i32,
    pub repeat_delay: i32,
    pub repeat_key: u32,
    pub backspace_held_since: Option<std::time::Instant>,
    pub backspace_last_repeat: Option<std::time::Instant>,
    pub app_state: Arc<Mutex<GlobalAppState>>,
    pub notifier: tokio::sync::mpsc::UnboundedSender<TrayMessage>,
    pub surrounding_text: String,
    pub surrounding_cursor: i32,
    pub surrounding_anchor: i32,
    pub content_hint: ContentHint,
    pub content_purpose: ContentPurpose,
    pub pending_text_change_cause: Option<ChangeCause>,
    pub pending_own_commits: u32,
    pub last_own_commit_at: Option<std::time::Instant>,
    pub last_key_event_at: Option<std::time::Instant>,
    pub pending_commit: bool,
    pub surrounding_initialized: bool,
    pub pending_forward_keys: Vec<(u32, WEnum<KeyState>)>,
    pub pending_forward_deadline: Option<std::time::Instant>,
}

impl InputMethodState {
    pub fn new(
        app_state: Arc<Mutex<GlobalAppState>>,
        notifier: tokio::sync::mpsc::UnboundedSender<TrayMessage>,
    ) -> Self {
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
            repeat_rate: 0,
            repeat_delay: 600,
            repeat_key: 0,
            backspace_held_since: None,
            backspace_last_repeat: None,
            app_state,
            notifier,
            surrounding_text: String::new(),
            surrounding_cursor: -1,
            surrounding_anchor: -1,
            content_hint: ContentHint::None,
            content_purpose: ContentPurpose::Normal,
            pending_text_change_cause: None,
            pending_own_commits: 0,
            last_own_commit_at: None,
            last_key_event_at: None,
            pending_commit: false,
            surrounding_initialized: false,
            pending_forward_keys: Vec::new(),
            pending_forward_deadline: None,
        }
    }

    pub fn get_preedit(&self) -> String {
        let mode = self.app_state.lock().unwrap().current_mode;

        let mut result = String::new();
        let transformer = mode.get_transformer();
        transformer.transform(self.pending_chars.clone(), &mut result);

        result
    }

    pub fn im_commit(&mut self) {
        if self.serial == 0 {
            debug!("im_commit deferred (serial=0, no Done yet)");
            self.pending_commit = true;
            return;
        }
        self.commit_now();
    }

    pub fn commit_now(&mut self) {
        let serial = self.serial;
        if let Some(im) = &self.input_method {
            im.commit(serial);
        }
        self.pending_own_commits = self.pending_own_commits.saturating_add(1);
        self.last_own_commit_at = Some(std::time::Instant::now());
    }

    pub fn clear_pending(&mut self, reason: &'static str) {
        if !self.pending_chars.is_empty() {
            log_pending(self, PendingOp::Clear(reason));
            self.pending_chars.clear();
        }
    }
}
