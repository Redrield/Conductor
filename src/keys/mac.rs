use crate::state::State;
use crate::webserver::WebsocketHandler;
use actix::Addr;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[path = "mac/mgr.rs"]
mod mgr;

use crate::ipc;
use mgr::*;

pub fn bind_keys(state: Arc<RwLock<State>>, addr: Addr<WebsocketHandler>) -> bool {
    thread::spawn(move || {
        if let Some(mgr) = InputManager::new() {
            let mut return_pressed = false;
            let mut space_pressed = false;

            loop {
                if mgr.poll_enter() && !return_pressed {
                    println!("MACOS BIT: Disable");
                    state.write().unwrap().disable();
                    addr.do_send(ipc::Message::UpdateEnableStatus {
                        enabled: false,
                        from_backend: true,
                    });
                }
                if mgr.poll_spacebar() && !space_pressed {
                    let mut state = state.write().unwrap();
                    println!("MACOS BIT: Estop");
                    if state.ds.enabled() {
                        state.estop();
                        addr.do_send(ipc::Message::EstopRobot { from_backend: true });
                    }
                }

                return_pressed = mgr.poll_enter();
                space_pressed = mgr.poll_spacebar();
                thread::sleep(Duration::from_millis(20))
            }
        } else {
            println!("Failed to crate input manager.");
            addr.do_send(ipc::Message::Capabilities {
                backend_keybinds: false,
            });
        }
    });
    true
}
