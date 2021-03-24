import React, { Component } from 'react'
import {Card} from "react-bootstrap";
import {Container} from 'react-bootstrap';
import '../styling/container-pages.css';
import Avatar from 'react-avatar';
import Voting from './Voting'

// props: comment (json), posturl, impID, level
class Comment extends Component {

    render() {
        return (
                <Card border="dark">
                        <Card.Body>
                            <div class="post-comment-voting-container">
                                <Voting class="voting-post"
                                    upvotes={this.props.comment.upvotes} 
                                    downvotes={this.props.comment.downvotes} 
                                    _userVotes={this.props.comment._userVotes}
                                    type="comments"
                                    postID={this.props.comment.id}
                                    impID={this.props.impID}
                                ></Voting>
                                <div class="voting-adj">
                                    <Card.Text>
                                        <Avatar size="50" round={true} src={"/api/users/" + this.props.comment.userId + "/profilepicture"} name={this.props.comment.username}/> 
                                            {"  "} {this.props.comment.username}
                                    </Card.Text>
                                </div>
                            </div>
                            <Card.Text className="mt-3">{this.props.comment.commentContent}</Card.Text>
                            <Card.Link href={this.props.posturl + "/" + this.props.comment.id + "/new"}>Reply to {this.props.comment.username}</Card.Link>
                        </Card.Body>
                    <Comments url={"/api/comments/" + this.props.comment.id + "/comments"} impID={this.props.impID} posturl={this.props.posturl} level={this.props.level + 1} commentID={this.props.comment.id}/>
                </Card>
        )
    }
}

// props: url, posturl, impID, level, commentID
export default class Comments extends Component {
    constructor(props) {
        super(props);
        const root = (typeof this.props.level == 'undefined'); // it's a root comment if the comment ID is undefined
        const level = root ? (0)
                         : (this.props.level);
        this.state = {
            level: level,
            commentList: [] // the list of comments will be stored here
        }
    }

    // Runs when the component is loaded, fetching the list of comments into state
    componentDidMount = async () => {
        try {
            // the url to make the request to is given by the parent
            let url = this.props.url;
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect': this.props.impID
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({commentList: result._embedded.commentList }); // we store the json for the list of comments in the state

        } catch (e) {
            console.log(e);
        }
    }

    render() {
        if (this.state.expanded) {  // provide a link to return to the post
            return (
                <Container>
                    <a className="button" href={this.props.posturl}>Return</a>
                    <Comments url={this.props.url} impID={this.props.impID} expanded={false} posturl={this.props.posturl}/>
                </Container>
            )
        } else if (this.state.level >= 3) { // to prevent cramped elements due to heavy nesting
            return (
                <Container>
                    <a className="button" href={this.props.posturl + "/" + this.props.commentID}>Expand</a>
                </Container>
            )
        } else {

            // if there are comments, display them
            return(
                <Container>
                    {/*map is used to apply this html for each comment in the list */}
                    {this.state.commentList.map((comment) => (
                        // the Comment element above is used for this, which takes the comment json
                        <Comment comment={comment} impID={this.props.impID} level={this.state.level} posturl={this.props.posturl}/>
                    ))}
                </Container>
            )
        } 
    }
}
