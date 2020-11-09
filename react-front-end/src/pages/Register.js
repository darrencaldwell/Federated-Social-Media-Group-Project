import React from 'react';

import axios from 'axios'

class Login extends React.Component {

    handleSubmit = e => {
        e.preventDefault()
        const data = {
            firstName: this.firstName,
            lastName: this.lastName,
            email: this.email,
            username: this.username,
            password: this.password,
            password_confirm: this.confirmPassword
        }

        // api/users/register
        axios.post('register', data)
            .then(res => {
                console.log(res)
            }).catch(
                err => {
                    console.log(err)
                }
        )
    }

    render() {
        return (
            <form onSubmit={this.handleSubmit}>
                <h3>Register</h3>
                <div className="form-group">
                    <label>First Name</label>
                    <input type="text" className="form-control" placeholder="First Name"
                           onChange={e => this.firstName = e.target.value}/>
                </div>

                <div className="form-group">
                    <label>Last Name</label>
                    <input type="text" className="form-control" placeholder="Last Name"
                           onChange={e => this.lastName = e.target.value}/>
                </div>

                <div className="form-group">
                    <label>Email</label>
                    <input type="email" className="form-control" placeholder="Email"
                           onChange={e => this.email = e.target.value}/>
                </div>

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

                <div className="form-group">
                    <label>Confirm Password</label>
                    <input type="password" className="form-control" placeholder="Confirm Password"
                           onChange={e => this.confirmPassword = e.target.value}/>
                </div>

                <button className="btn btn-primary btn-block">Register</button>
            </form>
        );
    }
}

export default Login;
