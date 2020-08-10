import React from 'react'
import {AllianceStation, AllianceColour, initRobotState, Mode, RobotState, UpdateGSM, Message} from './ipc'
import {ActivePage, CHANGE_PAGE, DriverStationState, SOCKET_CONNECTED} from "./store";
import {connect, ConnectedProps} from "react-redux";
import ControlPage from "./components/ControlPage";
import ConfigPage from "./components/ConfigPage";

const mapState = (state: DriverStationState) => ({
    activePage: state.activePage
});

const mapDispatch = {
    socketConnected: (ws: WebSocket) => ({ type: SOCKET_CONNECTED, ws: ws }),
    socketMessage: (msg: Message) => (msg),
    changePage: (page: ActivePage) => ({type: CHANGE_PAGE, page: page })
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
        let body;
        switch(this.props.activePage) {
            case ActivePage.Control:
                body = (<ControlPage />)
                break;
            case ActivePage.Config:
                body = (<ConfigPage />)
                break;
            default:
                body = (<p>unimplemented</p>)
        }
        return (
            <div>
                <ul className="nav nav-tabs">
                    <li className="nav-item">
                        <a href="#" className={`nav-link ${this.props.activePage == ActivePage.Control ? "active" : ""}`}
                           onClick={() => this.props.changePage(ActivePage.Control)}>Control</a>
                    </li>
                    <li className="nav-item">
                        <a href="#" className={`nav-link ${this.props.activePage == ActivePage.Config ? "active" : ""}`}
                           onClick={() => this.props.changePage(ActivePage.Config)}>Config</a>
                    </li>
                </ul>
                {body}
            </div>
        )
    }

    connectWs(port: number) {
        let ws = new WebSocket("ws://127.0.0.1:" + port + "/ws/index");
        ws.onopen = () => {
            this.props.socketConnected(ws);
        }
        ws.onmessage = (ev: MessageEvent) => {
            if(typeof ev.data === "string") {
                this.props.socketMessage(JSON.parse(ev.data))
            }
        }
    }
}

export default connector(DriverStation)

