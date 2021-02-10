import React from "react";
import {Button, Card, Container, Nav} from "react-bootstrap";
import {Link} from "react-router-dom";
import DisplayPicture from "../components/account/DisplayPicture";
// import axios from 'axios'

class Account extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            uploadedPicture: false,
            detail: {}
        }
    }

    // componentDidMount() {
    //     axios.get('local/users/{id}')
    //         .then(res => {
    //             console.log(res)
    //         }).catch(err => {
    //             alert("something went wrong")
    //     })
    // }

    componentDidMount = async () => {
        try {
            // this is the url to fetch users
            const user_id = localStorage.getItem('userId')
            let url = '/local/users/' + user_id;
            console.log(url)
            let res = await fetch(url,
                {
                    method: 'get',  // we're making a GET request
                    withCredentials: true,  // we want to use authorisation
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({details: result} ); // and we store that json in the state
            this.setState(({}))
        } catch (e) {
        }
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
                            User id: {localStorage.getItem('userId')}
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