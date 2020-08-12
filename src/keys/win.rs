use std::sync::{Arc, RwLock};
use crate::state::State;
use actix::Addr;
use crate::webserver::WebsocketHandler;
use std::thread;
use crate::ipc::Message;
use inputbot::{KeybdKey::{SpaceKey, EnterKey, OtherKey}, handle_input_events};
// use inputbot::{KeybdKey::*, MouseButton::*, *};

extern crate winapi;
use winapi::um::winuser::GetAsyncKeyState;

const LBRACKET_KEYCODE: u64 = 219;
const VBAR_KEYCODE: u64 = 220;
const RBRACKET_KEYCODE: u64 = 221;

pub fn is_pressed(keycode: u64) -> bool {
    (unsafe { GetAsyncKeyState(keycode as i32) } >> 15) != 0
}

pub fn enable_hotkey_pressed() -> bool{
    is_pressed(LBRACKET_KEYCODE) && is_pressed(VBAR_KEYCODE) && is_pressed(RBRACKET_KEYCODE)
}

pub fn bind_keys(state: Arc<RwLock<State>>, addr: Addr<WebsocketHandler>) -> bool {
    let state1 = state.clone();
    let addr1 = addr.clone();
    let state2 = state.clone();
    let addr2 = addr.clone();
    let state3 = state.clone();
    let addr3 = addr.clone();
    let state4 = state.clone();
    let addr4 = addr.clone();
    thread::spawn(|| {
        EnterKey.bind(move || {
            state.write().unwrap().ds.disable();
            addr.do_send(Message::UpdateEnableStatus { enabled: false, from_backend: true });
            println!("Disable the robot from hotkey");
        });
        SpaceKey.bind(move || {
            let mut state1 = state1.write().unwrap();
            if state1.ds.enabled() {
                state1.ds.estop();
                addr1.do_send(Message::EstopRobot { from_backend: true });
                println!("Estop the robot");
            }
        });
        OtherKey(LBRACKET_KEYCODE).bind(move || {
            if enable_hotkey_pressed(){
                state2.write().unwrap().ds.enable();
                addr2.do_send(Message::UpdateEnableStatus { enabled: true, from_backend: true });
                println!("Enable the robot from hotkey");
            }
        });
        OtherKey(VBAR_KEYCODE).bind(move || {
            if enable_hotkey_pressed(){
                state3.write().unwrap().ds.enable();
                addr3.do_send(Message::UpdateEnableStatus { enabled: true, from_backend: true });
                println!("Enable the robot from hotkey");
            }
        });
        OtherKey(RBRACKET_KEYCODE).bind(move || {
            if enable_hotkey_pressed(){
                state4.write().unwrap().ds.enable();
                addr4.do_send(Message::UpdateEnableStatus { enabled: true, from_backend: true });
                println!("Enable the robot from hotkey");
            }
        });

        handle_input_events();

    });
    true
}