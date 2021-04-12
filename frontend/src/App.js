import React from 'react';
import NavigationBar from "./components/NavigationBar";
import {BrowserRouter as Router, Switch, Route} from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";
import Register from "./pages/Register";
import CreateForum from "./components/CreateForum";
import CreatePost from "./components/CreatePost";
import CreateSubforum from "./components/CreateSubforum";
import Chat from "./components/Chat";
import ForumList from "./components/ForumList";
import Post from "./components/Post";
import PostList from "./components/PostList";
import SubforumList from "./components/SubforumList";
import CreateComment from "./components/CreateComment";
import Account from "./pages/Account";
import AccountComments from "./components/account/AccountComments";
import AccountPosts from "./components/account/AccountPosts";
import EditPost from "./components/EditPost";
import EditComment from "./components/EditComment";
import '../node_modules/bootstrap/dist/css/bootstrap.min.css'
import './styling/container-pages.css';
import UserAccount from "./pages/UserAccount";
// import BackButton from "./components/BackButton";
import EditPerms from './components/permissions/EditPerms';


class App extends React.Component {
    componentDidMount = async () => {
        document.title = 'St BeeFives'
        try {
            // the url needs the post id from the props
            let url = '/local/implementations';
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({imp: {name: "local", id: 1}, impList: result._embedded.implementationList }); // we store the json for the post in the state

        } catch (e) {
            console.log("error: " + e);
        }
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

    changeImp = (imp) => {
        this.setState({imp: imp});
    }

    /**
     *
     * @returns {JSX.Element}
     */
    render() {
        // Whether user is logged in or logged out
        const {token, impList, imp} = this.state;
        return (
            <Router>
                <div className="App">
                    {/* Pass the state onto Nav bar about state of user login /*/}
                    <NavigationBar imps={impList} currImp={imp} changeImp={this.changeImp} isLoggedIn={token} logout={this.logout}/>
                    {/*<BackButton/>*/}
                    <div className="columns">
                        <Switch>
                            <Route path="/editperms"/>
                            <Route exact path="/user/:userURL"/>
                            <Route exact path="/:impID/forums" component={ForumList}/>
                            <Route exact path="/:impID/new" component={ForumList}/>
                            <Route path="/:impID/:forumID" component={ForumList}/>
                        </Switch>
                        <Switch>
                            <Route path="/editperms"/>
                            <Route exact path="/:impID/forums"/>
                            <Route exact path="/:impID/new"/>
                            <Route exact path="/user/:userURL"/>
                            <Route exact path="/:impID/:forumID" component={SubforumList}/>  {/*SubForumList gets forum ID from this.props.match.params.forumID */}
                            <Route exact path="/:impID/:forumID/new" component={SubforumList}/> 
                            <Route exact path="/:impID/:forumID/chat" component={SubforumList}/> 
                            <Route path="/:impID/:forumID/:subforumID" component={SubforumList}/> 
                        </Switch>
                        <Switch>
                            <Route exact path="/" component={() => <Home user={this.state.user} currImp={this.state.imp}/>}/> {/*default homepage*/}
                            <Route exact path="/login" component={() => <Login login={this.login}/>}/> {/*login page*/}
                            <Route exact path="/register" component={Register}/> {/*registration page*/}
                            <Route exact path="/usercomments" component={AccountComments}/> {/* Comments done by a given user */}
                            <Route exact path="/userposts" component={AccountPosts}/> {/* Posts done by a given user*/}

                            {/*<Route exact path="/user/:id/comments" component={AccountComments}/>*/}
                            {/*<Route exact path="/user/:id/posts" component={AccountPosts}/>*/}

                            {/*<Route exact path="/userposts" component={AccountPosts}/>*/}
                            {/*these should all contain some sort of identifier for the instance, but that is not implemented yet*/}
                            <Route exact path="/account" component={Account}/> {/*your account, should be replaced with /user/:id*/}
                            <Route exact path="/user/:userURL" component={UserAccount}/>
                            {/*<Route exact path="/user/:impID/:userId" component={UserAccount}/>*/}

                            <Route exact path="/:impID" component={props => <Home {...props} user={this.state.user} currImp={this.state.imp}/>}/> {/*default homepage*/}
                            <Route exact path="/editperms/:type/:id/:name" component={EditPerms}></Route>

                            {/*<Route exact path="/:impID/forums" component={() => <Home user={this.state.user}/>}/> {/*shows the list of forums (for our server)*/}
                            <Route exact path="/:impID/new" component={CreateForum}/> {/*page to create a new forum*/}
                            <Route exact path="/:impID/:forumID"/> {/*could have forum info here*/}

                            <Route exact path="/:impID/:forumID/chat" component={Chat}/> {/*could have forum info here*/}
                            <Route exact path="/:impID/:forumID/new" component={CreateSubforum}/> {/*page to create a new subforum in the forum*/}
                            <Route exact path="/:impID/:forumID/:subforumID" component={PostList}/> {/*page for a specific subforum*/}

                            <Route exact path="/:impID/:forumID/:subforumID/new" component={CreatePost}/> {/*page to create a new post in a subforum*/}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID" component={Post}/> {/*page for a specific post, contains comments and ability to create a comment*/}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID/edit" component={EditPost}/> {/*like create post, but edits*/}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID/new" component={CreateComment}/> {/*page to comment on a post */}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID/:commentID" component={Post}/> {/*page for a post with expanded comments */}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID/:commentID/edit" component={EditComment}/> {/*like create comment, but edits*/}
                            <Route exact path="/:impID/:forumID/:subforumID/:postID/:commentID/new" component={CreateComment}/> {/*page to comment on a comment */}
                        </Switch>
                    </div>
                </div>
            </Router>
        );
    }
}

export default App;
