import React from 'react';
import axios from 'axios'
import {Redirect} from "react-router-dom";


class Login extends React.Component {
    state = {}

    handleSubmit = e => {
        e.preventDefault() // Prevent the browser refreshing

        // Store data to be sent to backend
        const data = {
            username: this.username,
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
                this.setState({
                    loggedIn: true
                })
                this.props.login()
            }).catch(err => {
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

        let error = ''

        if (this.state.message) {
            error = (
                <div className="alert alert-danger" role="alert">
                    {this.state.message}
                </div>
            )
        }
        return (
            <form onSubmit={this.handleSubmit}>
                <h3>Login</h3>

                {error} {/* // if invalid login. if valid then empty string shown*/}

                {/* Username Input */}
                <div className="form-group">
                    <label>Username</label>
                    <input type="text" className="form-control" placeholder="Username"
                           onChange={e => this.username = e.target.value}/>
                </div>

                {/* Password Input */}
                <div className="form-group">
                    <label>Password</label>
                    <input type="password" className="form-control" placeholder="Password"
                           onChange={e => this.password = e.target.value}/>
                </div>

                <button className="btn btn-primary btn-block">Login</button>
            </form>
        );
    }
}

export default Login;
