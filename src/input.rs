use gilrs::{Gilrs, Gamepad};
use std::sync::{RwLock, Arc, Mutex};
use lazy_static::lazy_static;
use std::thread;
use ds::JoystickValue;
use spin::Once;
use std::collections::HashMap;
use std::time::Duration;
use crate::ipc::Message;
use std::iter::FromIterator;
use crate::webserver::WebsocketHandler;
use actix::Addr;

mod mapping;

lazy_static! {
    pub static ref QUEUED_MAPPING_UPDATES: RwLock<Vec<MappingUpdate>> = RwLock::new(Vec::new());
}

pub static JS_STATE: Once<RwLock<JoystickState>> = Once::new();

#[derive(Clone)]
pub struct MappingUpdate {
    pub name: String,
    pub pos: usize,
}

pub struct JoystickState {
    gil: Gilrs,
    gamepad_names: Vec<String>,
    mappings: HashMap<String, usize>,
    addr: Addr<WebsocketHandler>
}

unsafe impl Send for JoystickState {}

unsafe impl Sync for JoystickState {}

impl JoystickState {
    fn new(addr: Addr<WebsocketHandler>) -> JoystickState {
        addr.do_send(Message::JoystickUpdate {
            removed: false,
            name: "Virtual Joystick".to_string()
        });
        JoystickState {
            addr,
            gil: Gilrs::new().unwrap(),
            gamepad_names: Vec::new(),
            mappings: HashMap::new(),
        }
    }

    pub fn has_joysticks(&self) -> bool {
        !self.gamepad_names.is_empty()
    }

    pub fn add_mapping(&mut self, name: String, pos: usize) {
        self.mappings.insert(name, pos);
    }

    pub fn update(&mut self) {
        self.gil.next_event();

        let connected_names = self.gil.gamepads().map(|(_, gp)| gp.name().to_string()).collect::<Vec<String>>();
        if connected_names != self.gamepad_names {
            for new_name in connected_names.iter().filter(|name| !self.gamepad_names.contains(*name)) {
                println!("Got new name; reporting");
                self.report_joystick(new_name.clone(), false);
            }
            let mut joystick_removed = false;
            for old_name in self.gamepad_names.iter().filter(|name| !connected_names.contains(*name)) {
                self.report_joystick(old_name.clone(), true);
                self.mappings.remove(old_name);
                joystick_removed = true;
            }
            self.gamepad_names = connected_names;

            if joystick_removed {
                println!("Detected a joystick removal; trying to apply safety.");
                self.apply_joystick_safety();
            }
        }
    }

    fn apply_joystick_safety(&self) {
        let msg = Message::UpdateEnableStatus { enabled: false };
        println!("Dispatching update to Elm.");
        self.addr.do_send(msg);
    }

    fn report_joystick(&self, name: String, removed: bool) {
        let msg = Message::JoystickUpdate { removed, name };
        // Always unwrap because this should be set prior to anything starting to go
        self.addr.do_send(msg);
    }
}

pub fn input_thread(addr: Addr<WebsocketHandler>) {
    JS_STATE.call_once(move || RwLock::new(JoystickState::new(addr)));
    thread::spawn(move || {
        loop {
            {
                let mut state = JS_STATE.wait().unwrap().write().unwrap();
                state.update();

                if !QUEUED_MAPPING_UPDATES.read().unwrap().is_empty() {
                    let reqs = {
                        let mut v = QUEUED_MAPPING_UPDATES.write().unwrap();
                        let v2 = v.clone();
                        v.clear();
                        v2
                    };

                    for update in reqs {
                        state.add_mapping(update.name, update.pos);
                    }
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
    });
}

pub fn joystick_callback() -> Vec<Vec<JoystickValue>> {
    let state = match JS_STATE.wait() {
        Some(state) => state.read().unwrap(),
        None => return vec![] // Input thread uninitialized
    };

    let gil = &state.gil;
    let mappings = &state.mappings;

    if gil.gamepads().count() == 0 {
        // Non-empty implies there is 1 or more mapping to Virtual Joystick
        return if !mappings.is_empty() {
            let up_bound = *mappings.values().max().unwrap_or(&0)+1;
            Vec::from_iter(std::iter::repeat(vec![]).take(up_bound))
        } else {
            vec![]
        }
    }

    let min = *mappings.values().min().unwrap_or(&0);


    let mut sorted_joysticks = gil.gamepads().map(|(_, gp)| gp).collect::<Vec<Gamepad>>();
    sorted_joysticks.sort_by(|a, b| mappings.get(a.name()).unwrap_or(&0).cmp(mappings.get(b.name()).unwrap_or(&1)));

    mapping::apply_mappings(min, sorted_joysticks)
}

