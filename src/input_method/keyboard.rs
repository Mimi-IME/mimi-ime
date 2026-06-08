use std::os::fd::AsFd;
use wayland_client::{
    WEnum,
    protocol::wl_keyboard::{KeyState, KeymapFormat},
};
use wayland_protocols_misc::zwp_input_method_v2::client::zwp_input_method_keyboard_grab_v2;
use xkbcommon::xkb;

use crate::config::InputMode;
use crate::input_method::wayland::InputMethodState;

pub fn handle_keyboard_event(
    state: &mut InputMethodState,
    event: zwp_input_method_keyboard_grab_v2::Event,
) {
    match event {
        zwp_input_method_keyboard_grab_v2::Event::Keymap { format, fd, size } => {
            handle_keymap(state, format, fd, size);
        }
        zwp_input_method_keyboard_grab_v2::Event::Modifiers {
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
            ..
        } => {
            state.suppress_until_modifiers_sync = false;
            handle_modifiers(state, mods_depressed, mods_latched, mods_locked, group);
        }
        zwp_input_method_keyboard_grab_v2::Event::Key {
            key,
            state: key_state,
            ..
        } => {
            if state.suppress_until_modifiers_sync {
                return;
            }
            handle_key(state, key, key_state);
        }
        _ => {}
    }
}

fn handle_keymap(
    state: &mut InputMethodState,
    format: WEnum<KeymapFormat>,
    fd: std::os::fd::OwnedFd,
    size: u32,
) {
    if format != WEnum::Value(KeymapFormat::XkbV1) {
        return;
    }

    let fd_for_vk = fd.try_clone().expect("Failed to clone fd");

    let keymap = unsafe {
        xkb::Keymap::new_from_fd(
            &state.xkb_context,
            fd,
            size as usize,
            xkb::KEYMAP_FORMAT_TEXT_V1,
            xkb::COMPILE_NO_FLAGS,
        )
    };

    match keymap {
        Ok(Some(keymap)) => {
            state.xkb_state = Some(xkb::State::new(&keymap));
        }
        Ok(None) => eprintln!("Keymap is None"),
        Err(e) => eprintln!("Failed to create keymap: {}", e),
    }

    if let Some(vk) = &state.virtual_keyboard {
        vk.keymap(1, fd_for_vk.as_fd(), size);
    }
}

fn handle_modifiers(
    state: &mut InputMethodState,
    mods_depressed: u32,
    mods_latched: u32,
    mods_locked: u32,
    group: u32,
) {
    if let Some(xkb_state) = &mut state.xkb_state {
        xkb_state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);
    }
    if let Some(vk) = &state.virtual_keyboard {
        vk.modifiers(mods_depressed, mods_latched, mods_locked, group);
    }
}

fn handle_key(state: &mut InputMethodState, key: u32, key_state: WEnum<KeyState>) {
    let is_pressed = key_state == WEnum::Value(KeyState::Pressed);

    let xkb_keystate = match &state.xkb_state {
        Some(s) => s,
        None => return,
    };

    let ctrl_active =
        xkb_keystate.mod_name_is_active(xkb::MOD_NAME_CTRL, xkb::STATE_MODS_EFFECTIVE);
    let alt_active = xkb_keystate.mod_name_is_active(xkb::MOD_NAME_ALT, xkb::STATE_MODS_EFFECTIVE);

    if ctrl_active || alt_active {
        if is_pressed {
            if !state.pending_chars.is_empty() {
                if let Some(im) = &state.input_method {
                    im.set_preedit_string(String::new(), 0, 0);
                    im.commit(state.serial);
                    state.pending_chars.clear();
                }
            }

            if let Some(kb) = state.keyboard_grab.take() {
                kb.release();
            }
            if let (Some(im), Some(qh)) = (&state.input_method, &state.queue_handle) {
                state.keyboard_grab = Some(im.grab_keyboard(qh, ()));
            }

            state.suppress_until_modifiers_sync = true;
            forward_key(state, key, key_state);

            return;
        } else {
            forward_key(state, key, key_state);
            return;
        }
    }

    let keycode = xkb::Keycode::new(key + 8);
    let keysym = xkb_keystate.key_get_one_sym(keycode);
    let ch = xkb_keystate.key_get_utf8(keycode);

    // Backspace
    if keysym.raw() == xkb::keysyms::KEY_BackSpace {
        if is_pressed && !state.pending_chars.is_empty() {
            handle_backspace(state);
        } else {
            forward_key(state, key, key_state);
        }
        return;
    }

    // Arrow keys, Home, End, Delete, Return, Tab, Escape -> forward to app
    let forward_keys = [
        xkb::keysyms::KEY_Left,
        xkb::keysyms::KEY_Right,
        xkb::keysyms::KEY_Up,
        xkb::keysyms::KEY_Down,
        xkb::keysyms::KEY_Home,
        xkb::keysyms::KEY_End,
        xkb::keysyms::KEY_Delete,
        xkb::keysyms::KEY_Return,
        xkb::keysyms::KEY_Tab,
        xkb::keysyms::KEY_Escape,
    ];

    if forward_keys.iter().any(|&k| keysym.raw() == k) {
        if is_pressed && !state.pending_chars.is_empty() {
            if let Some(im) = &state.input_method {
                let text = state.get_preedit();
                im.set_preedit_string(String::new(), 0, 0);
                im.commit_string(text);
                im.commit(state.serial);
                state.pending_chars.clear();
            }
        }
        forward_key(state, key, key_state);
        return;
    }

    if !is_pressed || ch.is_empty() {
        forward_key(state, key, key_state);
        return;
    }

    let mode = state.app_state.lock().unwrap().current_mode;
    if mode == InputMode::English {
        forward_key(state, key, key_state);
        return;
    }

    handle_char(state, ch);
}

fn forward_key(state: &mut InputMethodState, key: u32, key_state: WEnum<KeyState>) {
    if let Some(vk) = &state.virtual_keyboard {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u32;

        let state_val = match key_state {
            WEnum::Value(KeyState::Pressed) => 1,
            _ => 0,
        };

        vk.key(time, key, state_val);
    }
}

fn handle_backspace(state: &mut InputMethodState) {
    if let Some(im) = &state.input_method {
        state.pending_chars.pop();
        let preedit = state.get_preedit();
        im.set_preedit_string(preedit, -1, -1);
        im.commit(state.serial);
    }
}

fn handle_char(state: &mut InputMethodState, ch: String) {
    let Some(im) = &state.input_method else {
        return;
    };
    let first_char = ch.chars().next().unwrap();

    match first_char {
        ' ' => {
            let mut text = state.get_preedit();
            text.push(' ');
            im.set_preedit_string(String::new(), 0, 0);
            im.commit_string(text);
            im.commit(state.serial);
            state.pending_chars.clear();
        }
        '\n' | '\r' => {
            let text = state.get_preedit();
            if !text.is_empty() {
                im.set_preedit_string(String::new(), 0, 0);
                im.commit_string(text);
                im.commit(state.serial);
                state.pending_chars.clear();
            }
        }
        _ => {
            state.pending_chars.extend(ch.chars());
            let preedit = state.get_preedit();
            im.set_preedit_string(preedit, -1, -1);
            im.commit(state.serial);
        }
    }
}
