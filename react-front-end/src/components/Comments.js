import React, { Component } from 'react'

export class Comment extends Component {
    // The empty lines are part of the format. Do Not Remove them
    render() {
        return (
                <p>
                    UserID:{this.props.comment.userId}-    {this.props.comment.commentContent}

                </p>
        )
    }
}

export default Comment
