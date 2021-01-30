import React from 'react';
import {Button, Container, Form, FormGroup} from 'react-bootstrap'
import BackButton from './BackButton';
import '../styling/create-post.css'

//props: forumID
class Make extends React.Component {
    constructor(props) {
        super(props);
        this.changeTitle = this.changeTitle.bind(this); // bind these functions so they can override the onChange functions
        // declare these as constants here so 2 different state attributes can be set to each
        const defaultTitle = 'Put your subforum name here'; // the placeholder text for the title

        // JS seems to be weird with concat on undefined variables, this seemed to fix the undef issues
        var url = ""
        url = '/api/forums/' + this.props.forumID + '/subforums'
        console.log(url)
        this.state = {
            buttonText: 'Create Subforum',
            defaultTitle: defaultTitle, // the default title needs to be preserved
            titleText: defaultTitle, // the title starts as the default
            url: url
        };
            console.log(this.props.forumID)
            console.log(this.url)
    }

    submit() {
        // if no text has been entered, it will return to default before the button is pressed
        if (this.state.titleText === this.state.defaultTitle) {
            alert('Please enter a title');
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
                    "subforumName": this.state.titleText,
                    "forumId": this.props.forumId
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

    render() {
        return (
            <Container>
                <BackButton url={"/" + this.props.forumID}/>
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
