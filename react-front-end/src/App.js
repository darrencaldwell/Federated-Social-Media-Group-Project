import React from 'react';
import Nav from "./components/Nav";
import {BrowserRouter as Router, Switch, Route} from "react-router-dom";
import Home from "./pages/Home";
// import CreatePost from "./pages/CreatePost"
import Login from "./pages/Login";
import Register from "./pages/Register";
import '../node_modules/bootstrap/dist/css/bootstrap.min.css'
import axios from "axios";

class App extends React.Component {
    state = {}
    componentDidMount = () => {

        axios.get('user')
            .then(res => {
                    this.setUser(res.data)
                },
                err => {
                    console.log(err)
                })
    }

    setUser = user => {
        this.setState({
            user: user
        })
    }

    render() {
        return (

            <Router>
                <div className="App">
                    <Nav user={this.state.user} setUser={this.setUser}/>

                    <div className="auth-wrapper">
                        <div className="auth-inner">
                            <Switch>
                                <Route exact path="/" component={() => <Home user={this.state.user}/>}/>
                                <Route exact path="/login" component={() => <Login setUser={this.setUser}/>}/>
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
