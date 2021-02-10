import React from 'react';
import NavigationBar from "./components/NavigationBar";
import {BrowserRouter as Router, Switch, Route} from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";
import Register from "./pages/Register";
import CreateForum from "./components/CreateForum";
import CreatePost from "./components/CreatePost";
import CreateSubforum from "./components/CreateSubforum";
import ForumList from "./components/ForumList";
import Post from "./components/Post";
import PostList from "./components/PostList";
import SubforumList from "./components/SubforumList";
import CreateComment from "./components/CreateComment";
import Account from "./pages/Account";
import '../node_modules/bootstrap/dist/css/bootstrap.min.css'
import './styling/container-pages.css';
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
                    <div className="columns">
                        <Switch>
                            <Route exact path="/"/>
                            <Route exact path="/login"/>
                            <Route exact path="/register"/>
                            <Route exact path="/account"/>
                            <Route exact path="/forums" component={ForumList}/>
                            <Route exact path="/new" component={ForumList}/>
                            <Route path="/:forumID" component={ForumList}/>
                        </Switch>
                        <Switch>
                            <Route exact path="/"/>
                            <Route exact path="/login"/>
                            <Route exact path="/register"/>
                            <Route exact path="/account"/>
                            <Route exact path="/forums"/>
                            <Route exact path="/new"/>
                            <Route path="/:forumID" component={SubforumList}/>  {/*SubForumList gets forum ID from this.props.match.params.forumID */}
                        </Switch>
                        <Switch>
                            <Route exact path="/" component={() => <Home user={this.state.user}/>}/> {/*homepage*/}
                            <Route exact path="/login" component={() => <Login login={this.login}/>}/> {/*login page*/}
                            <Route exact path="/register" component={Register}/> {/*registration page*/}

                            {/*these should all contain some sort of identifier for the instance, but that is not implemented yet*/}
                            <Route exact path="/account" component={Account}/> {/*your account, should be replaced with /user/:id*/}

                            <Route exact path="/forums" component={() => <Home user={this.state.user}/>}/> {/*shows the list of forums (for our server)*/}
                            <Route exact path="/new" component={CreateForum}/> {/*page to create a new forum*/}
                            <Route exact path="/:forumID"/> {/*could have forum info here*/}

                            <Route exact path="/:forumID/new" component={CreateSubforum}/> {/*page to create a new subforum in the forum*/}
                            <Route exact path="/:forumID/:subforumID" component={PostList}/> {/*page for a specific subforum*/}

                            <Route exact path="/:forumID/:subforumID/new" component={CreatePost}/> {/*page to create a new post in a subforum*/}
                            <Route exact path="/:forumID/:subforumID/:postID" component={Post}/> {/*page for a specific post, contains comments and ability to create a comment*/}
                            <Route exact path="/:forumID/:subforumID/:postID/new" component={CreateComment}/> {/*page to comment on a post */}
                            <Route exact path="/:forumID/:subforumID/:postID/:commentID/new" component={CreateComment}/> {/*page to comment on a comment */}
                        </Switch>
                    </div>
                </div>
            </Router>
        );
    }
}

export default App;
