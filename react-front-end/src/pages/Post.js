import React, { Component } from 'react';
import { BrowserRouter as Router, Link } from 'react-router-dom';

export class Post extends Component {
    render() {
        return (
            <div>
                <h1>
                    {this.props.post.postTitle}
                </h1>
                <p>
                    {this.props.post.postContents}
                </p>
                <Router>
                    <p>
                        <Link to={this.props.post._links.self.href} onClick={() => this.props.expandPost(this.props.post._links.self.href)}>
                            Link to this post (only link that works rn).
                        </Link>
                    </p>
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
                    <p>
                        <Link to={this.props.post._links.comments.href} onClick={() => { console.log("hello") }}>
                            comments of post (not working, prints hello to console).
                        </Link>
                    </p>

                </Router>
            </div>
        )
    }
}

export default Post
