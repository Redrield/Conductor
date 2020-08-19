use actix::{Actor, StreamHandler, Message, Handler, Addr};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message as WsMessage, ProtocolError};
use crate::ipc;
use crate::state::State;
use std::sync::{Arc, RwLock};
use crate::ipc::Request;
use crate::input::{self, MappingUpdate};

pub struct StdoutHandler;

impl Actor for StdoutHandler {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<ipc::Message> for StdoutHandler {
    type Result = ();

    fn handle(&mut self, msg: ipc::Message, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ipc::Message::UpdateEnableStatus { .. } | ipc::Message::NewStdout { .. } =>
                ctx.text(serde_json::to_string(&msg).unwrap()),
            _ => {}
        }
    }
}

impl StreamHandler<Result<WsMessage, ws::ProtocolError>> for StdoutHandler {
    fn handle(&mut self, _item: Result<WsMessage, ProtocolError>, _ctx: &mut Self::Context) {
    }
}

pub struct WebsocketHandler {
    state: Arc<RwLock<State>>,
    stdout_addr: Option<Addr<StdoutHandler>>,
}

pub struct SetAddr {
    pub addr: Addr<StdoutHandler>,
}

impl Message for SetAddr {
    type Result = ();
}

impl Message for ipc::Message {
    type Result = ();
}

impl Actor for WebsocketHandler {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<ipc::Message> for WebsocketHandler {
    type Result = ();

    fn handle(&mut self, msg: ipc::Message, ctx: &mut Self::Context) -> Self::Result {
        let json = serde_json::to_string(&msg).unwrap();
        ctx.text(json);
        if let Some(addr) = &self.stdout_addr {
            addr.do_send(msg);
        }
    }
}

impl Handler<SetAddr> for WebsocketHandler {
    type Result = ();

    fn handle(&mut self, msg: SetAddr, _ctx: &mut Self::Context) -> Self::Result {
        self.stdout_addr = Some(msg.addr);
    }
}

impl StreamHandler<Result<WsMessage, ws::ProtocolError>> for WebsocketHandler {
    fn handle(&mut self, item: Result<WsMessage, ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(WsMessage::Text(json)) = item {
            self.handle_message(serde_json::from_str(&json).unwrap(), ctx);
        }
    }
}

impl WebsocketHandler {
    pub fn new(state: Arc<RwLock<State>>) -> WebsocketHandler {
        WebsocketHandler { state, stdout_addr: None }
    }

    pub fn handle_message(&self, msg: ipc::Message, ctx: &mut ws::WebsocketContext<Self>) {
        let mut state = self.state.write().unwrap();
        match msg {
            ipc::Message::UpdateTeamNumber { team_number } => {
                if team_number != state.ds.team_number() {
                    log::info!("Updating team number to {}", team_number);
                    state.update_ds(team_number);
                }
            }
            ipc::Message::UpdateUSBStatus { use_usb } => {
                println!("Trying to connect over USB");
                state.ds.set_use_usb(use_usb);
            }
            ipc::Message::UpdateGSM { gsm } => {
                if gsm.len() == 3 {
                    let _ = state.ds.set_game_specific_message(&gsm);
                }
            }
            ipc::Message::UpdateMode { mode } => {
                println!("Update mode to {:?}", mode);
                state.set_mode(mode.to_ds());
            }
            ipc::Message::UpdateEnableStatus { enabled, .. } => {
                println!("Changing enable status to {}", enabled);

                if let Some(addr) = &self.stdout_addr {
                    addr.do_send(msg);
                }

                if enabled {
                    state.enable();
                } else {
                    state.disable();
                }
            }
            ipc::Message::UpdateJoystickMapping { name, pos } => {
                println!("Got updated joystick mapping: {} => {}", name, pos);
                input::QUEUED_MAPPING_UPDATES.write().unwrap().push(MappingUpdate { name, pos });
            }
            ipc::Message::UpdateAllianceStation { station } => {
                state.ds.set_alliance(station.to_ds());
            }
            ipc::Message::Request { req } => match req {
                Request::RestartRoborio => {
                    state.ds.restart_roborio();
                }
                Request::RestartCode => {
                    state.ds.restart_code();
                }
            }
            ipc::Message::EstopRobot { .. } => state.estop(),
            ipc::Message::QueryEstop => {
                ctx.text(serde_json::to_string(&ipc::Message::RobotEstopStatus { estopped: state.ds.estopped() }).unwrap());
            }
            _ => {}
        }
    }
}