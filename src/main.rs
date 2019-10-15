#![feature(decl_macro, proc_macro_hygiene)]
use web_view::{Content, WVResult};

mod resources;
use resources::Resources;
use ds::{DriverStation, Alliance, TcpPacket};

mod ws;
mod ipc;
use ipc::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> WVResult {
    ws::launch_rocket();

    let mut ds = Arc::new(Mutex::new(DriverStation::new_team(9999, Alliance::new_red(1))));

    let wv_ds = ds.clone();
    let mut webview = web_view::builder()
        .title("Driver Station")
        .content(Content::Url("http://localhost:8000"))
        .size(1000, 300)
        .debug(true)
        .user_data(())
        .invoke_handler(move |wv, arg| {
            let mut ds = wv_ds.lock().unwrap();
            match serde_json::from_str::<Message>(arg).unwrap() {
                Message::UpdateTeamNumber { team_number } => {
                    println!("Update to {}", team_number);
                    *ds = DriverStation::new_team(team_number, Alliance::new_red(1));

                    let handle = wv.handle();
                    ds.set_tcp_consumer(move |packet| {
                        let TcpPacket::Stdout(msg) = packet;
                        handle.dispatch(|wv| wv.eval(&format!("update({})", serde_json::to_string(&Message::NewStdout { message: msg.message }).unwrap()))).unwrap();
                    });

                }
                Message::UpdateMode { mode } => {
                    println!("Update mode to {:?}", mode);
                    ds.set_mode(mode.to_ds());
                }
                Message::UpdateEnableStatus { enabled } => {
                    println!("Changing enable status to {}", enabled);

                    if enabled {
                        ds.enable();
                    } else {
                        ds.disable();
                    }
                }
                Message::NewStdout { message } => {
                    println!("Got error {}", message);
                }
                _ => {}
            }
            Ok(())
        })
        .build()?;

    let ticker_ds = ds.clone();
    let handle = webview.handle();
    thread::spawn(move || {
        loop {
            let msg = {
                let ds = ticker_ds.lock().unwrap();
                let comms = ds.trace().is_connected();
                let code = ds.trace().is_code_started();
                let voltage = ds.battery_voltage();

                Message::RobotStateUpdate { comms_alive: comms, code_alive: code, voltage }
            };

//            println!("{:?}", msg);
            handle.dispatch(move |wv| wv.eval(&format!("update({})", serde_json::to_string(&msg).unwrap()))).unwrap();

            thread::sleep(Duration::from_millis(50));
        }
    });

    webview.run()
}

