import React from 'react';
import axios from 'axios'
import {Redirect} from "react-router-dom";
import {Button, Container, Form, FormGroup} from "react-bootstrap";

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
        }

        const {password, confirmPassword} = this.state

        if (password !== confirmPassword) {
            alert("Passwords don't match")
        } else {
            // api/users/register
            // register
            // POST registration data to backend
            axios.post('api/users/register', data)
                .then(res => {
                    console.log("success")
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


    render() {
        // If registered successfully, Redirect to login page
        if (this.state.registered) {
            return <Redirect to={'/login'}/>
        }

        return (
            <Container>
                <Form className="register" onSubmit={this.handleSubmit}>
                    <h3>Register</h3>

                    <FormGroup>
                        <label>First Name</label>
                        <input type="text" className="form-control" placeholder="First Name"
                               onChange={e => this.firstName = e.target.value}/>
                    </FormGroup>

                    <FormGroup>
                        <label>Last Name</label>
                        <input type="text" className="form-control" placeholder="Last Name"
                               onChange={e => this.lastName = e.target.value}/>
                    </FormGroup>

                    <FormGroup>
                        <label>Email</label>
                        <input type="email" className="form-control" placeholder="Email"
                               onChange={e => this.email = e.target.value}/>
                    </FormGroup>

                    <FormGroup>
                        <label>Username</label>
                        <input type="text" className="form-control" placeholder="Username"
                               onChange={e => this.username = e.target.value}/>
                    </FormGroup>

                    <FormGroup>
                        <label>Password</label>
                        <input type="password" className="form-control" placeholder="Password"
                               onChange={e => this.password = e.target.value}/>
                    </FormGroup>

                    <FormGroup>
                        <label>Confirm Password</label>
                        <input type="password" className="form-control" placeholder="Confirm Password"
                               onChange={e => this.confirmPassword = e.target.value}/>
                    </FormGroup>

                    <Button variant="light" type="submit">Register</Button>
                </Form>
            </Container>
        );
    }
}

export default Register;
