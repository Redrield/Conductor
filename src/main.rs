#![feature(decl_macro, proc_macro_hygiene)]
use web_view::{Content, WVResult};

mod resources;
use resources::Resources;
use ds::{DriverStation, Alliance, TcpPacket};

mod ws;
mod ipc;
use ipc::*;
use std::sync::{Arc, Mutex};

fn main() -> WVResult {
    ws::launch_rocket();

    let mut ds = Arc::new(Mutex::new(DriverStation::new_team(0, Alliance::new_red(1))));

    let wv_ds = ds.clone();
    let mut webview = web_view::builder()
        .title("Driver Station")
        .content(Content::Url("http://localhost:8000"))
        .size(800, 250)
        .debug(true)
        .user_data(())
        .invoke_handler(move |_wv, arg| {
            let mut ds = wv_ds.lock().unwrap();
            match serde_json::from_str::<Message>(arg).unwrap() {
                Message::UpdateTeamNumber { team_number } => {
                    //*ds = DriverStation::new_team(team_number, Alliance::new_red(1));
                }
                Message::UpdateMode { mode } => {
                    println!("Update mode to {:?}", mode);
                    //ds.set_mode(mode.to_ds());
                }
                Message::UpdateEnableStatus { enabled } => {
                    //if enabled {
                    //    ds.enable();
                    //} else {
                    //    ds.disable();
                    //}
                }
                _ => {}
            }
            Ok(())
        })
        .build()?;

    let handle = webview.handle();
    ds.lock().unwrap().set_tcp_consumer(move |packet| {
        let TcpPacket::Stdout(msg) = packet;

        handle.dispatch(move |wv| wv.eval(&format!("update({})", serde_json::to_string(&Message::NewStdout { message: msg.message }).unwrap())));
    });

    // run() copied from web-view so we can put in some refresh calls to update the frontend state with each new tick
    loop {
        //let ds = ds.lock().unwrap();

        //let comms_alive = ds.trace().is_connected();
        //let code_alive = ds.trace().is_code_started();
        //let voltage = ds.battery_voltage();

        // Update frontend state
        //webview.eval(&format!("update({})", serde_json::to_string(&Message::RobotStateUpdate { comms_alive, code_alive, voltage }).unwrap()));

        match webview.step() {
            Some(Ok(_)) => continue,
            Some(e) => e?,
            None => return Ok(webview.user_data().clone())
        }
    }
}

