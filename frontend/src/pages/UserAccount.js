import React from "react";
import {Button, Card, Container, Nav} from "react-bootstrap";
import {Link, Redirect} from "react-router-dom";
//import DisplayPicture from "../components/account/DisplayPicture";
//import axios from 'axios'

class UserAccount extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            uploadedPicture: false,
            userInfo: []
        }
    }


    componentDidMount = async () => {
        try {
            // get user_id, we need it if its a local link for the backend
            let user_id = /[^/]*$/.exec(atob(this.props.match.params.userURL))[0];
            let url = "/api/users/" + user_id

            let res = await fetch(url
                , {
                    method: 'get',
                    withCredentials: true,
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect-url': atob(this.props.match.params.userURL)
                    }
                })
            const result = await res.json()
            this.setState({userInfo: result}); // we store the json for the post in the state
            console.log(this.state.userInfo)

        } catch (e) {
            console.log(e)
        }

    }


    render() {
        const date = new Date(this.state.userInfo.createdTime * 1000)
        const forums_url = "/1/forums"

        if (this.state.userInfo.id === localStorage.getItem('userId')) {
            return <Redirect to={'/account'}/>
        }

        return (
            <Container>
                <Card.Title>Account for {this.state.userInfo.username}</Card.Title>
                {/*<DisplayPicture uploadedPicture={false}/>*/}


                {/*<Nav fill variant="tabs" defaultActiveKey="/">*/}
                {/*    <Nav.Item>*/}
                {/*        <Nav.Link as={Link} to='/'>About Us</Nav.Link>*/}
                {/*    </Nav.Item>*/}
                {/*    <Nav.Item>*/}
                {/*        <Nav.Link as={Link} to='/userposts'>//users name Posts</Nav.Link>*/}
                {/*    </Nav.Item>*/}
                {/*    <Nav.Item>*/}
                {/*        <Nav.Link as={Link} to='/usercomments'>//users Comments</Nav.Link>*/}
                {/*    </Nav.Item>*/}
                {/*    <Nav.Item>*/}
                {/*        <Nav.Link as={Link} to='/'>//users Roles</Nav.Link>*/}
                {/*    </Nav.Item>*/}
                {/*</Nav>*/}
                <Card>
                    <Card.Body>
                        <Card.Text>
                            Username: {this.state.userInfo.username}
                        </Card.Text>
                        <Card.Text>
                            User id: {this.state.userInfo.id}
                        </Card.Text>
                        <Card.Text>
                            Joined: {date.toLocaleString()}
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
