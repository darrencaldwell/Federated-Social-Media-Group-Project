import React from 'react';
import {Button, Container, Form, FormGroup} from 'react-bootstrap'
import '../styling/create-post.css'

//props: url
class Make extends React.Component {
    constructor(props) {
        super(props);
        this.changeBody = this.changeBody.bind(this); // bind this so it can override onChange
        const defaultBody = 'Put the body of your comment here';
        this.state = {
            buttonText: 'Create Comment',
            defaultBody: defaultBody, // default body needs to be preserved
            bodyText: defaultBody, // body starts as default
        };
    }

    submit() {
        // if no text has been entered, it will return to default before the button is pressed
        if (this.state.bodyText === this.state.defaultBody) {
            alert('Please enter a title and body');
        } else {
            // this is the HTML request
            fetch(this.props.url, {
                method: "POST",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                    'Content-Type': 'application/json'
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
        return (
            <Container>
                <Form className="createComment">
                    <Form.Label>Create Comment</Form.Label>
                    <FormGroup controlId="create-title">
                        {/*Clicking in this box removes the placeholder, typing in it calls the change function to update state*/}
                        <Form.Control onChange={this.changeBody} as="textarea" rows={3} placeholder={this.state.defaultBody}/>
                    </FormGroup>
                    <Button variant="light" onClick={() => this.submit()}>{this.state.buttonText}</Button> {/*Clicking this button calls the submit method*/}
                </Form>
            </Container>
        );
    }
}

export default Make
