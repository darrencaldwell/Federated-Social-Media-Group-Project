import React from 'react';
import InputField from "../components/InputField";
import SubmitButton from "../components/SubmitButton";
import '../styling/Login.css'
// import UserStore from "../stores/UserStore";

class Login extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            username: '',
            password: '',
            buttonDisabled: false
        }

    }

    setInputValue(property, val) {
        val = val.trim();

        if (val.length > 12) {
            return;
        }
        this.setState({
            [property]: val
        })
    }

    resetForm() {
        this.setState({
            username: '',
            password: '',
            buttonDisabled: false
        })
    }

    async doRegister() {
        this.setState({
            buttonDisabled: true
        })

        try {
            await fetch('/api/users/register/{username}/{password}?', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    username: this.state.username,
                    password: this.state.password
                })
            });
        } catch (e) {
            console.log();
            this.resetForm();
        }
    }

    render() {
        return (
            <div className="register">
                <h1>User Registration</h1>
                <InputField
                    type='text'
                    placeholder='Username'
                    value={this.state.username ? this.state.username : ''}
                    onChange={(val) => this.setInputValue('username', val)}
                />

                <InputField
                    type='password'
                    placeholder='Password'
                    value={this.state.password ? this.state.password : ''}
                    onChange={(val) => this.setInputValue('password', val)}
                />


                <SubmitButton
                    text='Register'
                    disabled={this.state.buttonDisabled}
                    onClick={() => this.doRegister()}
                />
            </div>
        );
    }
}

export default Login;
