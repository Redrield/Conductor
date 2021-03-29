use crate::state::State;
use crate::webserver::SetAddr;
use ds::DsMode;
use ipc::*;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Duration;
use web_view::{Content, WVResult};

mod cfg;
mod input;
mod ipc;
mod keys;
mod resources;
mod scrn;
mod util;
mod webserver;
use cfg::Config;

mod state;

const PERCENT_WIDTH: f64 = 0.7906295754026355;
const PERCENT_HEIGHT: f64 = 0.42;

fn main() -> WVResult {
    env_logger::init();
    let mut cfg = confy::load::<Config>("conductor").unwrap();

    // You're welcome dalton :)
    #[cfg(target_os = "windows")]
    {
        use tinyfiledialogs::{message_box_ok, MessageBoxIcon};
        message_box_ok("Unsupported Environment", "The Conductor Driver Station is not supported on your operating system. Please use the NI Driver Station instead.\n\nThis application will now terminate.", MessageBoxIcon::Error);

        return std::process::exit(1);
    }

    let state = Arc::new(RwLock::new(State::new()));
    let end_state = state.clone();
    let (tx, rx) = mpsc::channel();
    let (stdout_tx, stdout_rx) = mpsc::channel();

    let port = webserver::launch_webserver(state.clone(), tx, stdout_tx);
    println!("Webserver launched on port {}", port);

    // let screen_size = autopilot::screen::size();
    let (width, height) = scrn::screen_resolution();
    println!("Detected Resolution {} {}", width, height);

    println!(
        "Resized {} {}",
        (width * PERCENT_WIDTH) as i32,
        (height * PERCENT_HEIGHT) as i32
    );
    let mut webview = web_view::builder()
        .title("Conductor DS")
        .content(Content::Url(&format!("http://localhost:{}", port)))
        .size(
            (width * PERCENT_WIDTH) as i32,
            (height * PERCENT_HEIGHT) as i32,
        )
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_, _| Ok(()))
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

    for _ in 0..100 {
        match webview.step() {
            Some(res) => res?,
            None => return Ok(()),
        }
        #[cfg(target_os = "linux")]
        {
            match stdout_wv.step() {
                Some(res) => res?,
                None => return Ok(()),
            }
        }
    }

    // Need to call this to start the app so that it knows the port to connect to
    webview.eval(&format!("window.startapp({})", port)).unwrap();
    #[cfg(target_os = "linux")]
    stdout_wv
        .eval(&format!("window.startapp({})", port))
        .unwrap();

    let addr = rx.recv().unwrap();
    #[cfg(target_os = "linux")]
    {
        let stdout_addr = stdout_rx.recv().unwrap();
        addr.do_send(SetAddr { addr: stdout_addr });
    }
    state.write().unwrap().wire_stdout(addr.clone());

    if cfg.team_number != 0 {
        addr.do_send(Message::UpdateTeamNumber {
            team_number: cfg.team_number,
            from_backend: true,
        });
        state.write().unwrap().update_ds(cfg.team_number);
    }

    // Call to platform-specific function to add hooks for the keybindings
    // If hooks were added the function returns true, if not it returns false. This affects the frontend
    // in both displaying a disclaimer as well as installing local keypress handlers
    let keybindings_enabled = keys::bind_keys(state.clone(), addr.clone());
    addr.do_send(Message::Capabilities {
        backend_keybinds: keybindings_enabled,
    });

    // Start input thread when all the globals are fully initialized
    input::input_thread(addr.clone());

    thread::spawn(move || loop {
        let msg = {
            let state = state.read().unwrap();
            let ds = &state.ds;
            let comms = ds.trace().is_connected();
            let code = ds.trace().is_code_started();
            let sim = ds.ds_mode() == DsMode::Simulation;
            let joysticks = input::JS_STATE
                .get()
                .unwrap()
                .read()
                .unwrap()
                .has_joysticks();
            let voltage = ds.battery_voltage();

            Message::RobotStateUpdate {
                comms_alive: comms,
                code_alive: code,
                simulator: sim,
                joysticks,
                voltage,
            }
        };

        addr.do_send(msg);

        thread::sleep(Duration::from_millis(50));
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
                None => break,
            }
        }
    }

    cfg.team_number = end_state.read().unwrap().ds.team_number();
    log::info!("Updating TN to {}", cfg.team_number);
    confy::store("conductor", cfg).unwrap();
    Ok(())
}
