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
    log_file: String
}

impl State {
    pub fn new(log_file: String) -> State {
//        if !Path::new(&log_file).exists() {
//            File::create(log_file.clone()).unwrap();
//        }
        State {
            ds: DriverStation::new_team(0, Alliance::new_red(1)),
            mode: Mode::Autonomous,
            has_joysticks: false,
            log_file,
        }
    }

    fn update_consumer(&mut self) {
//        let mut log_file = OpenOptions::new().write(true).open(&self.log_file).unwrap();
        self.ds.set_tcp_consumer(move |packet| {
            let handle = crate::WV_HANDLE.wait().unwrap();
            let stdout_handle = crate::STDOUT_HANDLE.wait().unwrap();
            let TcpPacket::Stdout(msg) = packet;
//            log_file.write_all(format!("[{:.4}] {}\n", msg.timestamp, msg.message).as_bytes()).unwrap();
            let msg = serde_json::to_string(&Message::NewStdout { message: msg.message }).unwrap();
            let msg2 = msg.clone();
            let _ = handle.dispatch(move |wv| wv.eval(&format!("update({})", msg)));
            let _ = stdout_handle.dispatch(move |wv| wv.eval(&format!("update({})", msg2)));
        });
    }

    pub fn update_ds(&mut self, team_number: u32) {
        self.ds = DriverStation::new_team(team_number, Alliance::new_red(1));
        self.update_consumer();
        self.ds.set_mode(self.mode);
        self.ds.set_joystick_supplier(crate::input::joystick_callback);
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.ds.set_mode(self.mode);
    }
}