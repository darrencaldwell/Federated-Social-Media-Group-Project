import React, {Component} from 'react';
//import { BrowserRouter as Router, Link } from 'react-router-dom';
// import CreatePost from './CreatePost.js';
import BackButton from './BackButton';
// import '../styling/Post.css';
import {Card, Container} from "react-bootstrap";
import styles from '../styling/chat.css';

// props: match.params.impID, match.params.forumID
export class Chat extends Component {

    constructor(props) {
        super(props);

        this.state = {
            ws: null,
            messages: [],
        };
    }

    // Runs when the component is loaded, fetching the post into state
    componentDidMount = async () => {
        this.connect();
    }

    connect = () => {
        console.log(localStorage.getItem("token"));
        let ws = new WebSocket("ws://localhost:21450/local/forums/" + this.props.match.params.forumID + "/chat", localStorage.getItem("token"));
        //let ws = new WebSocket("ws://localhost:25565");
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
            console.log("Message: ", event);
            that.state.messages.push(event.data);
            console.log(this.state.messages);
            this.setState(that)
        };
    };

    check = () => {
        const { ws } = this.state;
        if (!ws || ws.readystate === WebSocket.CLOSED) this.connect();
    };

    render() {
        if (this.state.ws == null) {
            return "Loading";
        } else {
            return (
                <div>
                    <div>
                        {this.state.messages.map(item => (
                            <div class="myclass"> {item} </div>
                        ))}
                    </div>
                    <div>
                        <input type="text" id="message" onChange={e => this.message = e.target.value}/>
                        <input type="button" onClick={() => this.state.ws.send(this.message)} value="send message"/>
                    </div>
                </div>
            );
        }
    }
}

export default Chat;
