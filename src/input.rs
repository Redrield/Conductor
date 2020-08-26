use gilrs::{Gilrs, Gamepad, GamepadId};
use std::sync::RwLock;
use lazy_static::lazy_static;
use std::thread;
use ds::JoystickValue;
use conquer_once::OnceCell;
use std::collections::HashMap;
use std::time::Duration;
use crate::ipc::Message;
use std::iter::FromIterator;
use crate::webserver::WebsocketHandler;
use actix::Addr;
use uuid::Uuid;
use std::str::FromStr;

mod mapping;

lazy_static! {
    pub static ref QUEUED_MAPPING_UPDATES: RwLock<Vec<MappingUpdate>> = RwLock::new(Vec::new());
}

pub static JS_STATE: OnceCell<RwLock<JoystickState>> = OnceCell::uninit();

#[derive(Clone)]
pub struct MappingUpdate {
    pub uuid: String,
    pub pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GamepadData {
    assigned_id: Uuid,
    gid: GamepadId,
    name: String,
}

pub struct JoystickState {
    gil: Gilrs,
    gamepads: Vec<GamepadData>,
    mappings: HashMap<Uuid, usize>,
    addr: Addr<WebsocketHandler>
}

unsafe impl Send for JoystickState {}

unsafe impl Sync for JoystickState {}

impl JoystickState {
    fn new(addr: Addr<WebsocketHandler>) -> JoystickState {
        addr.do_send(Message::JoystickUpdate {
            removed: false,
            name: "Virtual Joystick".to_string(),
            uuid: Uuid::nil().to_string(),
        });
        JoystickState {
            addr,
            gil: Gilrs::new().unwrap(),
            gamepads: Vec::new(),
            mappings: HashMap::new(),
        }
    }

    pub fn has_joysticks(&self) -> bool {
        !self.gamepads.is_empty()
    }

    pub fn add_mapping(&mut self, name: Uuid, pos: usize) {
        self.mappings.insert(name, pos);
    }

    pub fn update(&mut self) {
        self.gil.next_event();

        let new_gamepads = self.gil.gamepads().any(|(id, _)| !self.gamepads.iter().any(|gp| gp.gid == id));
        let removed_gamepads = self.gamepads.iter().any(|gp| !self.gil.gamepad(gp.gid).is_connected());

        if new_gamepads {
            let gp = self.gamepads.clone();
            for (id, gp) in self.gil.gamepads().filter(|(id, _)| !gp.iter().any(|gp| gp.gid == *id)) {
                let gamepad_id = Uuid::new_v4();
                let msg = Message::JoystickUpdate { removed: false, name: gp.name().to_string(), uuid: gamepad_id.to_string() };
                self.addr.do_send(msg);
                let data = GamepadData { assigned_id: gamepad_id, gid: id, name: gp.name().to_string() };
                self.gamepads.push(data);
            }
        }

        if removed_gamepads {
            for (i, gp)in self.gamepads.clone().into_iter().enumerate() {
                if self.gil.gamepad(gp.gid).is_connected() {
                    continue;
                }
                self.gamepads.remove(i);
                let msg = Message::JoystickUpdate { removed: true, uuid: gp.assigned_id.to_string(), name: gp.name };
                self.addr.do_send(msg);
            }
            self.apply_joystick_safety();
        }
    }

    fn apply_joystick_safety(&self) {
        let msg = Message::UpdateEnableStatus { enabled: false, from_backend: true };
        self.addr.do_send(msg);
    }

    fn map_gid(&self, id: GamepadId) -> Option<Uuid> {
        self.gamepads.iter().find(|gp| gp.gid == id).map(|gp| gp.assigned_id)
    }
}

pub fn input_thread(addr: Addr<WebsocketHandler>) {
    JS_STATE.init_once(move || RwLock::new(JoystickState::new(addr)));
    thread::spawn(move || {
        loop {
            {
                let mut state = JS_STATE.get().unwrap().write().unwrap();
                state.update();

                if !QUEUED_MAPPING_UPDATES.read().unwrap().is_empty() {
                    let reqs = {
                        let mut v = QUEUED_MAPPING_UPDATES.write().unwrap();
                        let v2 = v.clone();
                        v.clear();
                        v2
                    };

                    for update in reqs {
                        state.add_mapping(Uuid::from_str(&update.uuid).unwrap(), update.pos);
                    }
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
    });
}

pub fn joystick_callback() -> Vec<Vec<JoystickValue>> {
    let state = match JS_STATE.try_get() {
        Ok(state) => state.read().unwrap(),
        Err(_) => return vec![] // Input thread uninitialized
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
    sorted_joysticks.sort_by(|a, b| mappings.get(&state.map_gid(a.id()).unwrap()).unwrap_or(&0).cmp(mappings.get(&state.map_gid(b.id()).unwrap()).unwrap_or(&1)));

    mapping::apply_mappings(min, sorted_joysticks)
}

