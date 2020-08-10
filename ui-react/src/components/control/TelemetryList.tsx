import {DriverStationState} from "../../store";
import {connect, ConnectedProps} from "react-redux";
import React from "react";

const mapState = (state: DriverStationState) => ({
    robotState: state.robotState
});

const mapDispatch = {}

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

const TelemetryList = (props: Props) => (
    <ul className="list-group mt-4">
        {telemetryBadge("Communications", props.robotState.commsAlive)}
        {telemetryBadge("Robot Code", props.robotState.codeAlive)}
        {telemetryBadge("Joysticks", props.robotState.joysticksConnected)}
    </ul>
)

const successStyle = {
    color: "#00BC8C"
}

const failStyle = {
    color: "#E74C3C"
}

function telemetryBadge(name: string, alive: boolean) {
    let badge;
    if (alive) {
        badge = (
            <span className="badge badge-success" style={successStyle}>AA</span>
        )
    } else {
        badge = (
            <span className="badge badge-danger" style={failStyle}>AA</span>
        )
    }
    return (
        <li className="list-group-item d-flex justify-content-between align-items-center py-2">
            {name}
            {badge}
        </li>
    )
}

export default connector(TelemetryList)