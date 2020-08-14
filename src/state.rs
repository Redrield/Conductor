use ds::{DriverStation, Mode, Alliance, TcpPacket};
use crate::ipc::Message;
use crate::webserver::{WebsocketHandler, StdoutHandler};
use actix::Addr;

pub struct State {
    pub ds: DriverStation,
    mode: Mode,
    pub has_joysticks: bool,
    stdout_addr: Option<Addr<StdoutHandler>>,
}

impl State {
    pub fn new() -> State {
        let mut ds = DriverStation::new_team(0, Alliance::new_red(1));

        ds.set_joystick_supplier(crate::input::joystick_callback);

        State {
            ds,
            mode: Mode::Autonomous,
            has_joysticks: false,
            stdout_addr: None
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

    // #[cfg(target_os = "linux")]
    pub fn wire_stdout_two(&mut self, addr: Addr<WebsocketHandler>, stdout: Addr<StdoutHandler>) {
        self.stdout_addr = Some(stdout.clone());
        self.ds.set_tcp_consumer(move |packet| {
            match packet {
                TcpPacket::Stdout(msg) => {
                    let msg = Message::NewStdout { message: msg.message };
                    addr.do_send(msg.clone());
                    stdout.do_send(msg);
                }
                TcpPacket::Dummy => {}
            }
        })
    }

    pub fn update_ds(&mut self, team_number: u32) {
        self.ds.set_team_number(team_number);
    }

    pub fn enable(&mut self) {
        self.ds.enable();
        if let Some(addr) = &self.stdout_addr {
            addr.do_send(Message::UpdateEnableStatus { enabled: true, from_backend: true });
        }
    }

    pub fn disable(&mut self) {
        self.ds.disable();
        if let Some(addr) = &self.stdout_addr {
            addr.do_send(Message::UpdateEnableStatus { enabled: false, from_backend: true });
        }
    }

    pub fn estop(&mut self) {
        self.ds.estop();
        if let Some(addr) = &self.stdout_addr {
            addr.do_send(Message::UpdateEnableStatus { enabled: false, from_backend: true });
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.ds.set_mode(self.mode);
    }
}