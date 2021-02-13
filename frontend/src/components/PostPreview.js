import React, {Component} from 'react';
import {Card} from "react-bootstrap";
// import '../styling/individualPost.css';

// props: post, forumID, subforumID
export class PostPreview extends Component {
    render() {
        return (
            <Card className="mt-3" >
                <Card.Body>
                    <Card.Title>{this.props.post.postTitle}</Card.Title>    {/*Use the title from the prop as the title text */}
                    <Card.Text>{this.props.post.postContent}</Card.Text>     {/*Use the body from the prop as the body */}
                    {/*Links to the post itself, to view/make comments. Removing the /api part directs you to the correct app page. */}
                    <Card.Link href={'/' + this.props.forumID + '/' + this.props.subforumID + '/' + this.props.post.id}>View Post</Card.Link>
                </Card.Body>
            </Card>
        )
    }
}

export default PostPreview
