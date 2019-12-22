use serde::{Serialize, Deserialize};
use ds::Mode as DsMode;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    UpdateGSM {
        gsm: String
    },
    UpdateTeamNumber {
        team_number: u32,
    },
    UpdateMode {
        mode: Mode
    },
    UpdateEnableStatus {
        enabled: bool
    },
    JoystickUpdate {
        removed: bool,
        name: String,
    },
    UpdateJoystickMapping {
        name: String,
        pos: usize
    },
    RobotStateUpdate {
        comms_alive: bool,
        code_alive: bool,
        joysticks: bool,
        voltage: f32,
    },
    NewStdout {
        message: String,
    },
    UpdateAllianceStation { station: AllianceStation },
    Request { req: Request, },
    EstopRobot
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(tag = "color", content = "value")]
pub enum AllianceStation {
    Red(u8),
    Blue(u8)
}

impl AllianceStation {
    pub fn to_ds(self) -> ds::Alliance {
        match self {
            AllianceStation::Red(n) => ds::Alliance::new_red(n),
            AllianceStation::Blue(n) => ds::Alliance::new_blue(n)
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Request {
    RestartRoborio,
    RestartCode
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Mode {
    Autonomous,
    Teleoperated,
    Test,
}

impl Mode {
    pub fn to_ds(self) -> DsMode {
        match self {
            Mode::Autonomous => DsMode::Autonomous,
            Mode::Teleoperated => DsMode::Teleoperated,
            Mode::Test => DsMode::Test
        }
    }
}