import React, {Component} from 'react';
//import { BrowserRouter as Router, Link } from 'react-router-dom';
import Comments from './Comments';
// import CreatePost from './CreatePost.js';
import BackButton from './BackButton';
// import '../styling/Post.css';
import {Card, Container} from "react-bootstrap";

// props: match.params.impID, match.params.postID, match.params.subforumID, match.params.forumID, match.params.commentID
export class Post extends Component {

    constructor(props) {
        super(props);
        const expanded = (typeof this.props.match.params.commentID != 'undefined'); // it's an expanded comment if the url has the comment id
        this.state = {
            expanded: expanded,
            post: {} // the post is stored here once loaded
        }
    }

    // Runs when the component is loaded, fetching the post into state
    componentDidMount = async () => {
        try {
            // the url needs the post id from the props
            let url = '/api/posts/' + this.props.match.params.postID;
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json',
                        'redirect': this.props.match.params.impID
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({post: result }); // we store the json for the post in the state

        } catch (e) {
        }
    }

    render() {

        const backURL = "/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID;
        const url = this.state.expanded ? ('/api/comments/' + this.props.match.params.commentID + '/comments')
                                        : ('/api/posts/' + this.props.match.params.postID + '/comments/');

        return (
            <Container className="post-container">
                <BackButton url={backURL}/>
                <div className="mt-3">
                    <Card border="dark">
                        <Card.Body>
                            <Card.Title>{this.state.post.postTitle}</Card.Title>
                            <Card.Subtitle className="text-muted">
                                Post made by user Id: {this.state.post.id}</Card.Subtitle>
                            <Card.Text>{this.state.post.postContents}</Card.Text>
                        </Card.Body>
                    </Card>
                </div>

                <a className="button create-forum-button" href={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID + "/new"}>
                    Create Comment
                </a>

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
