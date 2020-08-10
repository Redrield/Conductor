import {DriverStationState} from "../../store";
import {connect, ConnectedProps} from "react-redux";
import React from "react";
import InfiniteScroll from "react-infinite-scroll-component";

const mapState = (state: DriverStationState) => ({
    messages: state.stdout,
    enabled: state.enabled
});

const mapDispatch = ({});

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

const stdoutStyle = {
    width: "330px",
    height: "200px",
    color: "#fff"
}

const itemStyle = {
    minHeight: "20px"
};

function no() {
}

class StdoutView extends React.Component<Props, any> {
    private listRef: React.RefObject<HTMLDivElement>;

    constructor(props: Props) {
        super(props);

        this.listRef = React.createRef();
    }

    componentDidUpdate() {
        if(this.props.enabled) {
            let node = this.listRef.current;
            if(node) {
                node.scrollIntoView({behavior: "auto"});
            }
        }
    }

    render() {
        return (
            <InfiniteScroll next={no} hasMore={false} loader={""} dataLength={this.props.messages.length}
                            style={stdoutStyle} className="form-control bg-secondary mt-4">
                {this.props.messages.map(msg => (
                    <div style={itemStyle}>{msg}</div>
                ))}
                <div ref={this.listRef} />
            </InfiniteScroll>
        )
    }
}

export default connector(StdoutView);