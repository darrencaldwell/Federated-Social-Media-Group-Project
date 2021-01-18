import React, {useState} from 'react';
import axios from 'axios'
import {Redirect} from "react-router-dom";
import {Alert, Button, Container, Form, FormGroup} from "react-bootstrap";
import {set} from "mobx";

class Register extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            registered: false,
            password: '',
            confirmPassword: ''
        }; // set initial state
    }

    handleSubmit = e => {
        e.preventDefault()
        const data = {
            firstName: this.firstName,
            lastName: this.lastName,
            email: this.email,
            username: this.username,
            password: this.password,
            confirmPassword: this.confirmPassword,
        }

        if ((data.password !== data.confirmPassword)) {
            alert("Passwords don't match")
        } else {
            axios.post('api/users/register', data)
            .then(res => {
                this.setState({
                    registered: true
                })
            }).catch(
            err => {
                this.setState({
                    message: err.response.data.message
                })
            }
        )
    }
}


render()
{
    // If registered successfully, Redirect to login page
    if (this.state.registered) {
        return <Redirect to={'/login'}/>
    }

    return (
        <Container>
            <Form className="register" onSubmit={this.handleSubmit}>
                <h3>Register</h3>

                <FormGroup controlId="fname">
                    <Form.Label>First Name</Form.Label>
                    <Form.Control type="text" placeholder="First Name"
                                  onChange={e => this.firstName = e.target.value}/>
                </FormGroup>

                <FormGroup controlId="lname">
                    <label>Last Name</label>
                    <Form.Control type="text" placeholder="Last Name"
                                  onChange={e => this.lastName = e.target.value}/>
                </FormGroup>

                <FormGroup controlId="email">
                    <Form.Label>Email</Form.Label>
                    <Form.Control type="email" placeholder="Email"
                                  onChange={e => this.email = e.target.value}/>
                    <Form.Text className="text-muted">We'll never share your email with anyone else</Form.Text>
                </FormGroup>

                <FormGroup controlId="uname">
                    <Form.Label>Username</Form.Label>
                    <Form.Control type="text" placeholder="Username"
                                  onChange={e => this.username = e.target.value}/>
                </FormGroup>

                <FormGroup controlId="pword">
                    <Form.Label>Password</Form.Label>
                    <Form.Control type="password" className="form-control" placeholder="Password"
                                  onChange={e => this.password = e.target.value}/>
                </FormGroup>

                <FormGroup controlId="cpword">
                    <Form.Label>Confirm Password</Form.Label>
                    <Form.Control type="password" placeholder="Confirm Password"
                                  onChange={e => this.confirmPassword = e.target.value}/>
                </FormGroup>

                <Button variant="light" type="submit">Register</Button>
            </Form>
        </Container>
    );
}
}

export default Register;
