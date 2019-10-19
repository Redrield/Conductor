use gilrs::{Gilrs, Button, Axis};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};
use crate::state::State;
use crate::util::map;
use lazy_static::lazy_static;
use std::thread;
use ds::JoystickValue;
use gilrs::ev::AxisOrBtn;

lazy_static! {
    static ref GIL: RwLock<GilWrapper> = RwLock::new(GilWrapper(Gilrs::new().unwrap()));
}

struct GilWrapper(Gilrs);

impl Deref for GilWrapper {
    type Target = Gilrs;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GilWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// improvise, adapt, overcome
// In all honesty, in order to provide the most updated joystick values to
// the driver station, I need one thread to tick through events, (gil_ticker),
// and the callback which references cached state. This only becomes unsound if I start
// using next_event() over multiple threads.
unsafe impl Send for GilWrapper {}

unsafe impl Sync for GilWrapper {}

pub fn input_thread(state: Arc<Mutex<State>>) {
    thread::spawn(move || {
        loop {
            let mut gil = GIL.write().unwrap();
            let mut joysticks = gil.gamepads().count() > 0;
            state.lock().unwrap().has_joysticks = joysticks;

            gil.next_event();
        }
    });
}

pub fn joystick_callback() -> Vec<Vec<JoystickValue>> {
    let gil = GIL.read().unwrap();

    let mut joysticks = vec![];

    for (id, gamepad) in gil.gamepads() {
        let state = gamepad.state();

        let mut values = vec![];

        let axes = state.axes().filter_map(|(code, axis)| {
            match gamepad.axis_or_btn_name(code) {
                Some(AxisOrBtn::Axis(ax)) => Some((ax, axis)),
                _ => None
            }
        })
            .filter_map(|(axis, value)| {
                if let Some(id) = axis_to_roborio(axis) {
                    Some((id, value))
                } else {
                    None
                }
            })
            .map(|(id, value)| {

                let value = if id == 2 || id == 3 {
                    map(value.value(), -1.0, 1.0, 0.0, 1.0)
                } else if id == 0 {
                    value.value()
                } else {
                    -value.value()
                };

                JoystickValue::Axis { id, value }
            });
        values.extend(axes);


        let buttons = state.buttons().filter_map(|(code, value)| {
            match gamepad.axis_or_btn_name(code) {
                Some(AxisOrBtn::Btn(button)) => Some((button, value)),
                _ => None
            }
        })
            .filter_map(|(button, value)| {
                if let Some(id) = button_to_roborio(button) {
                    Some(JoystickValue::Button { id, pressed: value.is_pressed() })
                } else {
                    None
                }
            });
        values.extend(buttons);

        // POVs
        if gamepad.is_pressed(Button::DPadDown) || gamepad.value(Axis::DPadY) == -1.0 {
            values.push(JoystickValue::POV { id: 0, angle: 180 });
        } else if gamepad.is_pressed(Button::DPadLeft) || gamepad.value(Axis::DPadX) == -1.0 {
            values.push(JoystickValue::POV { id: 0, angle: 270 });
        } else if gamepad.is_pressed(Button::DPadRight) || gamepad.value(Axis::DPadX) == 1.0{
            values.push(JoystickValue::POV { id: 0, angle: 90 });
        } else if gamepad.is_pressed(Button::DPadUp) || gamepad.value(Axis::DPadY) == 1.0 {
            values.push(JoystickValue::POV { id: 0, angle: 0 });
        }

        joysticks.push(values);
    }

    joysticks
}

fn axis_to_roborio(axis: Axis) -> Option<u8>{
    match axis {
        Axis::LeftStickX => Some(0),
        Axis::LeftStickY => Some(1),
        Axis::RightStickX => Some(4),
        Axis::RightStickY => Some(5),
        Axis::LeftZ => Some(2),
        Axis::RightZ => Some(3),
        _ => None
    }
}

fn button_to_roborio(button: Button) -> Option<u8> {
    match button {
        Button::South => Some(1),
        Button::East => Some(2),
        Button::North => Some(3),
        Button::West => Some(4),
        Button::LeftTrigger => Some(5),
        Button::RightTrigger => Some(6),
        Button::Select => Some(7),
        Button::Start => Some(8),
        Button::LeftThumb => Some(9),
        Button::RightThumb => Some(10),
        _ => None
    }
}
