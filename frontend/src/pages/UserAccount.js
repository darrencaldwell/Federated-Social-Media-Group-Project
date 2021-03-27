import React from "react";
import {Button, Card, Container, Nav} from "react-bootstrap";
import {Link} from "react-router-dom";
import DisplayPicture from "../components/account/DisplayPicture";
import axios from 'axios'

class UserAccount extends React.Component {

    constructor(props) {
        super(props);
        console.log('made it to user account')
        console.log('props ye',props)
        // console.log('constructor props->',this.props.match.params.userId)
        // console.log('constructor props field id->',props.user.userId)
        this.state = {
            uploadedPicture: false,
            userInfo: []
        }
    }


    componentDidMount = async () => {
        console.log('did mount props', this.props)
        console.log('did mount url???!', this.props.match.params)
        // console.log('did mount props userid', this.props.match.params.userId)
        // let user_url = 'local/users/' + this.props.match.params.userId
        // let user_url = 'local/users/'

        // try {
        //     let url = "/api/users/" + this.props.match.params.userId
        //     let res = await fetch(url
        //         , {
        //             method: 'get',
        //             withCredentials: true,
        //             credentials: 'include',
        //             headers: {
        //                 'Authorization': "Bearer" + localStorage.getItem('token'),
        //                 'Accept': 'application/json',
        //                 'redirect-url': this.props.match
        //             }
        //         })
        // } catch (e) {
        //     console.log(e)
        // }

    }


    render() {
        const date = new Date(this.state.userInfo.dateJoined * 1000)
        const forums_url = "/1/forums"
        console.log('user info', this.state.userInfo)

        return (
            <Container>
                <Card.Title>Account for {this.props.userId}</Card.Title>
                {/*<DisplayPicture uploadedPicture={false}/>*/}
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
                        <Nav.Link as={Link} to='/userposts'>//users name Posts</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to='/usercomments'>//users Comments</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to='/'>//users Roles</Nav.Link>
                    </Nav.Item>
                </Nav>
                <Card>
                    <Card.Body>
                        <Card.Text>
                            Username: {this.state.userInfo.username}
                        </Card.Text>
                        <Card.Text>
                            First name: {this.state.userInfo.firstName}
                        </Card.Text>
                        <Card.Text>
                            Last name: {this.state.userInfo.lastName}
                        </Card.Text>
                        <Card.Text>
                            User id: {this.state.userInfo.userId}
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

export default UserAccount
