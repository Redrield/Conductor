import {DriverStationState} from "../store";
import {UPDATE_JOYSTICK_MAPPING} from "../ipc";
import {connect, ConnectedProps} from "react-redux";
import React from "react";

const mapState = (state: DriverStationState) => ({
    joysticks: state.joysticks,
    mappings: state.joystickMappings
})

const mapDispatch = {
    updateMapping: (n: number, name: string) => ({type: UPDATE_JOYSTICK_MAPPING, name: name, pos: n})
}

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

function joystickItem(n: number, props: Props) {
    let mapping = props.mappings[n];
    return (
        <li className="list-group-item">
            <div className="input-group">
                <div className="input-group-prepend">
                    <span className="input-group-text">{n}: </span>
                </div>
                <div className="dropdown">
                    <button className="btn btn-secondary dropdown-toggle" type="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                        {mapping != null ? mapping : "Controller"}
                    </button>
                    <div className="dropdown-menu">
                        {props.joysticks.map(name => (
                            <a className="dropdown-item" href="#" onClick={(_) => props.updateMapping(n, name)}>{name}</a>
                        ))}
                    </div>
                </div>
            </div>
        </li>
    )
}

function joysticks(start: number, end: number, props: Props) {
    let body = []

    for(let i = start; i < end; i++) {
        body.push(joystickItem(i, props))
    }
    return body
}

const JoysticksPage = (props: Props) => (
    <div className="container">
        <div className="row">
            <div className="col">
                <ul className="list-group">
                    {joysticks(0, 3, props)}
                </ul>
            </div>

            <div className="col">
                <ul className="list-group">
                    {joysticks(3, 6, props)}
                </ul>
            </div>
        </div>
    </div>
)

export default connector(JoysticksPage)