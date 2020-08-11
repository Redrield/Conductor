import React from 'react'
import {AllianceStation, AllianceColour, initRobotState, Mode, RobotState, UpdateGSM, Message} from './ipc'
import {ACKNOWLEDGE_WARNING, ActivePage, CHANGE_PAGE, DriverStationState, SOCKET_CONNECTED} from "./store";
import {connect, ConnectedProps} from "react-redux";
import ControlPage from "./components/ControlPage";
import ConfigPage from "./components/ConfigPage";
import JoysticksPage from "./components/JoysticksPage";

const mapState = (state: DriverStationState) => ({
    activePage: state.activePage,
    capabilities: state.backendKeybinds,
    warningAck: state.warningAcknowledged
});

const mapDispatch = {
    socketConnected: (ws: WebSocket) => ({type: SOCKET_CONNECTED, ws: ws}),
    socketMessage: (msg: Message) => (msg),
    changePage: (page: ActivePage) => ({type: CHANGE_PAGE, page: page}),
    ackWarning: () => ({type: ACKNOWLEDGE_WARNING})
};

const connector = connect(mapState, mapDispatch);

type ReduxProps = ConnectedProps<typeof connector>;

type Props = ReduxProps & {
    webserverPort: number;
}

class DriverStation extends React.Component<Props, any> {
    constructor(props: Props) {
        super(props);

        this.connectWs(props.webserverPort);
    }

    render() {
        if (!this.props.capabilities && !this.props.warningAck) {
            const containerStyle = {
                marginTop: "50px"
            };
            return (
                <div className="container" style={containerStyle}>
                    <div className="row align-content-center">
                        <div className="col align-content-center">
                            <div className="card">
                                <div className="card-body">
                                    <h5 className="card-title text-danger font-weight-bold">Warning: Global Hotkeys Unavailable</h5>
                                    <p className="card-text text-danger">
                                        This version of the driver station can't accept disable or estop commands if the
                                        window is out of focus. The window must be focused for Enter and Space to work.
                                    </p>
                                    <a href="#" className="btn btn-lg btn-danger"
                                       onClick={() => this.props.ackWarning()}>Continue</a>
                                </div>
                            </div>
                            {/*<div className="modal">*/}
                            {/*    <div className="modal-dialog">*/}
                            {/*        <div className="modal-content">*/}
                            {/*            <div className="modal-header">*/}
                            {/*                <h5 className="modal-title text-danger font-weight-bold">Warning: Disable Keybinds</h5>*/}
                            {/*            </div>*/}
                            {/*            <div className="modal-body">*/}
                            {/*                </p>*/}
                            {/*            </div>*/}
                            {/*            <div className="modal-footer">*/}
                            {/*                <button className="btn btn-lg btn-danger" onClick={() => this.props.ackWarning()}>Continue</button>*/}
                            {/*            </div>*/}
                            {/*        </div>*/}
                            {/*    </div>*/}
                        </div>
                    </div>
                </div>
            )
        } else {
            let body;
            switch (this.props.activePage) {
                case ActivePage.Control:
                    body = (<ControlPage/>)
                    break;
                case ActivePage.Config:
                    body = (<ConfigPage/>)
                    break;
                case ActivePage.Joysticks:
                    body = (<JoysticksPage/>)
                    break;
                default:
                    body = (<p>unimplemented</p>)
            }
            return (
                <div>
                    <ul className="nav nav-tabs">
                        <li className="nav-item">
                            <a href="#"
                               className={`nav-link ${this.props.activePage == ActivePage.Control ? "active" : ""}`}
                               onClick={() => this.props.changePage(ActivePage.Control)}>Control</a>
                        </li>
                        <li className="nav-item">
                            <a href="#"
                               className={`nav-link ${this.props.activePage == ActivePage.Config ? "active" : ""}`}
                               onClick={() => this.props.changePage(ActivePage.Config)}>Config</a>
                        </li>
                        <li className="nav-item">
                            <a href="#"
                               className={`nav-link ${this.props.activePage == ActivePage.Joysticks ? "active" : ""}`}
                               onClick={() => this.props.changePage(ActivePage.Joysticks)}>Joysticks</a>
                        </li>
                    </ul>
                    {body}
                </div>
            )
        }
    }

    connectWs(port: number) {
        let ws = new WebSocket("ws://127.0.0.1:" + port + "/ws/index");
        ws.onopen = () => {
            this.props.socketConnected(ws);
        }
        ws.onmessage = (ev: MessageEvent) => {
            if (typeof ev.data === "string") {
                this.props.socketMessage(JSON.parse(ev.data))
            }
        }
    }
}

export default connector(DriverStation)

