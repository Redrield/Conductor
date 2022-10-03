import {DriverStationState, GSM_CHANGE, TEAM_NUMBER_CHANGE} from "../../store";
import {QUERY_ESTOP, UPDATE_GSM, UPDATE_TEAM_NUMBER, UPDATE_USB_STATUS} from "../../ipc";
import React, {ChangeEvent, FormEvent} from "react";
import {connect, ConnectedProps} from "react-redux";


const mapState = (state: DriverStationState) => ({
    teamNumber: state.teamNumber,
    useUSB: state.connectUSB,
    gsm: state.gsm
});

const mapDispatch = {
    updateTeamNumber: (tn: number) => ({type: UPDATE_TEAM_NUMBER, team_number: tn, from_backend: false}),
    changeTeamNumber: (tn: string) => ({type: TEAM_NUMBER_CHANGE, teamNumber: tn}),
    updateUSB: (useUSB: boolean) => ({type: UPDATE_USB_STATUS, use_usb: useUSB}),
    updateGSM: (gsm: string) => ({type: UPDATE_GSM, gsm: gsm}),
    changeGSM: (gsm: string) => ({type: GSM_CHANGE, gsm: gsm}),
    queryEstop: () => ({type: QUERY_ESTOP})
}

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

function onTeamNumberChange(props: Props) {
    return (ev: FormEvent<HTMLInputElement>) => {
        let team = ev.currentTarget.value;
        if (team.length <= 4) {
            props.changeTeamNumber(team);
        }
    }
}

function onGSMChange(props: Props) {
    return (ev: FormEvent<HTMLInputElement>) => {
        let gsm = ev.currentTarget.value;
        if (gsm.length <= 3) {
            props.changeGSM(gsm);
        }
    }
}

function onKeyDownHandler(props: Props, type: string) {
    if (type == UPDATE_TEAM_NUMBER) {
        return (ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key == "Enter") {
                console.log("Updating team number");
                props.updateTeamNumber(+props.teamNumber);
                setTimeout(() => props.queryEstop(), 100);
            }
        }
    } else {
        return (ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key == "Enter") {
                props.updateGSM(props.gsm);
            }
        }
    }
}

const Configurations = (props: Props) => (//TODO: Add hidden notification near team number change dialog to be displayed if an invalid entry is reported by the backend.
    <div className="col">
        <label htmlFor="teamNumberInput">Team Number</label>
        <div className="input-group mb-3">
            <input type="number" className="form-control" id="teamNumberInput" value={props.teamNumber}
                   onInput={onTeamNumberChange(props)}
                   onKeyDown={onKeyDownHandler(props, UPDATE_TEAM_NUMBER)}/>
        </div>
        <label htmlFor="useUSBCheckbox">Connect via USB?</label>
        <div className="input-group mb-3">
            <input type="checkbox" className="form-control" id="useUSBCheckbox" checked={props.useUSB}
                   onChange={(change: ChangeEvent<HTMLInputElement>) => props.updateUSB(change.currentTarget.checked)}/>
        </div>
        <label htmlFor="gameDataInput">Game Data</label>
        <div className="input-group mb-3">
            <input type="text" className="form-control disabled" id="gameDataInput" value={props.gsm}
                   onInput={onGSMChange(props)}
                   onKeyDown={onKeyDownHandler(props, UPDATE_GSM)}/>
        </div>
    </div>
);

export default connector(Configurations);
