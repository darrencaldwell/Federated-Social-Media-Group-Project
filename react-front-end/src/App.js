import React, {useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';

function App() {
    const [currentText, setCurrentText] = useState(0); // defines getter and setter. Where getter currentText is just a variable

    useEffect(() => { // calls this function after a render
      fetch('/text').then (res => res.json()).then(data => { // retrieves the current time using a GET request on /text, proxying to Actix server
          setCurrentText(data.text);
      });
    }, []);

  return (
    <div className="App"> 
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      <p> The current text is {currentText}. </p> // current time var!
      </header>
    </div>
  );
}

export default App;
