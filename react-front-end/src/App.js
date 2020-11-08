import React from 'react';
import {observer} from "mobx-react";
import Login from "./pages/Login";
import Home from "./pages/Home";
import Post from "./pages/Posts"
import PostList from "./pages/PostList"
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


    // async componentDidMount() {
    //     try {
    //         let res = await fetch('/isLoggedIn', {
    //             method: 'post',
    //             headers: {
    //                 'Accept': 'application/json',
    //                 'Content-Type': 'application/json'
    //             }
    //         });
    //
    //         let result = await res.json();
    //
    //         if (result && result.success) {
    //             UserStore.loading = false;
    //             UserStore.isLoggedIn = true;
    //             UserStore.username = result.username;
    //         } else {
    //             UserStore.loading = false;
    //             UserStore.isLoggedIn = false;
    //         }
    //     } catch (e) {
    //         UserStore.loading = false;
    //         UserStore.isLoggedIn = false;
    //     }
    // }
    //
    // async doLogout() {
    //     try {
    //         let res = await fetch('/logout', {
    //             method: 'post',
    //             headers: {
    //                 'Accept': 'application/json',
    //                 'Content-Type': 'application/json'
    //             }
    //         });
    //
    //         let result = await res.json();
    //
    //         if (result && result.success) {
    //             UserStore.isLoggedIn = false;
    //             UserStore.username = '';
    //         }
    //     } catch (e) {
    //         console.log(e);
    //     }
    // }

    async componentDidMount () {
        try {
            this.setState({loading: true, listingPosts: false, listingPost: false});
            let url = "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/subforums/1/posts";
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

        // if (UserStore.loading) {
        //     return (
        //         <div className="app">
        //             <div className='container'>
        //                 Loading, please wait..
        //             </div>
        //         </div>
        //     );
        // } else {
        //     if (UserStore.isLoggedIn) {
        //         return (
        //             <div className="app">
        //                 <div className='container'>
        //                     Welcome {UserStore.username}
        //                     <SubmitButton
        //                         text={'Log out'}
        //                         disabled={false}
        //                         onClick={() => this.doLogout()}
        //                     />
        //                 </div>
        //             </div>
        //         );
        //     }

        if (this.state.loading) {
            return (
                <div className="App">
                    loading...
                </div>
            );
        } else if (!this.state.loading && this.state.listingPosts) {
            return (
                <div className="App">
                    <PostList postList={this.state.info._embedded} expandPost={this.expandPost}></PostList>
                </div>
            )
        } else if (!this.state.loading && this.state.listingPost) {
            return (
                <div>
                    <Post post={this.state.info} expandPost={this.expandPost}></Post>
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
