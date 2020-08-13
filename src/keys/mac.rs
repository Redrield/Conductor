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


// #[link(name = "CoreGraphics", kind = "framework")]
// extern {
//     fn CGEventSourceKeyState(stateID: i32, key: CGKeyCode) -> bool;
// }

pub fn bind_keys(state: Arc<RwLock<State>>, addr: Addr<WebsocketHandler>) -> bool {
    unsafe {
        thread::spawn(move || {
        });
    }
    true
}