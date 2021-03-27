import React, {Component} from 'react';
import {Card} from "react-bootstrap";
import {Link} from 'react-router-dom';
import Voting from './Voting';
// import '../styling/individualPost.css';

// props: post, impID, forumID, subforumID
export class PostPreview extends Component {

    constructor(props) {
        super(props);
        let date_created = new Date(this.props.post.createdTime * 1000)
        let date_modified = new Date(this.props.post.modifiedTime * 1000)
        let diff = new Date (Math.abs(date_modified - date_created))
        let modified_string
        if (diff < 60000) { // 60s before editing is noticed
            modified_string = "Never"
        } else {
            modified_string = diff.getHours() + 'h ' + diff.getMinutes() + 'm ago'
        }
        this.state = {
            time:  date_created.getHours() + ':' + date_created.getMinutes() + ', ' + date_created.toDateString(),
            mod_time: modified_string
        };
    }

    render() {

        return (
            <Card border="dark" className="mt-3" >
                <Card.Body>
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
                            <Card.Link as={Link} to={'/' + this.props.impID + '/' + this.props.forumID + '/' + this.props.subforumID + '/' + this.props.post.id}>{this.props.post.postTitle}</Card.Link>
                            <Card.Subtitle className="text-muted mt-1">
                                by: {this.props.post.username} at {this.state.time}
                            </Card.Subtitle>
                            <Card.Subtitle className="text-muted mt-1">
                              last modified: {this.state.mod_time}
                            </Card.Subtitle>
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
