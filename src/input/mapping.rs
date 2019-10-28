use gilrs::{Gamepad, Button, Axis};
use ds::JoystickValue;
use gilrs::ev::AxisOrBtn;
use crate::util::map;

pub fn ensure_mapping_spacing(mappings: &Vec<&usize>, pos: usize, output: &mut Vec<Vec<JoystickValue>>) -> bool {
    if !mappings.contains(&&pos) {
        output.push(vec![]);
        true
    } else {
        false
    }
}

pub fn apply_mappings(offset: usize, mappings: Vec<&usize>, gamepads: Vec<Gamepad>) -> Vec<Vec<JoystickValue>> {
    let mut all_values = vec![];

    if offset != 0 {
        for i in 0..offset {
            all_values.push(vec![])
        }
    }

    for (n, gamepad) in gamepads.iter().enumerate() {
        let mapping_pos = n + offset;

        if ensure_mapping_spacing(&mappings, mapping_pos, &mut all_values) {
            continue;
        }

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
        } else if gamepad.is_pressed(Button::DPadRight) || gamepad.value(Axis::DPadX) == 1.0 {
            values.push(JoystickValue::POV { id: 0, angle: 90 });
        } else if gamepad.is_pressed(Button::DPadUp) || gamepad.value(Axis::DPadY) == 1.0 {
            values.push(JoystickValue::POV { id: 0, angle: 0 });
        }
    }

    all_values
}

fn axis_to_roborio(axis: Axis) -> Option<u8> {
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

#[cfg(test)]
mod tests {
    use crate::input::mapping::ensure_mapping_spacing;
    use ds::JoystickValue;

    #[test]
    fn test_ensure_order() {
        let offset = 2;
        let mut v = vec![vec![], vec![]];

        let mappings = vec![&2usize, &5usize];
        let value = JoystickValue::Axis { id: 0, value: 1.0 };

        for n in 0..=(5-offset) {
            if ensure_mapping_spacing(&mappings, n + offset, &mut v) {
                continue;
            }

            v.push(vec![value]);
        }

        assert_eq!(vec![vec![], vec![], vec![value], vec![], vec![value], vec![]], v);
    }
}
