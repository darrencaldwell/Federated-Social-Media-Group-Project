import React, {Component} from 'react';
import {Card, Button, ButtonGroup} from "react-bootstrap";
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
        this.delete = this.delete.bind(this);
        this.state = {}
    }

    delete(e) {
        e.preventDefault()
        
        if (window.confirm('Are you sure you wish to delete this comment?\n THIS CANNOT BE UNDONE!')) {
            // this is the HTML request
            fetch("/api/posts/" + this.props.post.id, {
                method: "DELETE",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                    'Content-Type': 'application/json',
                    'redirect': this.props.impID
                }

            }).then(responseJson => { // log the response for debugging
                console.log(responseJson);
                if (responseJson.status === 200) {
                    alert("Successfully deleted post.");
                    window.location.href = "/" + this.props.impID + "/" + this.props.forumID + "/" + this.props.subforumID + "/";
                }
            }).catch(error => this.setState({ // catch any error
                message: "Error deleting post: " + error
            }));
        }
    }

    render() {
        const parsed_user_link = btoa(this.props.post._links.user.href)

        return (
            <Card border="dark" className="mt-3 post-preview" >
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
                            
                        <CardActionArea style={{ textDecoration: 'none' }} 
                                        href={'/' + this.props.impID + '/' + this.props.forumID + '/' + this.props.subforumID + '/' + this.props.post.id}>
                            <Card.Body className="comment-body">
                                <Card.Title className="post-title"> {this.props.post.postTitle}</Card.Title>
                                <ReactMarkdown className="post-body">{this.props.post.postContents}</ReactMarkdown>     {/*Use the body from the prop as the body */}
                            </Card.Body>
                        </CardActionArea>

                        <ButtonGroup vertical className="buttons">
                            <ButtonGroup>
                                <Button className="button edit-button" title="Edit"
                                   href={"/" + this.props.impID + "/" + this.props.forumID + "/" + this.props.subforumID + "/" + this.props.post.id + "/edit"}>🖉</Button>
                                <Button className='button delete-button' title="Delete" onClick={this.delete} href="#">🗑</Button>
                            </ButtonGroup>
                            <a className="button reply-button" title="Comment"
                               href={"/" + this.props.impID + "/" + this.props.forumID + "/" + this.props.subforumID + "/" + this.props.post.id + "/new"}>Comment</a>
                        </ButtonGroup>
                    </div>
                </Card.Body>
            </Card>
        )
    }
}

export default PostPreview
