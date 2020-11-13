import React, {Component} from 'react';
import Posts from './Posts';
import Post from './Post';

class PostList extends Component {

    constructor(props) {
        super(props)
        this.state = {
            loading: true, // Set to true if loading
            info: {}, // Stores list of posts
            post: {}, // Stores a specific post
            comments: {}, // Stores a list of comments for a post
            listingPosts: false, // Set to true when rendering a list of posts
            listingPost: false // Set to true when rendering a specific post
        }
    }

    // Loads a list of posts, hard coded for now
    componentDidMount = async () => {
        try {
            this.setState({loading: true, listingPosts: false, listingPost: false});
            let url = "/api/subforums/1/posts";
            let res = await fetch(url
                , {
                    method: 'get',
                    withCredentials: true,
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );
            let result = await res.json();
            this.setState({info: result, loading: false, listingPosts: true}
            );
        } catch (e) {
            this.setState({loading: false});
        }

    }

    // Takes the url of a specific post and displays extra things like comments and other things that will be
    // implemented in the future.
    expandPost = async (id) => {
        try {
            this.setState({loading: true, listingPosts: false, listingPost: false});

            this.state.info._embedded.postList.forEach((post) => {
                if (post.postId === id) {
                    this.state.post = post;
                }
            });
            this.setState({listingPost: true});
            this.getComments(this.state.post._links.comments.href);
        } catch (e) {
            this.setState({loading: false});
        }
    }

    getComments = async (link) => {
        try {
            // Remove replace when CORS isn't blocking
            let url = link.replace('https://cs3099user-b5.host.cs.st-andrews.ac.uk', '');
            let res = await fetch(url
                , {
                    method: 'GET',
                    withCredentials: true,
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );
            let result = await res.json();
            this.setState({comments: result, loading: false});
        } catch (e) {
            console.log(e);
            console.log("Failed loading in comments");
        }
    }


    render() {
        if (this.state.loading) {
            return (
                <div>
                    <p className="loader"/>
                    <p>Loading...</p>
                </div>

            )
        } else if (!this.state.loading && this.state.listingPosts) {
            // If we are rendering a list of posts go through the returned array of posts and display them.
            return (
                <div className="container">
                    {this.state.info._embedded.postList.map((post) => (
                        <Posts key={post.id} post={post} expandPost={this.expandPost}/>
                    ))}
                </div>)
        } else if (!this.state.loading && this.state.listingPost) {
            // If we are rendering a singular post display everything like comments and other things to be implemented later
            return (
                <div>
                    <Post post={this.state.post} comments={this.state.comments._embedded}
                          loadPosts={this.componentDidMount}/>
                </div>
            )
        } else {
            // If for some reason loading is over but something can't be displayed
            return (
                <div>
                    Error has occurred and nothing is loaded :(
                </div>
            )
        }
    }
}

export default PostList;
