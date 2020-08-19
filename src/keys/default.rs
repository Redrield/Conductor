use std::sync::{Arc, RwLock};
use crate::state::State;
use actix::Addr;
use crate::webserver::WebsocketHandler;

pub fn bind_keys(_state: Arc<RwLock<State>>, _addr: Addr<WebsocketHandler>) -> bool {
    false
}