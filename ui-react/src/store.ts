import {
    AllianceColour,
    AllianceStation, ESTOP_ROBOT,
    JOYSTICK_UPDATE,
    Message,
    Mode,
    NEW_STDOUT, REQUEST,
    ROBOT_STATE_UPDATE,
    RobotState, UPDATE_ALLIANCE_STATION,
    UPDATE_ENABLE_STATUS,
    UPDATE_GSM,
    UPDATE_JOYSTICK_MAPPING,
    UPDATE_MODE,
    UPDATE_TEAM_NUMBER,
    UPDATE_USB_STATUS
} from "./ipc";
// import {combineReducers} from "redux";
// import {frontendReducer} from "./reducers/frontend";
// import {backendReducer} from "./reducers/backend";

export enum ActivePage {
    Control,
    Config,
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
    stdout: string[]
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

export type AppAction = Message | SocketConnected | ChangePage | TeamNumberChange | GSMChange;

export function rootReducer(state: DriverStationState, action: AppAction): DriverStationState {
    switch(action.type) {
        case JOYSTICK_UPDATE:
            //TODO: Joysticks not implemented
            return state;
        case ROBOT_STATE_UPDATE:
            // console.log("Robot state updated " + JSON.stringify(action));
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
            dispatchSocketMessage(state.ws, action);
            return {
                ...state,
                enabled: action.enabled
            }
        case UPDATE_JOYSTICK_MAPPING:
            dispatchSocketMessage(state.ws, action);
            // TODO: Joysticks not implemented
            return state;
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
            dispatchSocketMessage(state.ws, action);
            return state;
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