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
import AccountComments from "./components/account/AccountComments";
import AccountPosts from "./components/account/AccountPosts";
import '../node_modules/bootstrap/dist/css/bootstrap.min.css'
import './styling/container-pages.css';
import UserAccount from "./pages/UserAccount";
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
                            <Route path="/:impID/:something" component={ForumList}/>
                        </Switch>
                        <Switch>
                            <Route exact path="/:impID/forums"/>
                            <Route exact path="/:impID/new"/>
                            <Route path="/:impID/:forumID" component={SubforumList}/>  {/*SubForumList gets forum ID from this.props.match.params.forumID */}
                        </Switch>
                        <Switch>
                            <Route exact path="/" component={() => <Home user={this.state.user}/>}/> {/*default homepage*/}
                            <Route exact path="/login" component={() => <Login login={this.login}/>}/> {/*login page*/}
                            <Route exact path="/register" component={Register}/> {/*registration page*/}
                            <Route exact path="/usercomments" component={AccountComments}/>
                            <Route exact path="/userposts" component={AccountPosts}/>

                            {/*<Route exact path="/user/:id/comments" component={AccountComments}/>*/}
                            {/*<Route exact path="/user/:id/posts" component={AccountPosts}/>*/}

                            {/*<Route exact path="/userposts" component={AccountPosts}/>*/}
                            {/*these should all contain some sort of identifier for the instance, but that is not implemented yet*/}
                            <Route exact path="/account" component={Account}/> {/*your account, should be replaced with /user/:id*/}
                            {/*<Route exact path="/account" component={() => <Account user={this.user_id}}/>}/>*/}
                            {/*<Route exact path="/user/:id" component={() => <UserAccount user={this.state.post_author}/>}/>*/}
                            {/*<Route exact path="/user/:userURL" component={UserAccount}/>*/}
                            <Route exact path="/user/:userURL" component={UserAccount}/>
                            {/*<Route exact path="/user/:impID/:userId" component={UserAccount}/>*/}

                            <Route exact path="/:impID" component={props => <Home {...props} user={this.state.user}/>}/> {/*default homepage*/}

                            {/*<Route exact path="/:impID/forums" component={() => <Home user={this.state.user}/>}/> {/*shows the list of forums (for our server)*/}
                            <Route exact path="/:impID/new" component={CreateForum}/> {/*page to create a new forum*/}
                            <Route exact path="/:impID/:forumID"/> {/*could have forum info here*/}

                            <Route exact path="/:impID/:forumID/new" component={CreateSubforum}/> {/*page to create a new subforum in the forum*/}
                            <Route exact path="/:impID/:forumID/:subforumID" component={PostList}/> {/*page for a specific subforum*/}

                            <Route exact path="/:impID/:forumID/:subforumID/new" component={CreatePost}/> {/*page to create a new post in a subforum*/}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID" component={Post}/> {/*page for a specific post, contains comments and ability to create a comment*/}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID/new" component={CreateComment}/> {/*page to comment on a post */}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID/:commentID" component={Post}/> {/*page for a post with expanded comments */}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID/:commentID/new" component={CreateComment}/> {/*page to comment on a comment */}
                        </Switch>
                    </div>
                </div>
            </Router>
        );
    }
}

export default App;
