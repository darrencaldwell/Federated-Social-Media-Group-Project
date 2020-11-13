import React from 'react';

class Title extends React.Component {
    constructor(props) {
        super(props);
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(e) {
        this.props.onChange(e.target.value);
    }

    handleBlur() {
        if (this.props.value === '') {
            this.props.onChange(this.props.default);
        }
    }

    handleFocus() {
        if (this.props.value == this.props.default) {
            this.props.onChange('');
        }
    }

    render() {
        const value = this.props.value;
        return (
            <div>
                <input
                    value={value}
                    onChange={this.handleChange}
                    onFocus={() => this.handleFocus()}
                    onBlur={() => this.handleBlur()}/>
            </div>
        );
    }
}

class Body extends React.Component {
    constructor(props) {
        super(props);
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(e) {
        this.props.onChange(e.target.value);
    }

    handleBlur() {
        if (this.props.value === '') {
            this.props.onChange(this.props.default);
        }
    }

    handleFocus() {
        if (this.props.value == this.props.default) {
            this.props.onChange('');
        }
    }

    render() {
        return (
            <div>
    <textarea
        className="text" value={this.props.value} onChange={this.handleChange}
        onFocus={() => this.handleFocus()}
        onBlur={() => this.handleBlur()}>
    </textarea>
            </div>
        );
    }
}

function SendButton(props) {
    return (
        <button
            className="sendButton"
            onClick={(props.onClick)}>
            {props.value}
        </button>
    );
}
//props: url, mode
class Make extends React.Component {
    constructor(props) {
        super(props);
        this.changeTitle = this.changeTitle.bind(this);
        this.changeBody = this.changeBody.bind(this);
        this.mode = this.props.mode === "comment";
        const defaultBody = this.mode ? 'Put the body of your comment here' : 'Put the body of your post here';
        const defaultTitle = 'Title';
        this.state = {
            buttonText: this.mode ? 'Create Comment' : 'Create Post',
            defaultTitle: defaultTitle,
            titleText: defaultTitle,
            defaultBody: defaultBody,
            bodyText: defaultBody,
        };
    }

    submit () {
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
                body: this.mode ? JSON.stringify(
                    {
                        "commentContent": this.state.bodyText,
                        "userId": parseInt(localStorage.getItem('userId')),
                        "username":localStorage.getItem('username')
                    }
                ) : JSON.stringify({
                    "postTitle": this.state.titleText,
                    "postMarkup": this.state.bodyText,
                    "userId": parseInt(localStorage.getItem('userId'))
                })
            }).then(responseJson => {
                console.log(responseJson);
            }).catch(error => this.setState({
                message: "Error posting post: " + error
            }));
        }
    }

    changeTitle(v) {
        this.setState({titleText: v})
    }

    changeBody(v) {
        this.setState({bodyText: v})
    }

    renderTitle() {
        return ( this.mode ? null :
            <Title
                value={this.state.titleText}
                default={this.state.defaultTitle}
                onChange={this.changeTitle}
            />
        );
    }

    renderText() {
        return (
            <Body
                value={this.state.bodyText}
                default={this.state.defaultBody}
                onChange={this.changeBody}
            />
        );
    }

    renderButton() {
        return (
            <SendButton
                value={this.state.buttonText}
                onClick={() => this.submit()}
            />
        );
    }

    render() {
        return (
            <form>
                <h3>Make Posts</h3>
                <div>
                    <div className="title">
                        {this.renderTitle()}
                    </div>

                    <div className="text">
                        {this.renderText()}
                    </div>

                    <div className="sendButton">
                        {this.renderButton()}
                    </div>

                </div>
            </form>
        );
    }
}

export default Make
