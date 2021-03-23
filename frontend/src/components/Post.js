import React, {Component} from 'react';
//import { BrowserRouter as Router, Link } from 'react-router-dom';
import Comments from './Comments';
// import CreatePost from './CreatePost.js';
import BackButton from './BackButton';
// import '../styling/Post.css';
import {Card, Container, Spinner} from "react-bootstrap";
import Voting from './Voting';

// props: match.params.impID, match.params.postID, match.params.subforumID, match.params.forumID, match.params.commentID
export class Post extends Component {

    constructor(props) {
        super(props);
        const expanded = (typeof this.props.match.params.commentID != 'undefined'); // it's an expanded comment if the url has the comment id
        this.state = {
            expanded: expanded,
            loading: true, // Set to true if loading
            post: {}, // the post is stored here once loaded
            post_author: {}
        }
    }

    // Runs when the component is loaded, fetching the post into state
    componentDidMount = async () => {
        // get post
        try {
            this.setState({loading: true});
            // the url needs the post id from the props
            let url = '/api/posts/' + this.props.match.params.postID;
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect': this.props.match.params.impID
                    }
                }
            );

            let result_post = await res.json(); // we know the result will be json

            // the url needs the post id from the props
            url = '/api/users/' + result_post.userId;
            res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect': this.props.match.params.impID
                    }
                }
            );

            let result_auth = await res.json(); // we know the result will be json
            this.setState({post: result_post, post_author: result_auth, loading: false }); // we store the json for the post in the state

        } catch (e) {
            console.log(e)
        }
    }

    render() {
        // styling for the loading spinner - should be moved to a separate styling file if possible
        const spinnerStyle = { position: "fixed", top: "50%", left: "50%", transform: "translate(-50%, -50%)" }

        if (this.state.loading) {
            // while loading, show a loading spinner with the above style
            return (
                <Spinner animation="border" role="status" style={spinnerStyle}>
                    <span className="sr-only">Loading...</span>
                </Spinner>
            )
        }

        const backURL = "/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID;
        const url = this.state.expanded ? ('/api/comments/' + this.props.match.params.commentID + '/comments')
                                        : ('/api/posts/' + this.props.match.params.postID + '/comments');

        return (
            <Container className="post-container">
                <BackButton url={backURL}/>
                <div className="mt-3">
                    <Card border="dark">
                        <Card.Body>
                            <Voting 
                                upvotes={this.state.post.upvotes} 
                                downvotes={this.state.post.downvotes} 
                                _userVotes={this.state.post._userVotes}
                                type="posts"
                                postID={this.props.match.params.postID}
                                impID={this.props.match.params.impID}
                            ></Voting>
                            <Card.Title>{this.state.post.postTitle}</Card.Title>
                            <Card.Subtitle className="text-muted">
                                Post made by: {this.state.post_author.username} on TIME
                            </Card.Subtitle>
                            <Card.Text>{this.state.post.postContents}</Card.Text>
                            <Card.Link href={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID + "/new"}> Create Comment</Card.Link>
                        </Card.Body>
                    </Card>
                </div>

                {/*<CreateComment url={url}/>*/}

                {/*<Dropdown className="mt-3">*/}
                {/*<Dropdown.Toggle variant="light" id="dropdown-comments">View Comments</Dropdown.Toggle>*/}
                {/*<Dropdown.Menu>*/}
                <Comments url={url} impID={this.props.match.params.impID} expanded={this.state.expanded}
                        posturl={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID}/>
                {/*</Dropdown.Menu>*/}
                {/*</Dropdown>*/}

            </Container>
        )
    }
}
export default Post