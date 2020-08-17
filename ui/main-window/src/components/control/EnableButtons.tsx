import {DriverStationState} from "../../store";
import {UPDATE_ENABLE_STATUS} from "../../ipc";
import {connect, ConnectedProps} from "react-redux";
import React from "react";

const mapState = (state: DriverStationState) => ({
    enabled: state.enabled
});

const mapDispatch = {
    updateEnabled: (enabled: boolean) => ({type: UPDATE_ENABLE_STATUS, enabled: enabled, from_backend: false})
};

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

function buttonClick(ev: React.MouseEvent<HTMLButtonElement>, enable: boolean, props: Props) {
    ev.currentTarget.blur();
    props.updateEnabled(enable);
}

const EnableButtons = (props: Props) => (
    <div className="btn-group" role="group" aria-label="State Control Buttons">
        <button id="enableButton" type="button" className={`btn btn-lg btn-secondary ${props.enabled ? "active" : ""}`}
                onClick={(ev) => buttonClick(ev, true, props)} style={{color: "#11CD9D"}}>
            <b>Enable</b>
        </button>
        <button id="disableButton" type="button" className={`btn btn-lg btn-secondary ${!props.enabled ? "active" : ""}`}
                onClick={(ev) => buttonClick(ev, false, props)} style={{color: "#F85D4D"}}>
            <b>Disable</b>
        </button>
    </div>
)

export default connector(EnableButtons)