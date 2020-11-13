import React, { Component } from 'react';
//import { BrowserRouter as Router, Link } from 'react-router-dom';
import Comments from '../components/Comments';
import Make from './Make.js';
import '../styling/Post.css';
export class Post extends Component {
    render() {
        const url = "/api/posts/" + this.props.post.postId + "/comments";
        // If the list of commnts is empty print out a message saying so else print out comments
        if (this.props.comments.commentList.length === 0) {
            return (
                <div>
                    <h1 className="postTitle">
                        {this.props.post.postTitle}
                    </h1>
                    <p className="postMarkup">
                        {this.props.post.postMarkup}
                    </p>
                    <p>
                        <button onClick={() => this.props.loadPosts()}>Go back to list of posts</button>
                    </p>
                    <p>
                        Make posts here means make comment. Will be fixed later.
                    </p>
                    <div>
                        <Make mode="comment" url={url} />
                    </div>
                    <div>
                        <h>Comments:</h>
                        <p>No comments have been made yet.</p>
                    </div>

                    {/* <Router>
                        <p>
                            <Link to={this.props.post._links.subforum.href} onClick={() => { console.log("hello") }}>
                                subforum post belongs to (not working, prints hello to console).
                            </Link>
                        </p>
                        <p>
                            <Link to={this.props.post._links.forum.href} onClick={() => { console.log("hello") }}>
                                forum that post belongs to (not working, prints hello to console).
                            </Link>
                        </p>
                        <p>
                            <Link to={this.props.post._links.user.href} onClick={() => { console.log("hello") }}>
                                user that posted this (not working, prints hello to console).
                            </Link>
                        </p>
                    </Router> */}
                </div>
            )
        } else {
            return (
                <div>
                    <h1 className="postTitle">
                        {this.props.post.postTitle}
                    </h1>
                    <p className="postMarkup">
                        {this.props.post.postMarkup}
                    </p>
                    <p>
                        <button onClick={() => this.props.loadPosts()}>Go back to list of posts</button>
                    </p>
                    <p>
                        Make posts here means make comment. Will be fixed later.
                    </p>
                    <div>
                        <Make mode="comment" url={url} />
                    </div>
                    <div>
                        <h>Comments:</h>
                        {this.props.comments.commentList.map((comment) => (
                            <Comments key={comment.id} comment={comment} ></Comments>

                        ))}
                    </div>
                    {/* <Router>
                        <p>
                            <Link to={this.props.post._links.subforum.href} onClick={() => { console.log("hello") }}>
                                subforum post belongs to (not working, prints hello to console).
                            </Link>
                        </p>
                        <p>
                            <Link to={this.props.post._links.forum.href} onClick={() => { console.log("hello") }}>
                                forum that post belongs to (not working, prints hello to console).
                            </Link>
                        </p>
                        <p>
                            <Link to={this.props.post._links.user.href} onClick={() => { console.log("hello") }}>
                                user that posted this (not working, prints hello to console).
                            </Link>
                        </p>
                    </Router> */}
                </div>
            )
        }
    }
}

export default Post
