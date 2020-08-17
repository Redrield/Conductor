use serde::{Serialize, Deserialize};
use ds::Mode as DsMode;

/// Messages sent between the frontend and backend of the application
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Message {
    /// Backend -> Frontend
    /// The capabilities of the compiled backend
    /// If compiled on Linux with X11, the backend handles the keybinds for disabling and estopping
    /// which it can do without regard for window focus. Otherwise, these need to be handled on the frontend
    Capabilities {
        backend_keybinds: bool,
    },
    /// Frontend -> Backend
    /// Updates the Game Specific Message (Game Data) in the driver station
    UpdateGSM {
        gsm: String
    },
    /// Frontend -> Backend
    /// Updates the team number, and the address the driver station is trying to connect to
    UpdateTeamNumber {
        team_number: u32,
    },
    /// Frontend -> Backend
    /// Tells the driver station whether to override the network IP and attempt to connect to the USB
    /// interface at 172.22.11.2
    UpdateUSBStatus {
        use_usb: bool
    },
    /// Frontend -> Backend
    /// Updates the operating mode of the driver station
    UpdateMode {
        mode: Mode
    },
    /// Frontend <-> Backend
    /// Updates the enable status of the robot
    /// May be sent from the backend if a joystick is removed when enabled, or if the backend
    /// is configured to provide global disable hotkeys.
    UpdateEnableStatus {
        enabled: bool,
        from_backend: bool
    },
    /// Backend -> Frontend
    /// Informs the UI of a joystick that has been detected as being added or removed
    JoystickUpdate {
        removed: bool,
        name: String,
    },
    /// Frontend -> Backend
    /// Informs the backend of a change to the joystick mappings,
    /// that is how the joysticks are mapped to the ports readable by robot code
    UpdateJoystickMapping {
        name: String,
        pos: usize
    },
    /// Backend -> Frontend
    /// A periodic message that updates the telemetry badges and voltage display in the UI
    RobotStateUpdate {
        /// Whether the driver station is currently connected to a robot
        comms_alive: bool,
        /// Whether the user code is running
        code_alive: bool,
        /// Whether there are any input devices attached to the computer
        joysticks: bool,
        /// Whether the DS is in Simulator mode, where it connects to the WPILib simulator instead of a robot
        simulator: bool,
        /// The reported voltage of the robot
        voltage: f32,
    },
    /// Backend -> Frontend
    /// A message sent when new stdout data is received from the robot
    NewStdout {
        message: String,
    },
    /// Frontend -> Backend
    /// Updates the alliance station
    UpdateAllianceStation { station: AllianceStation },
    /// Frontend -> Backend
    /// Informs the backend of a specific request the user selected in the Config menu, restarting code or rebooting the roboRIO
    Request { req: Request, },
    /// Frontend <-> Backend
    /// Informs the backend that the user wishes to emergency stop the robot
    /// iff the backend is handling keybinds, this message will be sent to the frontend to notify the UI that the robot is estopped.
    EstopRobot {
        from_backend: bool
    },
    /// Frontend -> Backend
    /// Queries the backends estop status, used when switching connection targets
    /// to update the frontend if the RIO is estop locked when connected
    QueryEstop,
    /// Backend -> Frontend
    /// Response to the QueryEstop message
    RobotEstopStatus {
        estopped: bool
    }
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