import React from 'react';
import ReactDOM from 'react-dom';
import * as serviceWorker from './serviceWorker';
import {Message, UpdateGSM} from "./ipc";
import {Provider} from "react-redux";
import {initState, rootReducer} from "./store";
import {createStore} from "redux";
import DriverStation from "./DriverStation";


// Uncomment this for dev server, comment again for rust integration
// @ts-ignore
// const store = createStore(rootReducer, initState());
//
// ReactDOM.render(
//     <React.StrictMode>
//         <Provider store={store}>
//             <DriverStation webserverPort={1234} />
//         </Provider>
//     </React.StrictMode>,
//     document.getElementById('root')
// );

export function start(port: number) {
    // @ts-ignore
    const store = createStore(rootReducer, initState());

    ReactDOM.render(
        <React.StrictMode>
            <Provider store={store}>
                <DriverStation webserverPort={port} />
            </Provider>
        </React.StrictMode>,
        document.getElementById('root')
    );
}

// @ts-ignore
window.startapp = start;

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
