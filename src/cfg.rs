use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub team_number: u32
}
