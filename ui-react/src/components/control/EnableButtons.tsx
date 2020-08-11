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

const EnableButtons = (props: Props) => (
    <div className="btn-group" role="group" aria-label="State Control Buttons">
        <button id="enableButton" type="button" className={`btn btn-lg btn-success ${props.enabled ? "active" : ""}`}
                onClick={(_) => props.updateEnabled(true)}>Enable</button>
        <button id="disableButton" type="button" className={`btn btn-lg btn-danger ${!props.enabled ? "active" : ""}`}
                onClick={(_) => props.updateEnabled(false)}>Disable</button>
    </div>
)

export default connector(EnableButtons)