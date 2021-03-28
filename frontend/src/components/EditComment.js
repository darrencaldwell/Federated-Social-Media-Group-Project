import React from 'react';
import {Button, Container, Form, FormGroup} from 'react-bootstrap'
import BackButton from "./BackButton";
import '../styling/create-post.css'

//props: match.params.impID, match.params.postID, match.params.commentID
class Make extends React.Component {
    constructor(props) {
        super(props);
        this.changeBody = this.changeBody.bind(this); // bind this so it can override onChange
        const url = ("/api/comments/" + this.props.match.params.commentID);
        this.state = {
            buttonText: 'Create Comment',
            bodyText: "", // body starts as empty
            url: url
        };
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
            this.setState({bodyText: result.commentContent }); // we store the json for the list of comments in the state

        } catch (e) {
            console.log(e);
        }
    }

    submit() {
        // if no text has been entered, it will return to default before the button is pressed
        if (this.state.bodyText === this.state.defaultBody) {
            alert('Please enter a body');
        } else {
            // this is the HTML request
            fetch(this.state.url, {
                method: "PATCH",
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
            }).catch(error => this.setState({ // catch any error
                message: "Error posting post: " + error
            }));
        }
    }

    changeBody(v) { // update state with the new value of the body
        this.setState({bodyText: v.target.value})
    }

    render() {
        const backURL = "/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID;

        return (
            <Container>
                <BackButton url={backURL}/>
                <Form className="createComment">
                    <Form.Label>Create Comment</Form.Label>
                    <FormGroup controlId="create-title">
                        {/*Clicking in this box removes the placeholder, typing in it calls the change function to update state*/}
                        <Form.Control onChange={this.changeBody} as="textarea" rows={3} value={this.state.bodyText}/>
                    </FormGroup>
                    <Button variant="light" onClick={() => this.submit()}>{this.state.buttonText}</Button> {/*Clicking this button calls the submit method*/}
                </Form>
            </Container>
        );
    }
}

export default Make
