use std::sync::{RwLock, Arc};
use crate::state::State;
use actix::Addr;
use crate::webserver::WebsocketHandler;

pub fn bind_keys(state: Arc<RwLock<State>>, addr: Addr<WebsocketHandler>) -> bool {
    false
}