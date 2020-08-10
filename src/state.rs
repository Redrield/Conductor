use ds::{DriverStation, Mode, Alliance, TcpPacket};
use crate::ipc::Message;
use crate::webserver::WebsocketHandler;
use std::sync::{Arc, Mutex};
use actix::Addr;

pub struct State {
    pub ds: DriverStation,
    mode: Mode,
    pub has_joysticks: bool,
}

impl State {
    pub fn new() -> State {
        let mut ds = DriverStation::new_team(0, Alliance::new_red(1));

        ds.set_tcp_consumer(move |packet| {
            let handle = crate::WV_HANDLE.wait().unwrap();
            #[cfg(target_os = "linux")]
            let stdout_handle = crate::STDOUT_HANDLE.wait().unwrap();
            match packet {
                TcpPacket::Stdout(msg) => {
                    let msg = serde_json::to_string(&Message::NewStdout { message: msg.message }).unwrap();
                    let msg2 = msg.clone();
                    let _ = handle.dispatch(move |wv| wv.eval(&format!("update({})", msg)));
                    #[cfg(target_os = "linux")]
                    let _ = stdout_handle.dispatch(move |wv| wv.eval(&format!("update({})", msg2)));
                }
                TcpPacket::Dummy => {}
            }
        });

        ds.set_joystick_supplier(crate::input::joystick_callback);

        State {
            ds,
            mode: Mode::Autonomous,
            has_joysticks: false,
        }
    }

    pub fn wire_stdout(&mut self, addr: Arc<Mutex<Addr<WebsocketHandler>>>) {
        self.ds.set_tcp_consumer(move |packet| {
            match packet {
                TcpPacket::Stdout(msg) => {
                    let msg = Message::NewStdout { message: msg.message };
                    addr.lock().unwrap().do_send(msg);
                }
                TcpPacket::Dummy => {}
            }
        });
    }

    pub fn update_ds(&mut self, team_number: u32) {
        self.ds.set_team_number(team_number);
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.ds.set_mode(self.mode);
    }
}