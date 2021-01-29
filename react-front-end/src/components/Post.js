import React, {Component} from 'react';
//import { BrowserRouter as Router, Link } from 'react-router-dom';
import Comments from './Comments';
// import CreatePost from './CreatePost.js';
import CreateComment from './CreateComment'
// import '../styling/Post.css';
import {Button, Card, Container} from "react-bootstrap";

// props: postID
export class Post extends Component {

    constructor(props) {
        super(props);
        this.state = {
            post: {} // the post is stored here once loaded
        }
    }

    // Runs when the component is loaded, fetching the post into state
    componentDidMount = async () => {
        try {
            // the url needs the post id from the props
            let url = "/api/posts/${this.props.postID}";
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({post: result }); // we store the json for the post in the state

        } catch (e) {
        }
    }

    render() {
        const url = "/api/posts/" + this.props.post.postId + "/comments";
        
        return (
            <Container>
                <Button variant="light" onClick={() => this.props.loadPosts()}>Go back to post list</Button>
                <div className="mt-3">
                    <Card border="dark">
                        <Card.Body>
                            <Card.Title>{this.props.post.postTitle}</Card.Title>
                            <Card.Subtitle className="text-muted">Post made by user
                                Id: {this.props.post.postId}</Card.Subtitle>
                            <Card.Text>{this.props.post.postMarkup}</Card.Text>
                        </Card.Body>
                    </Card>
                </div>

                <CreateComment url={url}/>

                {/*<Dropdown className="mt-3">*/}
                {/*<Dropdown.Toggle variant="light" id="dropdown-comments">View Comments</Dropdown.Toggle>*/}
                {/*<Dropdown.Menu>*/}
                {this.props.comments.commentList.map((comment) => (
                    <Comments key={comment.id} comment={comment}></Comments>
                ))}
                {/*</Dropdown.Menu>*/}
                {/*</Dropdown>*/}


                </Container>
            )
        }
    }
}

export default Post
