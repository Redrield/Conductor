import {Message, NEW_STDOUT, UPDATE_ENABLE_STATUS} from "./ipc";

export interface State {
    stdout: string[];
    enabled: boolean;
    ws: WebSocket | null;
}

export function initState() {
    return {
        stdout: [],
        enabled: false,
        ws: null
    }
}

export const SOCKET_CONNECTED = "SocketConnected";
export interface SocketConnected {
    type: typeof SOCKET_CONNECTED;
    ws: WebSocket;
}

export const STUPID_INITIALIZER = "StupidInitializer";
export interface StupidInitializer {
    type: typeof STUPID_INITIALIZER
}

export type AppAction = SocketConnected | StupidInitializer | Message;

export function reducer(state: State, action: AppAction): State {
    switch(action.type) {
        case SOCKET_CONNECTED:
            return {
                ...state,
                ws: action.ws
            }
        case UPDATE_ENABLE_STATUS:
            return {
                ...state,
                enabled: action.enabled
            }
        case NEW_STDOUT:
            return {
                ...state,
                stdout: [...state.stdout, action.message]
            }
        case STUPID_INITIALIZER:
            return initState();
    }
}