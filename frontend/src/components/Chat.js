import React, {Component} from 'react';
import {Card, Container, Row, Col, Form} from 'react-bootstrap';
//import { BrowserRouter as Router, Link } from 'react-router-dom';
// import CreatePost from './CreatePost.js';
// import '../styling/Post.css';
import '../styling/chat.css';

class Message {
    constructor(type, message, userId, username, timestamp) {
        this.type = type;
        this.message = message;
        this.userId = userId;
        this.username = username;
        this.timestamp = timestamp;
    }
}

// props: match.params.impID, match.params.forumID
export class Chat extends Component {

    constructor(props) {
        super(props);

        this.state = {
            ws: null,
            messages: [],
            userList: new Map(),
            message: "",
        };
    }

    // Runs when the component is loaded, fetching the post into state
    componentDidMount = async () => {
        this.connect();
    }

    componentWillUnmount = async () => {
        if (this.state.ws != null) {
            this.state.ws.close();
        }
    }

    componentDidUpdate = (prevProps) => {
        if (this.props.match.url !== prevProps.match.url) {
            this.state.ws.close();
            this.setState({messages: [], userList: new Map(), message: ""})
            this.connect();
        }
    }

    handleMessage = (msg) => {
        console.log(msg);
        let message = JSON.parse(msg);
        let that = this.state;
        console.log(message);

        switch (message.message_type) {
            case "UserList":
                that.userList = new Map(message.user_list.map(obj => [obj.user_id, obj.user_name]));
                break
            case "Connect":
                that.userList.set(message.user_id, message.user_name)
                break;
            case "Disconnect":
                that.userList.delete(message.user_id);
                break;
            case "Whisper":
                break;
            default:
                let time = new Date().toLocaleTimeString();
                let temp = new Message(message.message_type, message.content, message.user_id, message.user_name, time);
                that.messages.push(temp);
        }

        this.setState(that);
        console.log(this.state);
    }

    connect = () => {
        console.log(localStorage.getItem("token"));
        //let ws = new WebSocket("ws://localhost:21450/local/forums/" + this.props.match.params.forumID + "/chat", localStorage.getItem("token"));
        //let ws = new WebSocket("ws://localhost:25565");
        let ws = new WebSocket(((window.location.protocol === "https:") ? "wss://" : "ws://") +
            window.location.host + "/local/forums/" + this.props.match.params.forumID + "/chat", localStorage.getItem('token'));
        let that = this;
        var connectInterval;

        ws.onopen = async () => {
            console.log("connected")

            this.setState({ws: ws});

            that.timeout = 250;
            clearTimeout(connectInterval);
        };

        ws.onclose = e => {
            console.log("Socket closed, attempting to reconnect", e.reason);
            that.timeout = that.timeout + that.timeout;
            connectInterval = setTimeout(this.check, Math.min(10000, that.timeout))
        }

        ws.onerror = err => {
            console.log("Error: ", err.message);
            ws.close();
        };

        ws.onmessage = event => {
            console.log(event);
            this.handleMessage(event.data);
        };
    };

    check = () => {
        const { ws } = this.state;
        if (!ws || ws.readystate === WebSocket.CLOSED) this.connect();
    };

    sendMessage = (e) => {
        if (e.keyCode === 13 && this.state.message != null && this.state.message.length !== 0) {
            this.state.ws.send(this.state.message);
            let time = new Date().toLocaleTimeString();
            let temp = new Message("Self", this.state.message, "You Sent", "You sent", time);
            this.state.messages.push(temp)
            let that = this;
            that.state.message = "";
            this.setState(that);
        }
    };

    onChange = (e) => {
        let that = this;
        that.state.message = e.target.value;
        this.setState(that);
    };

    renderMessage = (message) => {
        switch (message.type) {
        case "Self":
            return (
                <Card className="sent">
                    <Card.Body className="sent-body">
                        <Card.Title className="sent-title">{message.username}</Card.Title>
                        <Card.Text>{message.message}</Card.Text>
                    </Card.Body>
                </Card>
            );
        case "Message":
            return (
                <Card className="rec">
                    <Card.Body className="rec-body">
                        <Card.Title className="rec-title">{message.username}</Card.Title>
                        <Card.Text>{message.message}</Card.Text>
                    </Card.Body>
                </Card>
            );
        case "Server":
            return (
                <Card className="server">
                    <Card.Body className="server-body">
                        <Card.Text>{message.message}</Card.Text>
                    </Card.Body>
                </Card>
            );
        default:
            return "";
        }
    };

    renderUserList = () => {
        let list = [];
        this.state.userList.forEach((name, _) => {list.push(name)});
        return list
    }

    render() {
        if (this.state.ws == null) {
            return "Loading";
        } else {
            return (
            <Container>
                <Row>
                <Col xs={9}>
                    <Row>
                        <Container className="messages">
                        {this.state.messages.map(this.renderMessage)}
                        </Container>
                    </Row>
                    <Row className="input">
                        <Form.Control placeholder="Send a message" class="textbox" type="text" onKeyDown={(e) => this.sendMessage(e)} onChange={this.onChange} value={this.state.message}/>
                    </Row>
                </Col>
                <Col xs={3}>
                    <div>
                        <h1>User List:</h1>
                        <body class="list">{this.renderUserList()}</body>
                    </div>
                </Col>
                </Row>
            </Container>
            );
        }
    }
}

export default Chat;
