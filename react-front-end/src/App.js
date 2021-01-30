import React from 'react';
import NavigationBar from "./components/NavigationBar";
import {BrowserRouter as Router, Switch, Route} from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";
import Register from "./pages/Register";
import Root from "./pages/Root";
import NewForum from "./pages/NewForum";
import ForumRoot from "./pages/ForumRoot";
import NewSubforum from "./pages/NewSubforum";
import SubforumRoot from "./pages/SubforumRoot";
import NewPost from "./pages/NewPost";
import ExpandedPost from "./pages/ExpandedPost";
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
                        <Route exact path="/" component={() => <Home user={this.state.user}/>}/> {/*homepage*/}
                        <Route exact path="/login" component={() => <Login login={this.login}/>}/> {/*login page*/}
                        <Route exact path="/register" component={Register}/> {/*registration page*/}

                        {/*these should all contain some sort of identifier for the instance, but that is not implemented yet*/}
                        <Route exact path="/account" component={Account}/> {/*your account, should be replaced with /user/:id*/}

                        <Route exact path="/forums" component={Root}/> {/*shows the list of forums (for our server)*/}
                        <Route exact path="/new" component={NewForum}/> {/*page to create a new forum*/}
                        <Route exact path="/:forumID" component={ForumRoot}/> {/*page for a specific forum*/}

                        <Route exact path="/:forumID/new" component={NewSubforum}/> {/*page to create a new subforum in the forum*/}
                        <Route exact path="/:forumID/:subforumID" component={SubforumRoot}/> {/*page for a specific subforum*/}

                        <Route exact path="/:forumID/:subforumID/new" component={NewPost}/> {/*page to create a new post in a subforum*/}
                        <Route exact path="/:forumID/:subforumID/:postID" component={ExpandedPost}/> {/*page for a specific post, contains comments and ability to create a comment*/}
                    </Switch>
                </div>
            </Router>
        );
    }
}

export default App;
