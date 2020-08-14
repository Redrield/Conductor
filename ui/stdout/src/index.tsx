import React from 'react';
import ReactDOM from 'react-dom';
import * as serviceWorker from './serviceWorker';
import {createStore} from "redux";
import {initState, reducer, STUPID_INITIALIZER} from "./store";
import StdoutWindow from "./StdoutWindow";
import {Provider} from "react-redux";

// @ts-ignore
// const store = createStore(reducer, initState());
// store.dispatch({type:STUPID_INITIALIZER});
// ReactDOM.render(
//     <React.StrictMode>
//         <Provider store={store}>
//             <StdoutWindow webserverPort={1234} />
//         </Provider>
//     </React.StrictMode>,
//     document.getElementById("root")
// )

export function start(port: number) {
    // Why is preloaded state broken when this exact code functions in the main window?
    // god knows. Welcome to frontend
    // @ts-ignore
    const store = createStore(reducer, initState());
    store.dispatch({type:STUPID_INITIALIZER});

    ReactDOM.render(
        <React.StrictMode>
            <Provider store={store}>
                <StdoutWindow webserverPort={port} />
            </Provider>
        </React.StrictMode>,
        document.getElementById("root")
    );
}

// @ts-ignore
window.startapp = start;

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
