import React from 'react';
import {Button, Container, Form, FormGroup} from 'react-bootstrap'
import BackButton from './BackButton';
import '../styling/create-post.css'

// props: match.params.impID
class Make extends React.Component {
    constructor(props) {
        super(props);
        this.changeTitle = this.changeTitle.bind(this); // bind these functions so they can override the onChange functions
        // declare these as constants here so 2 different state attributes can be set to each
        const defaultTitle = 'Put your forum name here'; // the placeholder text for the title
        this.state = {
            buttonText: 'Create Forum',
            defaultTitle: defaultTitle, // the default title needs to be preserved
            titleText: defaultTitle, // the title starts as the default
            url : '/api/forums',
            backURL: "/" + this.props.match.params.impID + "/forums"
        };
    }

    move(id){ // Move to a forum given an id
        this.props.history.push("/" + this.props.match.params.impID + "/" + id);
    }

    submit() {
        // if no text has been entered, it will return to default before the button is pressed
        if (this.state.titleText === this.state.defaultTitle || this.state.titleText === "") {
            alert('Please enter a title');
        } else {
            // the HTML request
            fetch(this.state.url, {
                method: "POST",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), // need to get the auth token from localStorage
                    'Content-Type': 'application/json',
                    'redirect': this.props.match.params.impID
                },
                body: JSON.stringify({
                    "forumName": this.state.titleText
                })
            }).then(responseJson => {
                console.log(responseJson);
                responseJson.json().then(forum => {
                    // this.move(forum.id);
                    window.location.href = "/" + this.props.match.params.impID + "/" + forum.id;
                })
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

    render() {
        return (
            <Container>
                <BackButton url={this.state.backURL}/>
                <Form className="createForum">
                    <FormGroup controlId="create-title">
                        {/*These are the input forms for title and body, with placeholder text. They call the above change methods when you type in them.*/}
                        <Form.Control onChange={this.changeTitle} type="text" placeholder={this.state.defaultTitle}/>
                    </FormGroup>
                    <Button variant="light" onClick={() => this.submit()}>{this.state.buttonText}</Button> {/*this button calls the submit function on click*/}
                </Form>
            </Container>
        );
    }
}

export default Make
