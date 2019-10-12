import React from 'react';
import logo from './logo.svg';
import './App.css';

const App: React.FC = () => {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
            Reactjs epic style gang gang
        </p>
        <button onClick={yos}>
            Click here for epic
        </button>
      </header>
    </div>
  );
}

function yos() {
    // @ts-ignore
    window.external.invoke("test");
}

export default App;
