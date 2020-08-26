import {
    AllianceColour,
    AllianceStation,
    CAPABILITIES,
    ESTOP_ROBOT,
    ESTOP_STATUS,
    JOYSTICK_UPDATE,
    Message,
    Mode,
    NEW_STDOUT,
    QUERY_ESTOP,
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
import {JoystickData} from "./components/joysticks/Joystick";
import {v4 as newV4Uuid} from "uuid";

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
    joysticks: JoystickData[];
    joystickMappings: { [id: number]: string };
    backendKeybinds: boolean;
    warningAcknowledged: boolean;
}

function makeJoysticks() {
    let arr = new Array(6);
    for(let i = 0; i < 6; i++) {
        arr[i] = {name: "Unbound", id: newV4Uuid()};
    }
    return arr;
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
            simulatorConnected: false,
            voltage: 0.0
        },
        ws: null,
        activePage: ActivePage.Control,
        stdout: [],
        explanation: null,
        joysticks: makeJoysticks(),
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

export const REORDER_JOYSTICKS = "ReorderJoysticks";
export interface ReorderJoysticks {
    type: typeof REORDER_JOYSTICKS;
    js: JoystickData;
    oldIdx: number;
    newIdx: number;
}

export const UPDATE_JOYSTICK_MAPPING_INTERNAL = "__UpdateJoystickMapping";
export interface UpdateJoystickMappingInternal {
    type: typeof UPDATE_JOYSTICK_MAPPING_INTERNAL;
    name: string;
    pos: number;
    uuid: string;
}

export type AppAction =
    Message
    | SocketConnected
    | ChangePage
    | TeamNumberChange
    | GSMChange
    | ExplanationChange
    | AcknowledgeWarning
    | ReorderJoysticks
    | UpdateJoystickMappingInternal;

function reorder(items: JoystickData[], start: number, end: number) {
    let result = Array.from(items);
    let [removed] = result.splice(start, 1);
    result.splice(end, 0, removed);

    return result;
}

function clearSpace(items: JoystickData[]) {
    let result = Array.from(items);
    for(let i = 0; i < 6; i++) {
        if(items[i].name == "Unbound") {
            result.splice(i, 1);
            return result;
        }
    }
    return result;
}

function replaceRemoved(items: JoystickData[], removedName: string) {
    let result = Array.from(items);

    let idx = result.findIndex(item => item.name == removedName);
    result.splice(idx, 1, {name:"Unbound", id:newV4Uuid()});
    return result;
}

export function rootReducer(state: DriverStationState, action: AppAction): DriverStationState {
    switch (action.type) {
        case JOYSTICK_UPDATE:
            if (!action.removed) {
                let cleared = clearSpace(state.joysticks);
                // Put the virtual joystick at the front, it's only added at startup and is mapped to port 0 by default
                // For any other joystick, putting it at the front would mean sending multiple mapping updates for every other device
                if(action.name == "Virtual Joystick") {
                    return {
                        ...state,
                        joysticks: [{name: action.name, id: action.uuid}, ...cleared]
                    }
                }else {
                    return {
                        ...state,
                        joysticks: [...cleared, {name: action.name, id: action.uuid}]
                    }
                }
            } else {
                let newMappings: { [id: number]: string } = {};
                for (let key in state.joystickMappings) {
                    let name = state.joystickMappings[key];
                    if (name != action.name) {
                        newMappings[key] = name;
                    }
                }

                let joysticks = replaceRemoved(state.joysticks, action.name);
                return {
                    ...state,
                    joysticks: joysticks,
                    joystickMappings: newMappings
                }
            }
        case ROBOT_STATE_UPDATE:
            if (state.estopped && !action.code_alive) {
                return {
                    ...state,
                    estopped: false,
                    robotState: {
                        commsAlive: action.comms_alive,
                        codeAlive: action.code_alive,
                        joysticksConnected: action.joysticks,
                        voltage: action.voltage,
                        simulatorConnected: action.simulator,
                    }
                }
            } else {
                return {
                    ...state,
                    robotState: {
                        commsAlive: action.comms_alive,
                        codeAlive: action.code_alive,
                        joysticksConnected: action.joysticks,
                        voltage: action.voltage,
                        simulatorConnected: action.simulator
                    }
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
            if (!action.from_backend) {
                dispatchSocketMessage(state.ws, action);
            }
            return {
                ...state,
                enabled: action.enabled
            }
        case UPDATE_JOYSTICK_MAPPING_INTERNAL:
            if(action.name == "Unbound") {
                return state; // backend doesn't need to know about those
            }
            dispatchSocketMessage(state.ws, {type: UPDATE_JOYSTICK_MAPPING, pos: action.pos, uuid: action.uuid});
            return {
                ...state,
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
            if (!action.from_backend) {
                dispatchSocketMessage(state.ws, action);
            }
            return {
                ...state,
                enabled: false,
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
            if (action.backend_keybinds) {
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
        case QUERY_ESTOP:
            dispatchSocketMessage(state.ws, action);
            return state;
        case ESTOP_STATUS:
            return {
                ...state,
                estopped: action.estopped
            }
        case REORDER_JOYSTICKS:
            return {
                ...state,
                joysticks: reorder(state.joysticks, action.oldIdx, action.newIdx)
            }
        default:
            return state;
    }
}

// ReDuCeRs ShOuLdN't HaVe SiDe EfFeCtS
// well I don't feel like adding another 2gb of new libraries to learn
function dispatchSocketMessage(ws: WebSocket | null, msg: Message) {
    if (ws != null) {
        ws.send(JSON.stringify(msg));
    }
}