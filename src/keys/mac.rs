use std::sync::{Arc, RwLock};
use crate::state::State;
use actix::Addr;
use crate::webserver::WebsocketHandler;
use core_graphics::event::{KeyCode, CGKeyCode};
use std::thread;
use std::time::Duration;

const kCGEventSourceStateCombinedSessionState: i32 = 0;
const kCGEventSourceStateHIDSystemState: i32 = 1

#[link(name = "CoreGraphics", kind = "framework")]
extern {
    fn CGEventSourceKeyState(stateID: i32, key: CGKeyCode) -> bool;
}

pub fn bind_keys(state: Arc<RwLock<State>>, addr: Addr<WebsocketHandler>) -> bool {
    unsafe {
        thread::spawn(move || {
            let mut return_pressed = false;
            let mut space_pressed = false;

            loop {
                if CGEventSourceKeyState(kCGEventSourceStateCombinedSessionState, KeyCode::RETURN)
                    && !return_pressed {
                    println!("Disable the robot");
                }

                if CGEventSourceKeyState(kCGEventSourceStateCombinedSessionState, KeyCode::SPACE)
                    && !space_pressed {
                    println!("Estop the robot");
                }

                return_pressed = CGEventSourceKeyState(kCGEventSourceStateCombinedSessionState, KeyCode::RETURN);
                space_pressed = CGEventSourceKeyState(kCGEventSourceStateCombinedSessionState, KeyCode::SPACE);
                thread::sleep(Duration::from_millis(20));
            }
        });
    }
    true
}