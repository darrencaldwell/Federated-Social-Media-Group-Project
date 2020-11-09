import React from 'react';
import axios from 'axios'
import {Redirect} from "react-router-dom";


class Login extends React.Component {
    state = {}

    handleSubmit = e => {
        e.preventDefault()

        const data = {
            username: this.username,
            password: this.password
        }

        // api/users/login
        axios.post('login', data)
            .then(res => {
                localStorage.setItem('token', res.data.token)
                this.setState({
                    loggedIn: true
                })
                this.props.setUser(res.data.user)
            }).catch(err => {
                console.log(err)
        })
    }

    render() {

        if (this.state.loggedIn) {
            return <Redirect to={'/'}/>
        }

        return (
            <form onSubmit={this.handleSubmit}>
                <h3>Login</h3>

                <div className="form-group">
                    <label>Username</label>
                    <input type="text" className="form-control" placeholder="Username"
                           onChange={e => this.username = e.target.value}/>
                </div>

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
