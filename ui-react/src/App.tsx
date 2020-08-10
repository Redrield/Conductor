import React from 'react';
import logo from './logo.svg';
import './App.css';

interface AppState {
  a: string;
  b: string;
}

export class App extends React.Component<any, AppState> {
  constructor(props: any) {
    super(props)
    this.state = { a: "foobar", b: "foobaz" };
    this.clicky = this.clicky.bind(this);
    this.clicky2 = this.clicky2.bind(this);
  }

  render() {
    return (<div className="App">
      <p>a is {this.state.a}; b is {this.state.b}</p>
      <button onClick={this.clicky}>Click to change</button>
      <button onClick={this.clicky2}>Click to change moar</button>
    </div>)
  }

  clicky(e: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
      this.setState({ b: "fUwUbaz" });
  }

  clicky2(e: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    this.setState({ a: "fUwUbar", b: "foobaz" });
  }
}

// function App() {
//   return (
//     <div className="App">
//       <header className="App-header">
//         <img src={logo} className="App-logo" alt="logo" />
//         <p>
//           Edit <code>src/App.tsx</code> and save to reload.
//         </p>
//         <a
//           className="App-link"
//           href="https://reactjs.org"
//           target="_blank"
//           rel="noopener noreferrer"
//         >
//           Learn React
//         </a>
//       </header>
//     </div>
//   );
// }

export default App;
