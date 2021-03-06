import React from 'react';
import axios from 'axios'
import {Redirect} from "react-router-dom";
import {Form, FormGroup, Button, Container} from 'react-bootstrap'
import '../styling/forms.css';


class Login extends React.Component {
    state = {}

    handleSubmit = e => {
        e.preventDefault() // Prevent the browser refreshing

        // Store data to be sent to backend
        const data = {
            email: this.email,
            password: this.password
        }

        // api/users/login
        // login
        // POST data to backend and set and set the data in local storage
        axios.post('api/users/login', data)
            .then(res => {
                localStorage.setItem('token', res.data.token);
                localStorage.setItem('userId', res.data.userId);
                localStorage.setItem('username', res.data.username);
                localStorage.setItem('expr', res.data.exp);
                this.setState({
                    loggedIn: true
                })
                this.props.login()
            }).catch(err => {
            alert("Incorrect username or password")
            this.setState({
                message: err.response.data.message
            })
        })
    }

    render() {

        // If logged in, redirect to home page
        if (this.state.loggedIn) {
            return <Redirect to={'/'}/>
        }

        return (
            <Container>
                <Form className="login" onSubmit={this.handleSubmit}>
                    <h3>Login</h3>

                    {/* Username Input */}
                    <FormGroup>
                        <label>Email</label>
                        <input type="text" input="email" className="form-control" placeholder="Email"
                               onChange={e => this.email = e.target.value}/>
                    </FormGroup>

                    {/* Password Input */}
                    <FormGroup>
                        <label>Password</label>
                        <input type="password" input="current-password"  className="form-control" placeholder="Password"
                               onChange={e => this.password = e.target.value}/>
                    </FormGroup>

                    <Button variant="light" type="submit">Login</Button>
                </Form>
            </Container>
        );
    }
}

export default Login;
