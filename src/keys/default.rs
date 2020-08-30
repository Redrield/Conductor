use crate::state::State;
use crate::webserver::WebsocketHandler;
use actix::Addr;
use std::sync::{Arc, RwLock};

pub fn bind_keys(_state: Arc<RwLock<State>>, _addr: Addr<WebsocketHandler>) -> bool {
    false
}
