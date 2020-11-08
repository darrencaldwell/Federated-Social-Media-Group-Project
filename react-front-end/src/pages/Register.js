import React from 'react';
import InputField from "../components/InputField";
import SubmitButton from "../components/SubmitButton";

class Register extends React.Component {

    constructor(props) {
        super(props)

        this.state = {
            // email: "",
            username: "",
            password: "",
            buttonDisabled: false
        }
        this.handleSubmit=this.handleSubmit.bind(this)
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
            // email: '',
            username: '',
            password: '',
            buttonDisabled: false
        })
    }

    // emailhandler = (event) => {
    //     this.setState({
    //         email: event.target.value
    //     })
    // }

    usernamehandler = (event) => {
        this.setState({
            username: event.target.value
        })
    }
    passwordhandler = (event) => {
        this.setState({
            password: event.target.value
        })
    }

    handleSubmit = (event) => {
        alert(`${this.state.username}  Registered Successfully!`)
        console.log(this.state);
        this.setState({
            // email: "",
            username: "",
            password: '',
        })
        event.preventDefault()

    }

    doRegister() {
        this.resetForm()
    }

    render() {
        return (
            <div>

                <form onSubmit={this.handleSubmit}>
                    <h1>User Registration</h1>
                    {/*<InputField*/}
                    {/*    type='email'*/}
                    {/*    placeholder='Email'*/}
                    {/*    value={this.state.email ? this.state.email : ''}*/}
                    {/*    onChange={}*/}
                    {/*/>*/}
                    <InputField
                        type='username'
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
                        text='Login'
                        disabled={this.state.buttonDisabled}
                        onClick={() => this.doRegister()}
                    />
                </form>

            </div>
        );
    }
}

export default Register;