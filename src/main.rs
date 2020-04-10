use web_view::{Content, WVResult, Handle};
use ipc::*;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use crate::state::State;
use crate::input::MappingUpdate;

mod resources;
mod ws;
mod ipc;
mod input;
mod util;

mod state;

use spin::Once;

static WV_HANDLE: Once<Handle<()>> = Once::new();
#[cfg(target_os = "linux")]
static STDOUT_HANDLE: Once<Handle<()>> = Once::new();

fn main() -> WVResult {
    env_logger::init();
    let port = ws::launch_webserver();
    println!("Webserver launched on port {}", port);

    let state = Arc::new(RwLock::new(State::new()));

    let wv_state = state.clone();
    let mut webview = web_view::builder()
        .title("Driver Station")
        .content(Content::Url(&format!("http://localhost:{}", port)))
        .size(1080, 300)
        //.resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(move |_wv, arg| {
            let mut state = wv_state.write().unwrap();
            match serde_json::from_str::<Message>(arg).unwrap() {
                Message::UpdateTeamNumber { team_number } => {
                    if team_number != state.ds.team_number() {
                        state.update_ds(team_number);
                    }
                }
                Message::UpdateUSBStatus { use_usb } => {
                    println!("Trying to connect over USB");
                    state.ds.set_use_usb(use_usb);
                }
                Message::UpdateGSM { gsm } => {
                    if gsm.len() == 3 {
                        let _ = state.ds.set_game_specific_message(&gsm);
                    }
                }
                Message::UpdateMode { mode } => {
                    println!("Update mode to {:?}", mode);
                    state.set_mode(mode.to_ds());
                }
                Message::UpdateEnableStatus { enabled } => {
                    println!("Changing enable status to {}", enabled);

                    #[cfg(target_os = "linux")]
                    {
                        let handle = STDOUT_HANDLE.wait().unwrap();
                        // Autoscrolling is disabled with the robot, to make it easier to scroll up to view potential error stack traces.
                        // Updating the state of the robot console window means that it will start autoscrolling with new messages.
                        let msg = serde_json::to_string(&Message::UpdateEnableStatus { enabled }).unwrap();
                        let _ = handle.dispatch(move |wv| wv.eval(&format!("update({})", msg)));
                    }

                    if enabled {
                        state.ds.enable();
                    } else {
                        state.ds.disable();
                    }
                }
                Message::UpdateJoystickMapping { name, pos } => {
                    println!("Got updated joystick mapping: {} => {}", name, pos);
                    input::QUEUED_MAPPING_UPDATES.write().unwrap().push(MappingUpdate { name, pos });
                }
                Message::UpdateAllianceStation { station } => {
                    state.ds.set_alliance(station.to_ds());
                }
                Message::Request { req } => match req {
                    Request::RestartRoborio => {
                        state.ds.restart_roborio();
                    }
                    Request::RestartCode => {
                        state.ds.restart_code();
                    }
                }
                Message::EstopRobot => state.ds.estop(),
                _ => {}
            }
            Ok(())
        })
        .build()?;

    #[cfg(target_os = "linux")]
    let mut stdout_wv = web_view::builder()
        .title("Robot Console")
        .content(Content::Url(&format!("http://localhost:{}/stdout", port)))
        .size(650, 650)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_, _| Ok(()))
        .build()?;

    let handle = webview.handle();
    WV_HANDLE.call_once(move || handle);

    #[cfg(target_os = "linux")]
    {
        let stdout_handle = stdout_wv.handle();
        STDOUT_HANDLE.call_once(move || stdout_handle);
    }

    // Start input thread when all the globals are fully initialized
    input::input_thread();

    let ticker_state = state.clone();
    let handle = webview.handle();
    thread::spawn(move || {
        loop {
            let msg = {
                let state = ticker_state.read().unwrap();
                let ds = &state.ds;
                let comms = ds.trace().is_connected();
                let code = ds.trace().is_code_started();
                let joysticks = input::JS_STATE.wait().unwrap().read().unwrap().has_joysticks();
                let voltage = ds.battery_voltage();

                Message::RobotStateUpdate { comms_alive: comms, code_alive: code, joysticks, voltage }
            };

            handle.dispatch(move |wv| wv.eval(&format!("update({})", serde_json::to_string(&msg).unwrap()))).unwrap();

            thread::sleep(Duration::from_millis(50));
        }
    });


    loop {
        match webview.step() {
            Some(res) => res?,
            None => break,
        }

        #[cfg(target_os = "linux")]
        {
            match stdout_wv.step() {
                Some(res) => res?,
                None => break
            }
        }
    }

    Ok(())
}
