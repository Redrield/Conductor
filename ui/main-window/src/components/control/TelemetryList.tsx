import {DriverStationState, ErrorExplanation, EXPLANATION_CHANGE} from "../../store";
import {connect, ConnectedProps} from "react-redux";
import React from "react";

const mapState = (state: DriverStationState) => ({
    robotState: state.robotState
});

const mapDispatch = {
    updateExplanation: (explanation: ErrorExplanation | null) => ({type: EXPLANATION_CHANGE, explanation: explanation})
}

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

const TelemetryList = (props: Props) => (
    <ul className="list-group mt-4">
        {telemetryBadge(props, "Communications", props.robotState.commsAlive, ErrorExplanation.Comms)}
        {telemetryBadge(props, "Robot Code", props.robotState.codeAlive, ErrorExplanation.Code)}
        {telemetryBadge(props, "Joysticks", props.robotState.joysticksConnected, ErrorExplanation.Joysticks)}
    </ul>
)

export const successStyle = {
    color: "#00BC8C"
}

export const failStyle = {
    color: "#E74C3C"
}

function telemetryBadge(props: Props, name: string, alive: boolean, type: ErrorExplanation) {
    let badge;
    if (alive) {
        badge = (
            <span className="badge badge-success" style={successStyle}>AA</span>
        )
    } else {
        badge = (
            <span className="badge badge-danger" style={failStyle}
                  onMouseEnter={(_) => props.updateExplanation(type)}
                  onMouseLeave={(_) => props.updateExplanation(null)}>AA</span>
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