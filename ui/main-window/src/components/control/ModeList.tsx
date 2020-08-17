import {DriverStationState} from "../../store";
import {Mode, UPDATE_MODE} from "../../ipc";
import {connect, ConnectedProps} from "react-redux";
import React from "react";

const mapState = (state: DriverStationState) => ({
    mode: state.mode
});

const mapDispatch = {
    updateMode: (mode: Mode) => ({type: UPDATE_MODE, mode: mode})
};

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

const ModeList = (props: Props) => (
    <div className="btn-group-vertical">
        {modeItem(props, Mode.Autonomous)}
        {modeItem(props, Mode.Teleoperated)}
        {modeItem(props, Mode.Test)}
    </div>
)

function action(ev: React.MouseEvent<HTMLButtonElement>, props: Props, mode: Mode) {
    ev.currentTarget.blur();
    props.updateMode(mode);
}

function modeItem(props: Props, mode: Mode) {
    return (
        <button type="button" className={`btn btn-secondary btn-block border border-dark text-left ${props.mode == mode ? "active" : ""}`}
                onClick={(ev) => action(ev, props, mode)}>
            {mode.toString()}
        </button>
    )
}

export default connector(ModeList)