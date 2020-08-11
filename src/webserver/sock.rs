use actix::{Actor, StreamHandler, AsyncContext, Message, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message as WsMessage, ProtocolError};
use crate::ipc;
use crate::state::State;
use std::sync::{Arc, RwLock};
use crate::ipc::Request;
use crate::input::{self, MappingUpdate};

pub struct WebsocketHandler {
    state: Arc<RwLock<State>>
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
    }
}

impl StreamHandler<Result<WsMessage, ws::ProtocolError>> for WebsocketHandler {
    fn handle(&mut self, item: Result<WsMessage, ProtocolError>, _ctx: &mut Self::Context) {
        if let Ok(WsMessage::Text(json)) = item {
            self.handle_message(serde_json::from_str(&json).unwrap());
        }
    }
}

impl WebsocketHandler {
    pub fn new(state: Arc<RwLock<State>>) -> WebsocketHandler {
        WebsocketHandler { state }
    }

    pub fn handle_message(&self, msg: ipc::Message) {
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

                // #[cfg(target_os = "linux")]
                //     {
                //         let handle = STDOUT_HANDLE.wait().unwrap();
                        // Autoscrolling is disabled with the robot, to make it easier to scroll up to view potential error stack traces.
                        // Updating the state of the robot console window means that it will start autoscrolling with new messages.
                        // let msg = serde_json::to_string(&Message::UpdateEnableStatus { enabled }).unwrap();
                        // let _ = handle.dispatch(move |wv| wv.eval(&format!("update({})", msg)));
                    // }

                if enabled {
                    state.ds.enable();
                } else {
                    state.ds.disable();
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
            ipc::Message::EstopRobot { .. } => state.ds.estop(),
            _ => {}
        }
    }
}