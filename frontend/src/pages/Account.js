import React from "react";
import {Button, Card, Container, Nav} from "react-bootstrap";
import {Link} from "react-router-dom";
import DisplayPicture from "../components/account/DisplayPicture";
import axios from 'axios'

class Account extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            uploadedPicture: false,
            userInfo: []
        }
    }

    componentDidMount() {
        axios.get('local/users/' + localStorage.getItem('userId'))
            .then(res => {
                this.setState({
                    userInfo: res.data
                })
            }).catch(err => {
            alert("something went wrong")
        })
    }


    render() {
        const date = new Date(this.state.userInfo.dateJoined * 1000)
        const forums_url = "/1/forums"
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
                        <Nav.Link as={Link} to='/userposts'>Your Posts</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to='/usercomments'>Your Comments</Nav.Link>
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
                            First name: {this.state.userInfo.firstName}
                        </Card.Text>
                        <Card.Text>
                            Last name: {this.state.userInfo.lastName}
                        </Card.Text>
                        <Card.Text>
                            User id: {localStorage.getItem('userId')}
                        </Card.Text>
                        <Card.Text>
                            Email: {this.state.userInfo.email}
                        </Card.Text>
                        <Card.Text>
                            Joined: {date.toLocaleString()}
                        </Card.Text>
                        <Card.Text>
                            Total Subscribed Forums:
                        </Card.Text>
                        <Nav fill variant="tabs" defaultActiveKey="/">
                            <Nav.Item>
                                {/*<Link as={Link} variant-"light" to='/'>Return home</.Link>*/}
                                <Link to={'/'}><Button variant='light' as="input" type="button" value="Return home"/>{' '}</Link>
                            </Nav.Item>
                            <Nav.Item>
                                <Link to={forums_url}><Button variant='secondary' as="input" type="button" value="Go to Forums"/>{' '}</Link>
                            </Nav.Item>
                        </Nav>
                    </Card.Body>
                </Card>
            </Container>
        );
    }
}

export default Account
