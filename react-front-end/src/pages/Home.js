import React from 'react';
import {Container, Jumbotron} from "react-bootstrap";

class Home extends React.Component {

    render() {

        // If username exists, get username from token and output message
        if (localStorage.getItem('username')) {
            return (
                <Jumbotron id="home">
                    <h1 className="display-3">Welcome {localStorage.getItem('username')}</h1>
                    <p className="lead">
                        The home page currently doesn't contain anything useful but hopefully
                        it will in the future.
                    </p>
                </Jumbotron>
            )
        }

        // Otherwise just return a home page
        return (
            <Container className="jumbotron" fluid>
                <Jumbotron>
                    <h1 className="display-3">Welcome</h1>
                    <p className="lead">
                        The home page currently doesn't contain anything useful but hopefully
                        it will in the future.
                    </p>
                </Jumbotron>
            </Container>
        );
    }
}

export default Home;
