import React, {Component} from 'react';
//import { BrowserRouter as Router, Link } from 'react-router-dom';
import Comments from '../components/Comments';
import CreatePost from './CreatePost.js';
import CreateComment from '../components/CreateComment'
// import '../styling/Post.css';
import {Button, Card, Container, Dropdown} from "react-bootstrap";

export class Post extends Component {
    render() {
        const url = "/api/posts/" + this.props.post.postId + "/comments";
        // If the list of comments is empty print out a message saying so else print out comments
        if (this.props.comments.commentList.length === 0) {
            return (
                <div>
                    <h1 className="postTitle">
                        {this.props.post.postTitle}
                    </h1>
                    <p className="postMarkup">
                        {this.props.post.postMarkup}
                    </p>
                    <Button variant="light" onClick={() => this.props.loadPosts()}>Go back to list of posts</Button>
                    <div>
                        <CreatePost mode="comment" url={url}/>
                    </div>
                    <div>
                        <h>Comments:</h>
                        <p>No comments have been made yet.</p>
                    </div>
                </div>
            )
        } else {
            return (
                <Container>
                    <Button variant="light" onClick={() => this.props.loadPosts()}>Go back to post list</Button>
                    <div className="mt-3">
                        <Card border="dark">
                            <Card.Body>
                                <Card.Title>{this.props.post.postTitle}</Card.Title>
                                <Card.Subtitle className="text-muted">Post made by user Id: {this.props.post.postId}</Card.Subtitle>
                                <Card.Text>{this.props.post.postMarkup}</Card.Text>
                            </Card.Body>
                        </Card>
                    </div>

                    <CreateComment url={url}/>

                    <Dropdown>
                        <Dropdown.Toggle variant="light" id="dropdown-comments">View Comments</Dropdown.Toggle>
                        <Dropdown.Menu>
                            {this.props.comments.commentList.map((comment) => (
                                <Comments key={comment.id} comment={comment}></Comments>
                            ))}
                        </Dropdown.Menu>
                    </Dropdown>


                </Container>
            )
        }
    }
}

export default Post
