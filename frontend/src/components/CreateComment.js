import React from 'react';
import {Button, Container, Form, FormGroup, Card} from 'react-bootstrap'
import BackButton from "./BackButton";
import '../styling/create-post.css'
import ReactMarkdown from 'react-markdown';

//props: match.params.impID, match.params.postID, match.params.commentID
class Make extends React.Component {
    constructor(props) {
        super(props);
        this.changeBody = this.changeBody.bind(this); // bind this so it can override onChange
        const root = (typeof this.props.match.params.commentID == 'undefined'); // it's a root comment if the comment ID is undefined
        const url = root ? ("/api/posts/"    + this.props.match.params.postID    + "/comments")
                         : ("/api/comments/" + this.props.match.params.commentID + "/comments");

        const postURL = "/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" // URL of post comment belongs to
                         + this.props.match.params.subforumID + "/" + this.props.match.params.postID 
        const backURL =  postURL; // structured this way so that if there is a way to check how deep a comment is use ? : to go back to that comment.
        const defaultBody = 'Put the body of your comment here';
        this.state = {
            buttonText: 'Create Comment',
            defaultBody: defaultBody, // default body needs to be preserved
            bodyText: defaultBody, // body starts as default
            backURL: backURL, // Previous URL
            url: url
        };
    }

    goTo(x) { // Goes to the url argument then reloads to take updates into account
        this.props.history.push(x);
    }

    submit() {
        // if no text has been entered, it will return to default before the button is pressed
        if (this.state.bodyText === this.state.defaultBody || this.state.bodyText === "") {
            alert('Please enter a body');
        } else {
            // this is the HTML request
            fetch(this.state.url, {
                method: "POST",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                    'Content-Type': 'application/json',
                    'redirect': this.props.match.params.impID
                },
                body: JSON.stringify(
                    {
                        "commentContent": this.state.bodyText,
                        "userId": localStorage.getItem('userId'), //userId and username are strings in localStorage
                        "username": localStorage.getItem('username')
                    }
                )

            }).then(responseJson => { // log the response for debugging
                console.log(responseJson);
                if (responseJson.status === 200) {
                    alert("Successfully created comment");
                    window.location.href = this.state.backURL;
                }
            }).catch(error => this.setState({ // catch any error
                message: "Error posting post: " + error
            }));
            this.goTo(this.state.backURL);
        }
    }

    changeBody(v) { // update state with the new value of the body
        this.setState({bodyText: v.target.value})
    }

    render() {
        return (
            <div className="create-edit-page">
            <BackButton url={this.state.backURL}/>
            {/* <div className="edit-box"> */}
            <Container>
                <div className="separator"/>
                <Form className="createComment">
                    <Form.Label>Create Comment</Form.Label>
                    <FormGroup controlId="create-title">
                        {/*Clicking in this box removes the placeholder, typing in it calls the change function to update state*/}
                        <Form.Control onChange={this.changeBody} as="textarea" rows={3} placeholder={this.state.defaultBody}/>
                    </FormGroup>
                    <Button variant="light" onClick={() => this.submit()}>{this.state.buttonText}</Button> {/*Clicking this button calls the submit method*/}
                </Form>
                <Form className="preview">
                    <Form.Label>Preview:</Form.Label>
                    <Card>
                        <ReactMarkdown className="mt-3 comment-body">{this.state.bodyText}</ReactMarkdown>
                    </Card>
                </Form>
            </Container>
            {/* </div> */}
            </div>
        );
    }
}

export default Make
