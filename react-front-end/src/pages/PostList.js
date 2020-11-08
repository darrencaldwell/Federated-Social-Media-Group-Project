import React, { Component } from 'react';
import Posts from './Posts';

class PostList extends Component {

    render() {
        if (this.props.postList.postList !== null) {
            // The property you want is called postList and then the array you want is called postList
            return this.props.postList.postList.map((post) => (
                <Posts key={post.id} post={post} expandPost={this.props.expandPost}></Posts>
            ));
        } else {
            return (
                <div>
                    No posts avaliable :(
                </div>
            )
        }
    }
}

export default PostList;
