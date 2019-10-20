#![feature(decl_macro, proc_macro_hygiene)]
use web_view::{Content, WVResult};
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
                    state.update_ds(team_number, wv.handle());
                }
                Message::UpdateMode { mode } => {
                    println!("Update mode to {:?}", mode);
                    state.ds.set_mode(mode.to_ds());
                }
                Message::UpdateEnableStatus { enabled } => {
                    println!("Changing enable status to {}", enabled);

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

    state.write().unwrap().set_handle(webview.handle());

    let ticker_state = state.clone();
    let handle = webview.handle();
    thread::spawn(move || {
        loop {
            let msg = {
                let state = ticker_state.read().unwrap();
                let ds = &state.ds;
                let comms = ds.trace().is_connected();
                let code = ds.trace().is_code_started();
                let joysticks = state.has_joysticks;
                let voltage = ds.battery_voltage();

                Message::RobotStateUpdate { comms_alive: comms, code_alive: code, joysticks, voltage }
            };

            handle.dispatch(move |wv| wv.eval(&format!("update({})", serde_json::to_string(&msg).unwrap()))).unwrap();

            thread::sleep(Duration::from_millis(50));
        }
    });

    input::input_thread(state.clone());

    webview.run()
}
