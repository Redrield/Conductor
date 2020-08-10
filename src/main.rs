use web_view::{Content, WVResult, Handle};
use ipc::*;
use std::sync::{Arc, RwLock, mpsc, Mutex};
use std::thread;
use std::time::Duration;
use crate::state::State;
use crate::input::MappingUpdate;

mod resources;
mod webserver;
mod ipc;
mod input;
mod util;

mod state;

use spin::Once;

use actix::Addr;
use actix_web_actors::ws;

static WV_HANDLE: Once<Handle<()>> = Once::new();
#[cfg(target_os = "linux")]
static STDOUT_HANDLE: Once<Handle<()>> = Once::new();

fn main() -> WVResult {
    env_logger::init();

    let state = Arc::new(RwLock::new(State::new()));
    let (tx, rx) = mpsc::channel();

    let port = webserver::launch_webserver(state.clone(), tx);
    println!("Webserver launched on port {}", port);

    let mut webview = web_view::builder()
        .title("Driver Station")
        .content(Content::Url(&format!("http://localhost:{}", port)))
        .size(1080, 300)
        //.resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_,_| Ok(()))
        .build()?;

    // #[cfg(target_os = "linux")]
    // let mut stdout_wv = web_view::builder()
    //     .title("Robot Console")
    //     .content(Content::Url(&format!("http://localhost:{}/stdout", port)))
    //     .size(650, 650)
    //     .resizable(true)
    //     .debug(true)
    //     .user_data(())
    //     .invoke_handler(|_, _| Ok(()))
    //     .build()?;
    //
    // let handle = webview.handle();
    // WV_HANDLE.call_once(move || handle);
    //
    // #[cfg(target_os = "linux")]
    // {
    //     let stdout_handle = stdout_wv.handle();
    //     STDOUT_HANDLE.call_once(move || stdout_handle);
    // }

    // let addr = rx.recv().unwrap();

    // Start input thread when all the globals are fully initialized
    // input::input_thread(addr.clone());
    //
    // let ticker_state = state.clone();
    // let ticker_addr = addr.clone();
    // thread::spawn(move || {
    //     loop {
    //         let msg = {
    //             let state = ticker_state.read().unwrap();
    //             let ds = &state.ds;
    //             let comms = ds.trace().is_connected();
    //             let code = ds.trace().is_code_started();
    //             let joysticks = input::JS_STATE.wait().unwrap().read().unwrap().has_joysticks();
    //             let voltage = ds.battery_voltage();
    //
    //             Message::RobotStateUpdate { comms_alive: comms, code_alive: code, joysticks, voltage }
    //         };
    //
    //         ticker_addr.do_send(msg);
    //
    //         thread::sleep(Duration::from_millis(50));
    //     }
    // });


    loop {
        match webview.step() {
            Some(res) => res?,
            None => break,
        }

        // #[cfg(target_os = "linux")]
        // {
        //     match stdout_wv.step() {
        //         Some(res) => res?,
        //         None => break
        //     }
        // }
    }

    Ok(())
}
