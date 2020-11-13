import React, { Component } from 'react';
import '../styling/Post.css';

export class Posts extends Component {
    render() {
        return (
            <div>
                <h1 className="postTitle">
                    {this.props.post.postTitle}
                </h1>
                <p className="postMarkup">
                    {this.props.post.postMarkup}
                </p>
                <p>
                    <button className="expandPostButton" onClick={() => this.props.expandPost(this.props.post.postId)}>
                            ^Expand post to see comments and stuff.^
                    </button>
                </p>     
            </div>
        )
    }
}

export default Posts
