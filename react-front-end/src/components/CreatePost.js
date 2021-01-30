import React from 'react';
import {Button, Container, Form, FormGroup} from 'react-bootstrap'
import BackButton from './BackButton';
import '../styling/create-post.css'

//props: subforumID
class Make extends React.Component {
    constructor(props) {
        super(props);
        this.changeTitle = this.changeTitle.bind(this); // bind these functions so they can override the onChange functions
        this.changeBody = this.changeBody.bind(this);
        // declare these as constants here so 2 different state attributes can be set to each
        const defaultBody = 'Put the body of your post here'; // the placeholder text in the body
        const defaultTitle = 'Title'; // the placeholder text for the title
        this.state = {
            buttonText: 'Create Post',
            defaultTitle: defaultTitle, // the default title needs to be preserved
            titleText: defaultTitle, // the title starts as the default
            defaultBody: defaultBody, // the default body needs to be preserved
            bodyText: defaultBody, // the body starts as the default
            url: '/api/subforums/' + this.props.subforumID + '/posts'
        };
    }

    submit() {
        // if no text has been entered, it will return to default before the button is pressed
        if ((this.state.titleText === this.state.defaultTitle) || this.state.bodyText === this.state.defaultBody) {
            alert('Please enter a title and body');
        } else {
            // the HTML request
            fetch(this.state.url, {
                method: "POST",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), // need to get the auth token from localStorage
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    "postTitle": this.state.titleText,
                    "postContents": this.state.bodyText,
                    "userId": localStorage.getItem('userId') // userId is a string in localStorage
                })
            }).then(responseJson => {
                console.log(responseJson);
            }).catch(error => this.setState({
                message: "Error posting post: " + error
            }));
        }
    }

    /* these two functions override the onChange functions for the title and body,
        updating state with the value for the submit function to use */

    changeTitle(v) {
        this.setState({titleText: v.target.value})
    }

    changeBody(v) {
        this.setState({bodyText: v.target.value})
    }

    render() {
        return (
            <Container>
                <BackButton/>
                <Form className="createPost">
                    <Form.Label>Create Post</Form.Label>
                    <FormGroup controlId="create-title">
                        {/*These are the input forms for title and body, with placeholder text. They call the above change methods when you type in them.*/}
                        <Form.Control onChange={this.changeTitle} type="text" placeholder={this.state.defaultTitle}/>
                        <Form.Control onChange={this.changeBody} as="textarea" rows={3} placeholder={this.state.defaultBody}/>
                    </FormGroup>
                    <Button variant="light" onClick={() => this.submit()}>{this.state.buttonText}</Button> {/*this button calls the submit function on click*/}
                </Form>
            </Container>
        );
    }
}

export default Make
