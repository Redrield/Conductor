import {DriverStationState, REORDER_JOYSTICKS, UPDATE_JOYSTICK_MAPPING_INTERNAL} from "../store";
import {UPDATE_JOYSTICK_MAPPING} from "../ipc";
import {connect, ConnectedProps} from "react-redux";
import React from "react";
import {DragDropContext, Droppable, DropResult} from "react-beautiful-dnd";
import {Joystick, JoystickData} from "./joysticks/Joystick";

const mapState = (state: DriverStationState) => ({
    joysticks: state.joysticks,
    mappings: state.joystickMappings
})

const mapDispatch = {
    updateList: (js: JoystickData, startIdx: number, endIdx: number) => ({type:REORDER_JOYSTICKS, js:js, oldIdx: startIdx, newIdx:endIdx}),
    updateMapping: (name: string, pos: number, uuid: string) => ({type:UPDATE_JOYSTICK_MAPPING_INTERNAL, name:name, pos:pos, uuid:uuid})
}

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

class JoysticksPage extends React.Component<Props, any> {
    constructor(props: Props) {
        super(props)

        this.onDragEnd = this.onDragEnd.bind(this)
    }

    onDragEnd(result: DropResult) {
        if(!result.destination) {
            return;
        }

        this.props.updateList(this.props.joysticks[result.source.index], result.source.index, result.destination.index)
        this.props.updateMapping(this.props.joysticks[result.source.index].name, result.destination.index, this.props.joysticks[result.source.index].id);
    }

    render() {
        return (<div className="container">
            <DragDropContext onDragEnd={this.onDragEnd}>
                <div className="row">
                    <div className="col col-md-3">
                        <Droppable droppableId="joysticksList">
                            {(provided) => (
                                <div
                                    ref={provided.innerRef}
                                    {...provided.droppableProps}>
                                    {this.props.joysticks.map((data, index) => (<Joystick {...data} index={index} />))}
                                    {provided.placeholder}
                                </div>
                            )}
                        </Droppable>
                    </div>
                </div>
            </DragDropContext>
        </div>);
    }
}

export default connector(JoysticksPage)