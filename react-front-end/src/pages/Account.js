import React from "react";
import {Button, Card, Container} from "react-bootstrap";
import {Link} from "react-router-dom";

class Account extends React.Component {
    render() {
        return (
            <Container>
                <Card>
                    <Card.Img variant="top" src="" />
                    <Card.Body>
                        <Card.Title>Your Account</Card.Title>
                        <Card.Text>
                            Username: {localStorage.getItem('username')}
                        </Card.Text>
                        <Link to={'/'}><Button variant='light' as="input" type="button" value="Return home"/>{' '}</Link>
                    </Card.Body>
                </Card>
            </Container>
        );
    }
}

export default Account