import React from "react";
import {Button, Card, Container, Nav} from "react-bootstrap";
import {Link} from "react-router-dom";
import DisplayPicture from "../components/account/DisplayPicture";
import axios from 'axios'

class Account extends React.Component {

    constructor() {
        super();
        this.state = {
            uploadedPicture: false
        }
    }

    componentDidMount() {
        axios.get('local/users/{id}')
            .then(res => {
                console.log(res)
            }).catch(err => {
                alert("something went wrong")
        })
    }


    render() {
        return (
            <Container>
                <Card.Title>Your Account</Card.Title>
                <DisplayPicture uploadedPicture={false}/>
                <Container className="bio">
                    <Card.Title>Your Bio</Card.Title>
                    <Card>
                        <Card.Text>This is where the bio would go</Card.Text>
                    </Card>
                </Container>

                <Nav fill variant="tabs" defaultActiveKey="/">
                    <Nav.Item>
                        <Nav.Link as={Link} to='/'>About Us</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to='/'>Your Posts</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to='/'>Your Comments</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to='/'>Your Roles</Nav.Link>
                    </Nav.Item>
                </Nav>
                <Card>
                    <Card.Body>
                        <Card.Text>
                            Username: {localStorage.getItem('username')}
                        </Card.Text>
                        <Card.Text>
                            First name: {this.first_name}
                        </Card.Text>
                        <Card.Text>
                            Last name: {this.last_name}
                        </Card.Text>
                        <Card.Text>
                            Username: {localStorage.getItem('username')}
                        </Card.Text>
                        <Card.Text>
                            User id: {localStorage.getItem('userid')}
                        </Card.Text>
                        <Card.Text>
                            Email: {localStorage.getItem('email')}
                        </Card.Text>
                        <Card.Text>
                            Joined:
                        </Card.Text>
                        <Card.Text>
                            Total Subscribed Forums:
                        </Card.Text>
                        <Link to={'/'}><Button variant='light' as="input" type="button" value="Return home"/>{' '}</Link>
                    </Card.Body>
                </Card>
            </Container>
        );
    }
}

export default Account