import React from "react";
import {Button, Card, Container, Nav, Form, FormGroup, ListGroup, InputGroup} from "react-bootstrap";
import {Link} from "react-router-dom";
import DisplayPicture from "../components/account/DisplayPicture";
import axios from 'axios'
import FormControl from "@material-ui/core/FormControl";

class Account extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            uploadedPicture: false,
            userInfo: []
        }
    }


    componentDidMount = async () => {
        axios.get('local/users/' + localStorage.getItem('userId'))
            .then(res => {
                this.setState({
                    userInfo: res.data
                })
            }).catch(err => {
            // alert("something went wrong")
        })
    }

    // send patch req to backend to update the user details
    editBio = async () => {
        const data = {
            username: this.state.userInfo.username,
            description: this.description
        }

        axios.patch('local/users/' + localStorage.getItem('userId'), data)
            .then(res => {
                this.setState({
                    username: this.state.userInfo.username,
                    description: this.description
                })
                alert("Successfully updated!")
            }).catch(err => {
                if (err.response) {
                    alert(err.response.message())
                }
            }
        )

    }

    editUname = async () => {
        const data = {
            username: this.username,
            description: this.state.userInfo.description
        }

        axios.patch('local/users/' + localStorage.getItem('userId'), data)
            .then(res => {
                this.setState({
                    username: this.username,
                    description: this.state.userInfo.description
                })
                alert("Successfully updated bio!")
            }).catch(err => {
                if (err.response) {
                    alert(err.response.message())
                }
            }
        )
    }


    render() {
        const date = new Date(this.state.userInfo.dateJoined * 1000)
        const forums_url = "/1/forums"
        let desc = "Type your bio here..";
        console.log('info',this.state.userInfo)
        if (this.state.userInfo.description !== null) {
            desc = this.state.userInfo.description
        }

        return (
            <Container>
                <Card.Title>Your Account</Card.Title>
                <DisplayPicture uploadedPicture={false}/>
                <Container className="bio">
                    <Form onSubmit={this.editBio}>
                        <FormGroup controlId="bio">
                            <Form.Label>Your Bio</Form.Label>
                            <Form.Control type="text" placeholder={desc}
                                          onChange={e => this.description = e.target.value}/>
                        </FormGroup>

                        <Button variant="light" type="submit">Update Bio</Button>
                    </Form>
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
                </Nav>
                <Card>
                    <Card.Body>
                        <Form.Label>Username</Form.Label>
                        <Form onSubmit={this.editUname}>
                            <InputGroup controlId="uname">
                                <Form.Control type="text" placeholder={this.state.userInfo.username}
                                              onChange={e => this.username = e.target.value}/>

                                {/*<Button variant="light" type="submit">Update Username</Button>*/}
                                <InputGroup.Append>
                                    <Button variant="light" type="submit">Update Username</Button>
                                </InputGroup.Append>
                            </InputGroup>


                            {/*<Button variant="light" type="submit">Update Username</Button>*/}
                        </Form>

                        {' '}
                        <ListGroup>
                             <ListGroup.Item>First name: {this.state.userInfo.firstName}</ListGroup.Item>
                             <ListGroup.Item>Last name: {this.state.userInfo.lastName}</ListGroup.Item>
                             <ListGroup.Item>User id: {this.state.userInfo.user_id}</ListGroup.Item>
                             <ListGroup.Item>Email: {this.state.userInfo.email}</ListGroup.Item>
                             <ListGroup.Item>Date joined: {date.toLocaleString()}</ListGroup.Item>
                        </ListGroup>
                        {/*<Card.Text>*/}
                        {/*    First name: {this.state.userInfo.firstName}*/}
                        {/*</Card.Text>*/}
                        {/*<Card.Text>*/}
                        {/*    Last name: {this.state.userInfo.lastName}*/}
                        {/*</Card.Text>*/}
                        {/*<Card.Text>*/}
                        {/*    User id: {localStorage.getItem('userId')}*/}
                        {/*</Card.Text>*/}
                        {/*<Card.Text>*/}
                        {/*    Email: {this.state.userInfo.email}*/}
                        {/*</Card.Text>*/}
                        {/*<Card.Text>*/}
                        {/*    Joined: {date.toLocaleString()}*/}
                        {/*</Card.Text>*/}
                        <Nav fill variant="tabs" defaultActiveKey="/">
                            <Nav.Item>
                                {/*<Link as={Link} variant-"light" to='/'>Return home</.Link>*/}
                                <Link to={'/'}><Button variant='light' as="input" type="button"
                                                       value="Return home"/>{' '}</Link>
                            </Nav.Item>
                            <Nav.Item>
                                <Link to={forums_url}><Button variant='secondary' as="input" type="button"
                                                              value="Go to Forums"/>{' '}</Link>
                            </Nav.Item>
                        </Nav>
                    </Card.Body>
                </Card>
            </Container>
        );
    }
}

export default Account
