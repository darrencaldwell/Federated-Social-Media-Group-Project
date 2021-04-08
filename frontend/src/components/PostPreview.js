import React, {Component} from 'react';
import {Card} from "react-bootstrap";
import CardActionArea from '@material-ui/core/CardActionArea';
import Avatar, {Cache} from 'react-avatar';
//import {Link} from 'react-router-dom';
import Voting from './Voting';
import TimeSince from './TimeSince';
// import '../styling/individualPost.css';
import '../styling/container-pages.css';
import ReactMarkdown from 'react-markdown';

// for react avatar
const cache = new Cache({

    // Keep cached source failures for up to 7 days
    sourceTTL: 7 * 24 * 3600 * 1000,

    // Keep a maximum of 0 entries in the source cache (we don't care about remembering broken links!)
    sourceSize: 0
});

// props: post, impID, forumID, subforumID
export class PostPreview extends Component {
    constructor(props) {
        super(props);
        this.state = {}
    }

    render() {
        const parsed_user_link = btoa(this.props.post._links.user.href)

        return (
            <Card border="dark" className="mt-3 post" >
                <CardActionArea style={{ textDecoration: 'none' }} 
                                href={'/' + this.props.impID + '/' + this.props.forumID + '/' + this.props.subforumID + '/' + this.props.post.id}>
                    <Card.Body>
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
                                    <CardActionArea href={'/user/' + parsed_user_link}>
                                        <Avatar cache={cache} size="50" round={true} src={this.state.profilePicture}
                                            name={this.props.post.username}/>
                                        {"  "} {this.props.post.username}
                                    </CardActionArea>
                                    <Card.Subtitle className="text-muted mt-1 time-since">
                                        <TimeSince createdTime={this.props.post.createdTime}/>
                                    </Card.Subtitle>
                                    <Card.Subtitle className="text-muted mt-1 time-since">
                                        <TimeSince createdTime={this.props.post.createdTime} modifiedTime={this.props.post.modifiedTime}/>
                                    </Card.Subtitle>
                                </div>
                            </div>
                            <Card.Body className="post-text">
                                <Card.Title className="post-title"> {this.props.post.postTitle}</Card.Title>
                                <ReactMarkdown className="post-body">{this.props.post.postContents}</ReactMarkdown>     {/*Use the body from the prop as the body */}
                            </Card.Body>
                        </div>
                    </Card.Body>
                </CardActionArea>
            </Card>
        )
    }
}

export default PostPreview
