import React, {Component} from 'react';
import {Container, Jumbotron} from "react-bootstrap";

import "../styling/forum.css"

// props: match.params.impID match.params.forumID
class Forum extends Component {

    constructor(props) {
        super(props);
        this.state = {
            forum: null,
        }
    }

    getForum = async () => {
        try {
            let url = `/api/forums/${this.props.match.params.forumID}`;
            let res = await fetch(url,
                {
                    method: 'get',
                    withCredentials: true,
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json',
                        'redirect': this.props.match.params.impID
                    }
                }
            );

            this.setState({forum: await res.json()});
        } catch (e) {
        }
    }

    componentDidUpdate = (prevProps) => {
        if (this.props.match.url !== prevProps.match.url) {
            this.getForum();
        }
    }

    // Runs when the component is loaded, fetching the list of implementations to load into state
    componentDidMount = async () => {
        this.getForum();
    }

    render() {
        if (this.state.forum === null) {
            return "loading";
        }

        return (
            <Jumbotron>
                <Container>
                    <h1>{`Welcome to ${this.state.forum.forumName}`}</h1>
                </Container>
            </Jumbotron>
        );
    }
}

export default Forum;
