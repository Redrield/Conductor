import {DriverStationState} from "../store";
import {RequestType, REQUEST} from "../ipc";
import {connect, ConnectedProps} from "react-redux";
import React from "react";
import Configurations from "./config/Configurations";

const mapState = (_: DriverStationState) => ({});

const mapDispatch = {
    dispatchRequest: (req: RequestType) => ({type: REQUEST, req: req})
}

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

const ConfigPage = (props: Props) => (
    <div className="container">
        <div className="row align-items-center">
            <Configurations/>

            <div className="col"/>
            <div className="col pull-right">
                <div className="btn-group-vertical">
                    <button type="button" className="btn btn-secondary"
                            onClick={(_) => props.dispatchRequest(RequestType.RestartRoborio)}>Restart roboRIO
                    </button>
                    <button type="button" className="btn btn-secondary"
                            onClick={(_) => props.dispatchRequest(RequestType.RestartCode)}>Restart Robot Code
                    </button>
                </div>
            </div>
        </div>
    </div>
)

export default connector(ConfigPage);