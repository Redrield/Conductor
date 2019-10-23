#![feature(decl_macro, proc_macro_hygiene)]
use web_view::{Content, WVResult, Handle};
use resources::Resources;
use ds::{DriverStation, Alliance, TcpPacket};
use ipc::*;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use chrono::Local;
use crate::state::State;
use std::path::Path;
use crate::input::MappingUpdate;

mod resources;
mod ws;
mod ipc;
mod input;
mod util;

mod state;

use spin::Once;

static WV_HANDLE: Once<Handle<()>> = Once::new();
static STDOUT_HANDLE: Once<Handle<()>> = Once::new();

fn main() -> WVResult {
    ws::launch_rocket();

    let log_file_path = format!("stdout-{}.log", Local::now());
    let mut state = Arc::new(RwLock::new(State::new(log_file_path)));

    let wv_state = state.clone();
    let mut webview = web_view::builder()
        .title("Driver Station")
        .content(Content::Url("http://localhost:8000"))
        .size(1080, 300)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(move |wv, arg| {
            let mut state = wv_state.write().unwrap();
            match serde_json::from_str::<Message>(arg).unwrap() {
                Message::UpdateTeamNumber { team_number } => {
                    println!("Update to {}", team_number);
                    state.update_ds(team_number);
                }
                Message::UpdateMode { mode } => {
                    println!("Update mode to {:?}", mode);
                    state.set_mode(mode.to_ds());
                }
                Message::UpdateEnableStatus { enabled } => {
                    println!("Changing enable status to {}", enabled);

                    let handle = STDOUT_HANDLE.wait().unwrap();
                    let msg = serde_json::to_string(&Message::UpdateEnableStatus { enabled }).unwrap();
                    let _ = handle.dispatch(move |wv| wv.eval(&format!("update({})", msg)));

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
//                Message::InitStdout { contents } => {
//                    state.launch_stdout_window(contents, wv.handle());
//                }
                Message::EstopRobot => state.ds.estop(),
                _ => {}
            }
            Ok(())
        })
        .build()?;

    let mut stdout_wv = web_view::builder()
        .title("Robot Console")
        .content(Content::Url("http://localhost:8000/stdout"))
        .size(650, 650)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_, _| Ok(()))
        .build()?;

    let handle = webview.handle();
    WV_HANDLE.call_once(move || handle);

    let stdout_handle = stdout_wv.handle();
    STDOUT_HANDLE.call_once(move || stdout_handle);

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

    input::input_thread(webview.handle());

    loop {
        match webview.step() {
            Some(res) => res?,
            None => break
        }

        match stdout_wv.step() {
            Some(res) => res?,
            None => break
        }
    }
    webview.run()
}
