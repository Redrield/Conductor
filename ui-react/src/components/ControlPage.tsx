import {DriverStationState} from "../store";
import {AllianceStation, Mode, RobotState, UPDATE_ALLIANCE_STATION, UPDATE_ENABLE_STATUS, UPDATE_MODE} from "../ipc";
import {connect, ConnectedProps} from "react-redux";
import React from "react";
import ModeList from "./control/ModeList";
import TelemetryList from "./control/TelemetryList";
import StdoutView from "./control/StdoutView";
import EnableButtons from "./control/EnableButtons";
import AllianceStationSelector from "./control/AllianceStationSelector";

const mapState = (state: DriverStationState) => ({
    mode: state.mode,
    enabled: state.enabled,
    estopped: state.estopped,
    alliance: state.alliance,
    robotState: state.robotState,
    teamNumber: state.teamNumber
});

const mapDispatch = {
    updateMode: (mode: Mode) => ({ type: UPDATE_MODE, mode: mode }),
    updateEnabled: (enabled: boolean) => ({ type: UPDATE_ENABLE_STATUS, enabled: enabled }),
    updateAlliance: (station: AllianceStation) => ({ type: UPDATE_ALLIANCE_STATION, station: station })
};

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

function classFromVoltage(voltage: number): string {
    if(voltage >= 8.5 && voltage <= 11.5) {
        return "text-warning"
    } else if(voltage < 8.5)  {
        return "text-danger"
    } else {
        return "text-success"
    }
}

function robotStatus(state: Props) {
    if(state.estopped) {
        return "Emergency Stopped"
    } else if(state.robotState.codeAlive) {
        if(state.enabled) {
            return state.mode.toString() + "\nEnabled"
        } else {
            return state.mode.toString() + "\nDisabled"
        }
    } else if(!state.robotState.codeAlive && state.robotState.commsAlive) {
        return "No Robot Code"
    } else {
        return "No Robot Communication"
    }
}

const ControlPage = (props: Props) => (
    <div className="container-fluid">
        <div className="row">
            <div className="col-3 mt-4">
                <ModeList />
            </div>

            <div className="col-3">
                <TelemetryList />
            </div>

            <div className="col-2">
                {/* I don't know why I have to specify the font weight now, but I do */}
                <p className="lead mt-3">
                    Team # {props.teamNumber}
                </p>
                <p className={`text-center mt-4 ${classFromVoltage(props.robotState.voltage)}`}>
                    <b>{+props.robotState.voltage.toFixed(2)}V</b>
                </p>
            </div>
            <div className="col">
                <StdoutView />
            </div>
        </div>
        <div className="row" style={{marginTop: "-50px"}}>
            <div className="col-3 text-center mt-4">
                <EnableButtons />
            </div>
            <div className="col-3 mt-4">
                <AllianceStationSelector />
            </div>
            <div className="col-2 align-items-center">
                <p className="text-center lead">{robotStatus(props)}</p>
            </div>
            <div className="col" />
        </div>
    </div>
)

export default connector(ControlPage)