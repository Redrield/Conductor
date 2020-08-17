import {DriverStationState} from "../../store";
import {Mode, UPDATE_MODE} from "../../ipc";
import {connect, ConnectedProps} from "react-redux";
import React from "react";
import './ModeList.css'

const mapState = (state: DriverStationState) => ({
    mode: state.mode
});

const mapDispatch = {
    updateMode: (mode: Mode) => ({type: UPDATE_MODE, mode: mode})
};

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

const ModeList = (props: Props) => (
    <div className="list-group">
        {modeItem(props, Mode.Autonomous)}
        {modeItem(props, Mode.Teleoperated)}
        {modeItem(props, Mode.Test)}
    </div>
)

function modeItem(props: Props, mode: Mode) {
    return (
        <a href="#" className={`list-group-item list-group-item-action py-1 ${props.mode == mode ? "active" : ""}`}
           onClick={() => props.updateMode(mode)}>{mode.toString()}</a>
    )
}

export default connector(ModeList)