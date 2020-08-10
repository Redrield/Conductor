import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import * as serviceWorker from './serviceWorker';
import {Message, UpdateGSM} from "./ipc";
import {Provider} from "react-redux";
import {initState, rootReducer} from "./store";
import {createStore} from "redux";
import DriverStation from "./DriverStation";

// @ts-ignore
const store = createStore(rootReducer, initState());

ReactDOM.render(
  <React.StrictMode>
      <Provider store={store}>
          <DriverStation webserverPort={1234} />
      </Provider>
  </React.StrictMode>,
  document.getElementById('root')
);

function start(port: number) {
    let abc = { foo: "abcdef", port: port };

    return JSON.stringify({...abc, port: 6969});
}

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
