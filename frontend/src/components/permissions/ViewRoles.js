import React from "react";
import {Container, Row, Col, ListGroup, DropdownButton, Dropdown} from "react-bootstrap";
//import { ChevronCompactLeft } from "react-bootstrap-icons";

// props: forumID
class ViewRoles extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            roles: null,
        }
    }

    componentDidMount = async () => {
        try {
            let resp =  await fetch(`/local/forums/${this.props.forumID}/roles`, {
                method: "GET",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), // need to get the auth token from localStorage
                    'Content-Type': 'application/json',
                },
            });

            let data = await resp.json();
            let roles = data;

            this.setState({ roles: roles});
        } catch {
        }
    }

    render() {
        if (this.state.roles === null) return "";

        return (
            <Container>
            <Row>
                <Col>
                    {"Creators"}
                    <ListGroup>
                    {this.state.roles.creators.map(user => {
                        return (
                            <ListGroup.Item>
                                {user.username}
                            </ListGroup.Item>
                        );
                    })}
                    </ListGroup>
                </Col>
                <Col>
                    {"Moderators"}
                    <ListGroup>
                    {this.state.roles.moderators.map(user => {
                        return (
                            <ListGroup.Item>
                                {user.username}
                            </ListGroup.Item>
                        );
                    })}
                    </ListGroup>
                </Col>
                <Col>
                    {"Users"}
                    <ListGroup>
                    {this.state.roles.users.map(user => {
                        return (
                            <ListGroup.Item>
                                {user.username}
                            </ListGroup.Item>
                        );
                    })}
                    </ListGroup>
                </Col>
            </Row>
            </Container>
        );
    }
}

export default ViewRoles
