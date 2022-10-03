
// :bolbaaaa:
export enum Mode {
    Autonomous= "Autonomous",
    Teleoperated = "Teleoperated",
    Test = "Test"
}

export enum AllianceColour {
    Red = "Red",
    Blue = "Blue"
}

export enum RequestType {
    RestartRoborio = "RestartRoborio",
    RestartCode = "RestartCode"
}

export interface AllianceStation {
    color: AllianceColour;
    value: number;
}

export interface RobotState {
    commsAlive: boolean;
    codeAlive: boolean;
    voltage: number;
    joysticksConnected: boolean;
    simulatorConnected: boolean;
}

export function initRobotState() {
    return {
        commsAlive: false,
        codeAlive: false,
        voltage: 0.0,
        joysticksConnected: false
    }
}

// I miss ADTs

export const UPDATE_GSM = "UpdateGSM";
export interface UpdateGSM {
    type: typeof UPDATE_GSM;
    gsm: string;
}

export const UPDATE_TEAM_NUMBER = "UpdateTeamNumber";
export interface UpdateTeamNumber {
    type: typeof UPDATE_TEAM_NUMBER;
    team_number: number;
    from_backend: boolean;
}

export const UPDATE_USB_STATUS = "UpdateUSBStatus";
export interface UpdateUSBStatus {
    type: typeof UPDATE_USB_STATUS;
    use_usb: boolean;
}

export const UPDATE_MODE = "UpdateMode";
export interface UpdateMode {
    type: typeof UPDATE_MODE;
    mode: Mode;
}

export const UPDATE_ENABLE_STATUS = "UpdateEnableStatus";
export interface UpdateEnableStatus {
    type: typeof UPDATE_ENABLE_STATUS;
    from_backend: boolean;
    enabled: boolean;
}

export const JOYSTICK_UPDATE = "JoystickUpdate";
export interface JoystickUpdate {
    type: typeof JOYSTICK_UPDATE;
    removed: boolean;
    name: string;
    uuid: string;
}

export const UPDATE_JOYSTICK_MAPPING = "UpdateJoystickMapping";
export interface UpdateJoystickMapping {
    type: typeof UPDATE_JOYSTICK_MAPPING;
    uuid: string;
    pos: number;
}

export const ROBOT_STATE_UPDATE = "RobotStateUpdate";
export interface RobotStateUpdate {
    type: typeof ROBOT_STATE_UPDATE;
    comms_alive: boolean;
    code_alive: boolean;
    joysticks: boolean;
    simulator: boolean;
    voltage: number;
}

export const NEW_STDOUT = "NewStdout";
export interface NewStdout {
    type: typeof NEW_STDOUT;
    message: string;
}

export const UPDATE_ALLIANCE_STATION = "UpdateAllianceStation";
export interface UpdateAllianceStation {
    type: typeof UPDATE_ALLIANCE_STATION;
    station: AllianceStation
}

export const REQUEST = "Request";
export interface Request {
    type: typeof REQUEST
    req: RequestType
}

export const ESTOP_ROBOT = "EstopRobot";
export interface EstopRobot {
    type: typeof ESTOP_ROBOT;
    from_backend: boolean;
}

export const CAPABILITIES = "Capabilities";
export interface Capabilities {
    type: typeof CAPABILITIES;
    backend_keybinds: boolean;
}

export const QUERY_ESTOP = "QueryEstop";
export interface QueryEstop {
    type: typeof QUERY_ESTOP
}

export const ESTOP_STATUS = "RobotEstopStatus";
export interface RobotEstopStatus {
    type: typeof ESTOP_STATUS
    estopped: boolean;
}

export const VALUE_ERROR = "ValueError";
export interface ValueError {
    type: typeof VALUE_ERROR;
    error_message: string;
    instigator: string;
}

export type Message = UpdateGSM | UpdateTeamNumber | UpdateUSBStatus | UpdateMode
| UpdateEnableStatus | JoystickUpdate | UpdateJoystickMapping | RobotStateUpdate | ValueError
| NewStdout | UpdateAllianceStation | Request | EstopRobot | Capabilities | QueryEstop | RobotEstopStatus;