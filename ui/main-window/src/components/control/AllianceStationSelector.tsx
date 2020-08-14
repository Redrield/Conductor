import {DriverStationState} from "../../store";
import {AllianceColour, AllianceStation, UPDATE_ALLIANCE_STATION} from "../../ipc";
import {connect, ConnectedProps} from "react-redux";
import React from "react";

const mapState = (state: DriverStationState) => ({
    alliance: state.alliance
});

const mapDispatch = {
    updateAlliance: (alliance: AllianceStation) => ({type: UPDATE_ALLIANCE_STATION, station: alliance})
};

const connector = connect(mapState, mapDispatch);

type Props = ConnectedProps<typeof connector>;

function prettyPrint(alliance: AllianceStation): string {
    return alliance.color.toString() + " " + alliance.value
}

function allianceStations(props: Props) {
    let stations = []
    for(let i = 1; i <= 6; i++) {
        if(i <= 3) {
            let stn = { color: AllianceColour.Red, value: i };
            stations.push((<a className="dropdown-item py-1" href="#" onClick={(_) => props.updateAlliance(stn)}>{prettyPrint(stn)}</a>))
        } else {
            let stn = { color: AllianceColour.Blue, value: i - 3};
            stations.push((<a className="dropdown-item py-1" href="#" onClick={(_) => props.updateAlliance(stn)}>{prettyPrint(stn)}</a>))
        }
    }

    return (
        <>
            {stations}
        </>
    )
}

const AllianceStationSelector = (props: Props) => (
    <div className="input-group">
        <div className="input-group-prepend">
            <label htmlFor="teamSelectorDropdown" className="dropdown-label lead font-weight-normal">Team Station </label>
        </div>
        <div className="dropdown" id="teamSelectorDropdown">
            <button className="btn btn-secondary dropdown-toggle"
                    type="button" id="dropdownMenuButton"
                    data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                {prettyPrint(props.alliance)}
            </button>
            <div className="dropdown-menu py-1" aria-labelledby="dropdownMenuButton">
                {allianceStations(props)}
            </div>
        </div>
    </div>
)

export default connector(AllianceStationSelector)