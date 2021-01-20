import React from 'react';
import {Button, Container, Form, FormGroup} from 'react-bootstrap'
import '../styling/create-post.css'

//props: url
class Make extends React.Component {
    constructor(props) {
        super(props);
        this.changeTitle = this.changeTitle.bind(this);
        this.changeBody = this.changeBody.bind(this);
        const defaultBody = 'Put the body of your post here';
        const defaultTitle = 'Title';
        this.state = {
            buttonText: 'Create Post',
            defaultTitle: defaultTitle,
            titleText: defaultTitle,
            defaultBody: defaultBody,
            bodyText: defaultBody,
        };
    }

    submit() {
        // if no text has been entered, it will return to default before the button is pressed
        // don't worry about title if in comment mode
        if ((this.state.titleText === this.state.defaultTitle && !this.mode) ||
            this.state.bodyText === this.state.defaultBody) {
            alert('Please enter a title and body');
        } else {
            fetch(this.props.url, {
                method: "POST",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    "postTitle": this.state.titleText,
                    "postContents": this.state.bodyText,
                    "userId": localStorage.getItem('userId')
                })
            }).then(responseJson => {
                console.log(responseJson);
            }).catch(error => this.setState({
                message: "Error posting post: " + error
            }));
        }
    }

    changeTitle(v) {
        this.setState({titleText: v.target.value})
    }

    changeBody(v) {
        this.setState({bodyText: v.target.value})
    }

    render() {
        return (
            <Container>
                <Form className="createPost">
                    <Form.Label>Create Post</Form.Label>
                    <FormGroup controlId="create-title">
                        <Form.Control onChange={this.changeTitle} type="text" placeholder={this.state.defaultTitle}/>
                        <Form.Control onChange={this.changeBody} as="textarea" rows={3} placeholder={this.state.defaultBody}/>
                    </FormGroup>
                    <Button variant="light" onClick={() => this.submit()}>{this.state.buttonText}</Button>
                </Form>
            </Container>
        );
    }
}

export default Make
