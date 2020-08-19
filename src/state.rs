use ds::{DriverStation, Mode, Alliance, TcpPacket};
use crate::ipc::Message;
use crate::webserver::WebsocketHandler;
use actix::Addr;

pub struct State {
    pub ds: DriverStation,
    mode: Mode,
    pub has_joysticks: bool,
}

impl State {
    pub fn new() -> State {
        let mut ds = DriverStation::new_team(0, Alliance::new_red(1));

        ds.set_joystick_supplier(crate::input::joystick_callback);

        State {
            ds,
            mode: Mode::Autonomous,
            has_joysticks: false,
        }
    }

    pub fn wire_stdout(&mut self, addr: Addr<WebsocketHandler>) {
        self.ds.set_tcp_consumer(move |packet| {
            match packet {
                TcpPacket::Stdout(msg) => {
                    let msg = Message::NewStdout { message: msg.message };
                    addr.do_send(msg);
                }
                TcpPacket::Dummy => {}
            }
        });
    }

    pub fn update_ds(&mut self, team_number: u32) {
        self.ds.set_team_number(team_number);
    }

    pub fn enable(&mut self) {
        self.ds.enable();
    }

    pub fn disable(&mut self) {
        self.ds.disable();
    }

    pub fn estop(&mut self) {
        self.ds.estop();
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.ds.set_mode(self.mode);
    }
}