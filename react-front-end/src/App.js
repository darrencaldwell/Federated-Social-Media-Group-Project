import React from 'react';
import NavigationBar from "./components/NavigationBar";
import {BrowserRouter as Router, Switch, Route} from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";
import Register from "./pages/Register";
import ViewPosts from "./pages/ViewPosts";
import CreatePost from "./pages/CreatePost.js";
import Account from "./pages/Account";
import '../node_modules/bootstrap/dist/css/bootstrap.min.css'
// import BackButton from "./components/BackButton";

class App extends React.Component {
    componentDidMount() {
        document.title = 'St BeeFives'
    }
    constructor(props) {
        super(props);
        this.state = {token: localStorage.getItem('token')}; // set initial state
    }

    login = () => { // used to update App state from within login page
        this.setState({token: localStorage.getItem('token')});
    }

    logout = () => { // used to update App state from within navbar
        localStorage.clear();
        this.setState({token: localStorage.getItem('token')});
    }

    /**
     *
     * @returns {JSX.Element}
     */
    render() {
        // Whether user is logged in or logged out
        const {token} = this.state;
        return (

            <Router>
                <div className="App">
                    {/* Pass the state onto Nav bar about state of user login /*/}
                    <NavigationBar isLoggedIn={token} logout={this.logout}/>
                    {/*<BackButton/>*/}
                    <Switch>
                        <Route exact path="/posts" component={() => <ViewPosts/>}/>
                        <Route exact path="/" component={() => <Home user={this.state.user}/>}/>
                        <Route exact path="/login" component={() => <Login login={this.login}/>}/>
                        <Route exact path="/register" component={Register}/>
                        <Route exact="/account" component={Account}/>
                        <Route exact path="/makePost" component={() => <CreatePost url="/api/subforums/1/posts"/>}/>
                    </Switch>

                </div>
            </Router>
        );
    }
}

export default App;
