import React, {Component} from 'react';
import {Card} from "react-bootstrap";
import Voting from './Voting';
// import '../styling/individualPost.css';

// props: post, impID, forumID, subforumID
export class PostPreview extends Component {
    render() {
        return (
            <Card border="dark" className="mt-3" >
                <Card.Body>
                    <div class="post-preview-container">
                    <Voting class="voting-post" upvotes={this.props.post.upvotes}
    downvotes={this.props.post.downvotes}
    _userVotes={this.props.post._userVotes}
    type="posts"
    postID={this.props.post.id}
    impID={this.props.impID}
    />
                    {/*Links to the post itself, to view/make comments. Removing the /api part directs you to the correct app page. */}
                    <div class="post">
                    <Card.Link href={'/' + this.props.impID + '/' + this.props.forumID + '/' + this.props.subforumID + '/' + this.props.post.id}>{this.props.post.postTitle}</Card.Link>
                    <Card.Subtitle className="text-muted">
                        Post made by: {this.props.post.username} on TIME
                    </Card.Subtitle>
                    <Card.Text>{this.props.post.postContents}</Card.Text>     {/*Use the body from the prop as the body */}
                    </div>
                    </div>
                    <Card.Body>
                     <Card.Text>{this.props.post.postContents}</Card.Text>     {/*Use the body from the prop as the body */}
                    </Card.Body>
                </Card.Body>
            </Card>
        )
    }
}

export default PostPreview