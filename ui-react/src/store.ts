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
    teamNumber: number;
    connectUSB: boolean;
    gsm: string;
    enabled: boolean;
    estopped: boolean;
    mode: Mode;
    alliance: AllianceStation;
    robotState: RobotState;
    ws: WebSocket | null;
    activePage: ActivePage;
}

export function initState(): DriverStationState {
    return {
        teamNumber: 4069,
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
        activePage: ActivePage.Control
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

export type AppAction = Message | SocketConnected | ChangePage;

export function rootReducer(state: DriverStationState, action: AppAction): DriverStationState {
    switch(action.type) {
        case JOYSTICK_UPDATE:
            //TODO: Joysticks not implemented
            return state;
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
            //TODO: stdout not implemented
            return state;
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
            return {
                ...state,
                gsm: action.gsm
            }
        case UPDATE_TEAM_NUMBER:
            dispatchSocketMessage(state.ws, action);
            return {
                ...state,
                teamNumber: action.team_number
            }
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