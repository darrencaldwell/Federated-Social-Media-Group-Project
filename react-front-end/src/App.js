import React from 'react';
import Nav from "./components/Nav";
import {BrowserRouter as Router, Switch, Route} from "react-router-dom";
import Home from "./pages/Home";
// import CreatePost from "./pages/CreatePost"
import Login from "./pages/Login";
import Register from "./pages/Register";
import '../node_modules/bootstrap/dist/css/bootstrap.min.css'

class App extends React.Component {
    constructor(props) {
        super(props);
        this.state = {token: localStorage.getItem('token')}; // set initial state
    }
    login = () => { // used to update App state from within login page
        this.setState({ token: localStorage.getItem('token')});
    }
    logout = () => { // used to update App state from within navbar
        localStorage.clear();
        this.setState({ token: localStorage.getItem('token')});
    }

    render() {
        const {token} = this.state;
        return (

            <Router>
                <div className="App">
                    <Nav isLoggedIn={token} logout={this.logout}  /> 

                    <div className="auth-wrapper">
                        <div className="auth-inner">
                            <Switch>
                                <Route exact path="/" component={() => <Home user={this.state.user}/>}/>
                                <Route exact path="/login" component={() => <Login login={this.login}/>}/>
                                <Route exact path="/register" component={Register}/>
                            </Switch>
                        </div>
                    </div>
                </div>
            </Router>
        );
    }
}

export default App;
