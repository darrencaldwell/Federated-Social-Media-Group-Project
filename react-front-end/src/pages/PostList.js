import React, {Component} from 'react';
import Posts from './Posts';
import Post from './Post';

class PostList extends Component {

    constructor(props) {
        super(props)
        this.state = {
            loading: true,
            info: {},
            post: {},
            comments: {},
            listingPosts: false,
            listingPost: false
        }
    }

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

            return (
                <div className="container">
                    {this.state.info._embedded.postList.map((post) => (
                        <Posts key={post.id} post={post} expandPost={this.expandPost}/>
                    ))}
                </div>)
        } else if (!this.state.loading && this.state.listingPost) {
            return (
                <div>
                    <Post post={this.state.post} comments={this.state.comments._embedded}
                          loadPosts={this.componentDidMount}/>
                </div>
            )
        } else {
            return (
                <div>
                    Error has occurred and nothing is loaded :(
                </div>
            )
        }
    }
}

export default PostList;
