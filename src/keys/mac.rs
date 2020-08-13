use std::sync::{Arc, RwLock};
use crate::state::State;
use actix::Addr;
use crate::webserver::WebsocketHandler;
use core_graphics::event::{KeyCode, CGKeyCode};
use std::thread;
use std::time::Duration;
use objc::*;
use cocoa::appkit::{NSEvent, NSEventMask};
use block::{Block, ConcreteBlock};
use cocoa::base::id;

#[path = "mac/mgr.rs"]
mod mgr;
use mgr::*;
use crate::ipc;


// #[link(name = "CoreGraphics", kind = "framework")]
// extern {
//     fn CGEventSourceKeyState(stateID: i32, key: CGKeyCode) -> bool;
// }

pub fn bind_keys(state: Arc<RwLock<State>>, addr: Addr<WebsocketHandler>) -> bool {
    unsafe {
        thread::spawn(move || {
            if let Some(mgr) = InputManager::new() {
                let mut return_pressed = false;
                let mut space_pressed = false;

                loop {
                    if mgr.poll_enter() && !return_pressed {
                        state.write().unwrap().ds.disable();
                        addr.do_send(ipc::Message::UpdateEnableStatus { enabled: false, from_backend: true });
                    }
                    if mgr.poll_spacebar() && !space_pressed {
                        let mut state = state.write().unwrap();
                        if state.ds.enabled() {
                            state.ds.estop();
                            addr.do_send(ipc::Message::EstopRobot { from_backend: true });
                        }
                    }

                    return_pressed = mgr.poll_enter();
                    space_pressed = mgr.poll_spacebar();
                    thread::sleep(Duration::from_millis(20))
                }
            } else {
                println!("Failed to crate input manager.");
            }
        });
    }
    true
}