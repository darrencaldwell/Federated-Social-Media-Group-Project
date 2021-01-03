import React, { Component } from 'react'
import {Card} from "react-bootstrap";

export class Comment extends Component {
    // The empty lines are part of the format. Do Not Remove them
    render() {
        return (
            <Card>
                <Card.Body>
                    UserID:{this.props.comment.userId}-    {this.props.comment.commentContent}
                </Card.Body>
            </Card>
        )
    }
}

export default Comment
