use std::sync::{Arc, RwLock};
use crate::state::State;
use actix::Addr;
use crate::webserver::WebsocketHandler;
use std::{thread, ptr};
use std::time::Duration;
use crate::ipc::Message;
use x11::keysym::{XK_Return, XK_space, XK_bracketleft, XK_bracketright, XK_backslash};
use x11::xlib::{XInitThreads, XOpenDisplay, XKeysymToKeycode, XQueryKeymap};

pub fn bind_keys(state: Arc<RwLock<State>>, addr: Addr<WebsocketHandler>) -> bool {
    // BEGIN: Rust written like C
    unsafe {
        XInitThreads();
        thread::spawn(move || {
            // Just keep an XDisplay handle until the program dies since this is repeatedly polled
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                println!("Failed to open display");
                // Failure means the frontend needs to handle keybinds,
                addr.do_send(Message::Capabilities { backend_keybinds: false });
                return;
            }

            // Constants for the keycodes we care about
            let return_code = XKeysymToKeycode(display, XK_Return as u64);
            let space_code = XKeysymToKeycode(display, XK_space as u64);
            let left_bracket_code = XKeysymToKeycode(display, XK_bracketleft as u64);
            let right_bracket_code = XKeysymToKeycode(display, XK_bracketright as u64);
            let backslash_code = XKeysymToKeycode(display, XK_backslash as u64);

            // State variables to debounce signals
            let mut return_pressed = false;
            let mut space_pressed = false;
            let mut enable_triggered = true;

            loop {
                let mut keymap: [libc::c_char; 32] = [0; 32];
                XQueryKeymap(display, keymap.as_mut_ptr());

                if check_keycode(keymap, return_code) && !return_pressed {
                    state.write().unwrap().disable();
                    addr.do_send(Message::UpdateEnableStatus { enabled: false, from_backend: true });
                }

                if check_keycode(keymap, space_code) && !space_pressed {
                    let mut state = state.write().unwrap();
                    if state.ds.enabled() {
                        state.estop();
                        addr.do_send(Message::EstopRobot { from_backend: true });
                    }
                }

                if check_keycode(keymap, left_bracket_code) && check_keycode(keymap, right_bracket_code) && check_keycode(keymap, backslash_code)
                    && !enable_triggered {
                    state.write().unwrap().enable();
                }

                return_pressed = check_keycode(keymap, return_code);
                space_pressed = check_keycode(keymap, space_code);
                enable_triggered = check_keycode(keymap, left_bracket_code) && check_keycode(keymap, right_bracket_code) && check_keycode(keymap, backslash_code);

                thread::sleep(Duration::from_millis(20));
            }
        });
    }
    true
}

fn check_keycode(keymap: [libc::c_char; 32], code: u8) -> bool {
    keymap[code as usize / 8] & (1 << (code % 8)) != 0
}
