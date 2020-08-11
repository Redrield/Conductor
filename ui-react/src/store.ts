import {
    AllianceColour,
    AllianceStation,
    CAPABILITIES,
    ESTOP_ROBOT,
    JOYSTICK_UPDATE,
    Message,
    Mode,
    NEW_STDOUT,
    REQUEST,
    ROBOT_STATE_UPDATE,
    RobotState,
    UPDATE_ALLIANCE_STATION,
    UPDATE_ENABLE_STATUS,
    UPDATE_GSM,
    UPDATE_JOYSTICK_MAPPING,
    UPDATE_MODE,
    UPDATE_TEAM_NUMBER,
    UPDATE_USB_STATUS
} from "./ipc";

export enum ActivePage {
    Control,
    Config,
    Joysticks
}

export enum ErrorExplanation {
    Comms,
    Code,
    Joysticks
}

export interface DriverStationState {
    teamNumber: string;
    connectUSB: boolean;
    gsm: string;
    enabled: boolean;
    estopped: boolean;
    mode: Mode;
    alliance: AllianceStation;
    robotState: RobotState;
    ws: WebSocket | null;
    activePage: ActivePage;
    stdout: string[];
    explanation: ErrorExplanation | null;
    joysticks: string[];
    joystickMappings: { [id: number]: string };
    backendKeybinds: boolean;
    warningAcknowledged: boolean;
}

export function initState(): DriverStationState {
    return {
        teamNumber: "",
        connectUSB: false,
        gsm: "",
        enabled: false,
        estopped: false,
        mode: Mode.Autonomous,
        alliance: {
            color: AllianceColour.Red,
            value: 1
        },
        robotState: {
            commsAlive: false,
            codeAlive: false,
            joysticksConnected: false,
            voltage: 0.0
        },
        ws: null,
        activePage: ActivePage.Control,
        stdout: [],
        explanation: null,
        joysticks: [],
        joystickMappings: {},
        backendKeybinds: false,
        warningAcknowledged: true,
    }
}

export const SOCKET_CONNECTED = "SocketConnected";
export interface SocketConnected {
    type: typeof SOCKET_CONNECTED;
    ws: WebSocket;
}

export const CHANGE_PAGE = "ChangePage";
export interface ChangePage {
    type: typeof CHANGE_PAGE;
    page: ActivePage;
}

export const TEAM_NUMBER_CHANGE = "TeamNumberChange";
export interface TeamNumberChange {
    type: typeof TEAM_NUMBER_CHANGE;
    teamNumber: string;
}

export const GSM_CHANGE = "GSMChange";
export interface GSMChange {
    type: typeof GSM_CHANGE;
    gsm: string;
}

export const EXPLANATION_CHANGE = "ExplanationChange";
export interface ExplanationChange {
    type: typeof EXPLANATION_CHANGE,
    explanation: ErrorExplanation | null;
}

export const ACKNOWLEDGE_WARNING = "AcknowledgeWarning";
export interface AcknowledgeWarning {
    type: typeof ACKNOWLEDGE_WARNING
}

export type AppAction = Message | SocketConnected | ChangePage | TeamNumberChange | GSMChange | ExplanationChange | AcknowledgeWarning;

export function rootReducer(state: DriverStationState, action: AppAction): DriverStationState {
    switch(action.type) {
        case JOYSTICK_UPDATE:
            if(!action.removed) {
                return {
                    ...state,
                    joysticks: [...state.joysticks, action.name]
                }
            } else {
                let newMappings: { [id: number]: string } = {};
                for(let key in state.joystickMappings) {
                    let name = state.joystickMappings[key];
                    if(name != action.name) {
                        newMappings[key] = name;
                    }
                }
                return {
                    ...state,
                    joysticks: state.joysticks.filter(name => name != action.name),
                    joystickMappings: newMappings
                }
            }
        case ROBOT_STATE_UPDATE:
            return {
                ...state,
                robotState: {
                    commsAlive: action.comms_alive,
                    codeAlive: action.code_alive,
                    joysticksConnected: action.joysticks,
                    voltage: action.voltage
                }
            }
        case NEW_STDOUT:
            return {
                ...state,
                stdout: [...state.stdout, action.message]
            };
        case SOCKET_CONNECTED:
            return {
                ...state,
                ws: action.ws
            }
        case CHANGE_PAGE:
            return {
                ...state,
                activePage: action.page
            }
        case UPDATE_GSM:
            dispatchSocketMessage(state.ws, action);
            return state;
        case UPDATE_TEAM_NUMBER:
            dispatchSocketMessage(state.ws, action);
            return state;
        case UPDATE_USB_STATUS:
            dispatchSocketMessage(state.ws, action);
            return {
                ...state,
                connectUSB: action.use_usb
            }
        case UPDATE_MODE:
            dispatchSocketMessage(state.ws, action);
            return {
                ...state,
                mode: action.mode
            }
        case UPDATE_ENABLE_STATUS:
            if(!action.from_backend) {
                dispatchSocketMessage(state.ws, action);
            }
            return {
                ...state,
                enabled: action.enabled
            }
        case UPDATE_JOYSTICK_MAPPING:
            dispatchSocketMessage(state.ws, action);
            return {
                ...state,
                joystickMappings: {
                    ...state.joystickMappings,
                    [action.pos]: action.name
                }
            }
        case UPDATE_ALLIANCE_STATION:
            dispatchSocketMessage(state.ws, action);
            return {
                ...state,
                alliance: action.station
            }
        case REQUEST:
            dispatchSocketMessage(state.ws, action);
            return state;
        case ESTOP_ROBOT:
            if(!action.from_backend) {
                dispatchSocketMessage(state.ws, action);
            }
            return {
                ...state,
                estopped: true
            };
        case TEAM_NUMBER_CHANGE:
            return {
                ...state,
                teamNumber: action.teamNumber
            }
        case GSM_CHANGE:
            return {
                ...state,
                gsm: action.gsm
            }
        case EXPLANATION_CHANGE:
            return {
                ...state,
                explanation: action.explanation
            }
        case CAPABILITIES:
            if(action.backend_keybinds) {
                return {
                    ...state,
                    backendKeybinds: action.backend_keybinds,
                };
            } else {
                return {
                    ...state,
                    backendKeybinds: action.backend_keybinds,
                    warningAcknowledged: false
                };
            }
        case ACKNOWLEDGE_WARNING:
            return {
                ...state,
                warningAcknowledged: true
            }
        default:
            return state;
    }
}

// ReDuCeRs ShOuLdN't HaVe SiDe EfFeCtS
// well I don't feel like adding another 2gb of new libraries to learn
function dispatchSocketMessage(ws: WebSocket | null, msg: Message) {
    if(ws != null) {
        ws.send(JSON.stringify(msg));
    }
}