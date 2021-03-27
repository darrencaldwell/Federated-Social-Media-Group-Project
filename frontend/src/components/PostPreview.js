import React, {Component} from 'react';
import {Card} from "react-bootstrap";
import CardActionArea from '@material-ui/core/CardActionArea';
import Voting from './Voting';
import TimeSince from './TimeSince';
// import '../styling/individualPost.css';
import '../styling/container-pages.css';

// props: post, impID, forumID, subforumID
export class PostPreview extends Component {

    render() {
        return (
            <Card border="dark" className="mt-3" >
                <Card.Body>
                    <CardActionArea href={'/' + this.props.impID + '/' + this.props.forumID + '/' + this.props.subforumID + '/' + this.props.post.id}>
                        <div className="post-columns">
                            <div className="post-comment-voting-container">
                                <Voting className="voting" upvotes={this.props.post.upvotes} 
                                    downvotes={this.props.post.downvotes} 
                                    _userVotes={this.props.post._userVotes}
                                    type="posts"
                                    postID={this.props.post.id}
                                    impID={this.props.impID}
                                ></Voting>
                                {/*Links to the post itself, to view/make comments. Removing the /api part directs you to the correct app page. */}
                                <div className="voting-adj">
                                    <Card.Subtitle className="text-muted mt-1 time-since">
                                        Post made by {this.props.post.username} <TimeSince createdTime={this.props.post.createdTime}/>
                                    </Card.Subtitle>
                                    <Card.Subtitle className="text-muted mt-1 time-since">
                                        <TimeSince createdTime={this.props.post.createdTime} modifiedTime={this.props.post.modifiedTime}/>
                                    </Card.Subtitle>
                                </div>
                            </div>
                            <Card.Body>
                                <Card.Title className="post-title"> {this.props.post.postTitle}</Card.Title>
                                <Card.Text className="post-body">{this.props.post.postContents}</Card.Text>     {/*Use the body from the prop as the body */}
                            </Card.Body>
                        </div>
                    </CardActionArea>
                </Card.Body>
            </Card>
        )
    }
}

export default PostPreview