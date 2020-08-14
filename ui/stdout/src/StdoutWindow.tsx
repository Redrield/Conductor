import {SOCKET_CONNECTED, State} from "./store";
import {Message} from "./ipc";
import {connect, ConnectedProps} from "react-redux";
import React from "react";
import InfiniteScroll from "react-infinite-scroll-component";

const mapState = (state: State) => ({
    enabled: state.enabled,
    stdout: state.stdout
});

const mapDispatch = {
    socketConnected: (ws: WebSocket) => ({type: SOCKET_CONNECTED, ws: ws}),
    socketMessage: (msg: Message) => msg
};

const connector = connect(mapState, mapDispatch);

type ConnectProps = ConnectedProps<typeof connector>;

type Props = ConnectProps & {
    webserverPort: number;
}

function no() {}

const viewStyle = {
    width: "100%",
    height: "100%",
    color: "#fff"
}

const itemStyle = {
    minHeight: "20px"
}

class StdoutWindow extends React.Component<Props, any> {
    private readonly listRef: React.RefObject<HTMLDivElement>;

    constructor(props: Props) {
        super(props);

        this.listRef = React.createRef();
        this.connectWs(props.webserverPort);
    }

    componentDidUpdate() {
        if(this.props.enabled) {
            let node = this.listRef.current;
            if(node) {
                node.scrollIntoView({behavior: "auto"});
            }
        }
    }

    connectWs(port: number) {
        let ws = new WebSocket("ws://127.0.0.1:" + port + "/ws/stdout");
        ws.onopen = () => {
            this.props.socketConnected(ws);
        }
        ws.onmessage = (ev: MessageEvent) => {
            if (typeof ev.data === "string") {
                this.props.socketMessage(JSON.parse(ev.data))
            }
        }
    }

    render() {
        return (
            <InfiniteScroll next={no} hasMore={false} loader={""} dataLength={this.props.stdout.length}
                            style={viewStyle} className="bg-dark form-control">
                {this.props.stdout.map(msg => (
                    <div style={itemStyle}>{msg}</div>
                ))}
                <div ref={this.listRef} />
            </InfiniteScroll>
        )
    }
}

export default connector(StdoutWindow);