import React from "react";
import {Draggable} from "react-beautiful-dnd";

export type JoystickData = {
    name: string;
    id: string;
}

type JoystickProps = JoystickData & {
    index: number;
}

export class Joystick extends React.Component<JoystickProps, any> {
    render() {
        return (
            <Draggable key={this.props.id} draggableId={this.props.id} index={this.props.index}>
                {(provided) => (
                    <div className="rounded bg-secondary d-flex border border-dark"
                         ref={provided.innerRef}
                         {...provided.draggableProps}
                         {...provided.dragHandleProps}>
                        <p className="align-self-center">{this.props.index}: {this.props.name}</p>
                    </div>
                )}
            </Draggable>
        );
    }
}