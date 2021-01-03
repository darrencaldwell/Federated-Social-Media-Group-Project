import React, {Component} from 'react';
import {Card, Button} from "react-bootstrap";
// import '../styling/individualPost.css';

export class Posts extends Component {
    render() {
        return (
            <Card className="mt-3" >
                <Card.Body onClick={() => this.props.expandPost(this.props.post.postId)}>
                    <Card.Title>{this.props.post.postTitle}</Card.Title>
                    <Card.Text>
                        {this.props.post.postMarkup}
                    </Card.Text>
                    <Button variant="light" onClick={() => this.props.expandPost(this.props.post.postId)}>Expand Post</Button>
                </Card.Body>
            </Card>
        )
    }
}

export default Posts
