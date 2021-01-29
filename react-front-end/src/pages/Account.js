import React from "react";
import {Button, Card, Container, Nav} from "react-bootstrap";
import {Link} from "react-router-dom";

class Account extends React.Component {
    render() {
        return (
            <Container>
                <Card.Title>Your Account</Card.Title>
                <Nav fill variant="tabs" defaultActiveKey="/">
                    <Nav.Item>
                        <Nav.Link as={Link} to='/'>About</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to='/posts'>Posts</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to='/'>Comments</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to='/'>Roles</Nav.Link>
                    </Nav.Item>
                </Nav>
                <Card>
                    <Card.Body>
                        <Card.Text>
                            Username: {localStorage.getItem('username')}
                        </Card.Text>
                        <Card.Text>
                            Joined: nothing for now
                        </Card.Text>
                        <Link to={'/'}><Button variant='light' as="input" type="button" value="Return home"/>{' '}</Link>
                    </Card.Body>
                </Card>
            </Container>
        );
    }
}

export default Account