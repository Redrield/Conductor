
export const NEW_STDOUT = "NewStdout";
export interface NewStdout {
    type: typeof NEW_STDOUT;
    message: string;
}

export const UPDATE_ENABLE_STATUS = "UpdateEnableStatus";
export interface UpdateEnableStatus {
    type: typeof UPDATE_ENABLE_STATUS;
    from_backend: boolean;
    enabled: boolean;
}

export type Message = NewStdout | UpdateEnableStatus;