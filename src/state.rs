use ds::{DriverStation, Mode, Alliance, TcpPacket};
use web_view::{Handle, Error, Content, WebView};
use crate::ipc::Message;
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use std::fs::{self, File, OpenOptions};
use std::io::Write;

pub struct State {
    pub ds: DriverStation,
    mode: Mode,
    pub has_joysticks: bool,
    handle: Option<Handle<()>>,
    log_file: String
}

impl State {
    pub fn new(log_file: String) -> State {
        if !Path::new(&log_file).exists() {
            File::create(log_file.clone()).unwrap();
        }
        State {
            ds: DriverStation::new_team(0, Alliance::new_red(1)),
            mode: Mode::Autonomous,
            has_joysticks: false,
            handle: None,
            log_file,
        }
    }

    fn update_consumer(&mut self, handle: Handle<()>) {
        let mut log_file = OpenOptions::new().write(true).open(&self.log_file).unwrap();
        self.ds.set_tcp_consumer(move |packet| {
            let TcpPacket::Stdout(msg) = packet;
            log_file.write_all(format!("[{:.4}] {}\n", msg.timestamp, msg.message).as_bytes()).unwrap();
            let msg = serde_json::to_string(&Message::NewStdout { message: msg.message }).unwrap();
            let _ = handle.dispatch(move |wv| wv.eval(&format!("update({})", msg)));
        });
    }

    pub fn set_handle(&mut self, handle: Handle<()>) {
        self.handle = Some(handle);
    }

    pub fn report_joystick(&self, name: String, removed: bool) {
        let msg = serde_json::to_string(&Message::JoystickUpdate { removed, name }).unwrap();
        // Always unwrap because this should be set prior to anything starting to go
        let _ = self.handle.as_ref().unwrap().dispatch(move |wv| wv.eval(&format!("update({})", msg)));
    }

    pub fn update_ds(&mut self, team_number: u32, handle: Handle<()>) {
        self.ds = DriverStation::new_team(team_number, Alliance::new_red(1));
        self.update_consumer(handle);
        self.ds.set_mode(self.mode);
        self.ds.set_joystick_supplier(crate::input::joystick_callback);
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.ds.set_mode(self.mode);
    }
}