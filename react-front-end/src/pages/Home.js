import React from 'react';
import {Container, Jumbotron, Button} from "react-bootstrap";
import {Link} from 'react-router-dom'


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
                    <Link to={'/account'}><Button variant='light' as="input" type="button" value="Go to your account"/>{' '}</Link>
                    <Link to={'/forums'}><Button variant='light' as="input" type="button" value="Go to forums"/>{' '}</Link>
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
                    <Link to={'/login'}><Button variant='light' as="input" type="button"
                                                value="Go to login page"/>{' '}</Link>
                    <Link to={'/register'}><Button variant='light' as="input" type="button"
                                                   value="Go to register page"/></Link>
                </Jumbotron>
            </Container>
        );
    }
}

export default Home;
