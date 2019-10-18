#![feature(decl_macro, proc_macro_hygiene)]
use web_view::{Content, WVResult};
use resources::Resources;
use ds::{DriverStation, Alliance, TcpPacket};
use ipc::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Local;
use crate::state::State;
use std::path::Path;

mod resources;
mod ws;
mod ipc;

mod state;

fn main() -> WVResult {
    ws::launch_rocket();

    let log_file_path = format!("stdout-{}.log", Local::now());
    let mut state = Arc::new(Mutex::new(State::new(log_file_path)));

    let wv_state = state.clone();
    let mut webview = web_view::builder()
        .title("Driver Station")
        .content(Content::Url("http://localhost:8000"))
        .size(1000, 300)
        .debug(true)
        .user_data(())
        .invoke_handler(move |wv, arg| {
            let mut state = wv_state.lock().unwrap();
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
//                Message::InitStdout { contents } => {
//                    state.launch_stdout_window(contents, wv.handle());
//                }
                Message::EstopRobot => state.ds.estop(),
                _ => {}
            }
            Ok(())
        })
        .build()?;

    let ticker_state = state.clone();
    let handle = webview.handle();
    thread::spawn(move || {
        loop {
            let msg = {
                let ds = &ticker_state.lock().unwrap().ds;
                let comms = ds.trace().is_connected();
                let code = ds.trace().is_code_started();
                let voltage = ds.battery_voltage();

                Message::RobotStateUpdate { comms_alive: comms, code_alive: code, voltage }
            };

            handle.dispatch(move |wv| wv.eval(&format!("update({})", serde_json::to_string(&msg).unwrap()))).unwrap();

            thread::sleep(Duration::from_millis(50));
        }
    });

    webview.run()
}
