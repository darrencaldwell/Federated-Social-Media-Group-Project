import React from 'react';
import {observer} from "mobx-react";
import Login from "./pages/Login";
import Home from "./pages/Home";
import Post from "./pages/Posts"
import PostList from "./components/PostList"
import Posts from "./pages/Posts";
import Register from "./pages/Register";
import './styling/App.css';
import Navbar from "./components/Navbar/Navbar";
import {BrowserRouter as Router, Switch, Route} from "react-router-dom";


class App extends React.Component {

    constructor(props) {
        super(props)
        //this.expandPost = this.expandPost.bind(this)
        this.state = {
            loading: false,
            info: {},
            listingPosts: false,
            listingPost: false
        }
    }




    async componentDidMount() {
        try {
            this.setState({loading: true, listingPosts: false, listingPost: false});
//            let url = "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/subforums/1/posts";
            let url = "/api/subforums/1/posts";
            let res = await fetch(url
                , {
                    method: 'get',
                    headers: {
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );
            let result = await res.json();
            console.log(result);
            this.setState({info: result, loading: false, listingPosts: true});
        } catch (e) {
            this.setState({loading: false});
        }

    }

    expandPost = async (link) => {
        try {
            this.setState({loading: true, listingPosts: false, listingPost: false});
            let url = link;
            let res = await fetch(url
                , {
                    method: 'get',
                    headers: {
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );
            let result = await res.json();
            this.setState({info: result, loading: false, listingPost: true});
        } catch (e) {
            this.setState({loading: false});
        }
    }


    render() {

        if (this.state.loading) {
            return (
                <div className="App">
                    loading...
                </div>
            );
        } else if (!this.state.loading && this.state.listingPosts) {
            return (
                <div className="App">
                    <PostList postList={this.state.info._embedded} expandPost={this.expandPost}/>
                </div>
            )
        } else if (!this.state.loading && this.state.listingPost) {
            return (
                <div>
                    <Post post={this.state.info} expandPost={this.expandPost}/>
                </div>
            )
        }

        return (
            <div className="app">
                <Router>
                    <Navbar/>
                    <Switch>
                        <Route path="/" exact component={Home}/>
                        <Route path="/login" component={Login}/>
                        <Route path="/register" component={Register}/>
                        <Route path="/posts" component={Posts}/>
                        <Route path="/post" component={Post}/>
                    </Switch>
                </Router>
            </div>
        );
    }
}

export default observer(App);
