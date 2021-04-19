import React from "react";
import {Container, Row, Col, ListGroup, Button, DropdownButton, Dropdown} from "react-bootstrap";
//import { ChevronCompactLeft } from "react-bootstrap-icons";

// props: forumID
class ModifyRoles extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            subforums: null,
            perms: new Map(),
        }
    }

    componentDidMount = async () => {
        try {
            let resp =  await fetch(`/api/forums/${this.props.forumID}/subforums`, {
                method: "GET",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), // need to get the auth token from localStorage
                    'Content-Type': 'application/json',
                },
            });

            let data = await resp.json();
            let subforums = data._embedded.subforumList;

            subforums = await Promise.all(subforums.map(async subforum => {
                resp = await fetch(`/local/forums/${this.props.forumID}/subforum/${subforum.id}/permissions`, {
                    method: "GET",
                    withCredentials: true,
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                    },
                });

                let result = await resp.json();
                let canPost = result.guest.canPostPosts;
                let canView = result.guest.canViewPosts;
                this.state.perms.set(subforum.id, {oldPost: canPost, oldView: canView, canPost: canPost, canView: canView});
                return {id: subforum.id, name: subforum.subforumName};
            }));

            this.setState({ subforums: subforums });
        } catch {
        }
    }

    revert = (id) => {
        let prev = this.state.perms.get(id);
        prev.canPost = prev.oldPost;
        prev.canView = prev.oldView;
        this.state.perms.set(id, prev);
        this.setState({perms: this.state.perms});
    }

    update = async (id) => {
        try {
            let perms = this.state.perms.get(id); 
            let json = JSON.stringify({canPost: perms.canPost, canView: perms.canView});
            await fetch(`/local/forums/${this.props.forumID}/permissions`, {
                method: "PATCH",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), // need to get the auth token from localStorage
                    'Content-Type': 'application/json',
                },
                body: json,
            });

        } catch {
        }
    }

    updatePost = (id, bool) => {
        let prev = this.state.perms.get(id);
        prev.canPost = bool;
        this.state.perms.set(id, prev);
        this.setState({perms: this.state.perms});
    }

    updateView = (id, bool) => {
        let prev = this.state.perms.get(id);
        prev.canView = bool;
        this.state.perms.set(id, prev);
        this.setState({perms: this.state.perms});
    }

    Subforum = ({forum}) => {
        return (
            <Container>
            <Row>
                <Col>
                    {forum.name}
                </Col>
                <Col>
                    <div>
                    {"Can Post"}
                    <DropdownButton title={this.state.perms.get(forum.id).canPost ? "All Users" : "Members"}>
                        <Dropdown.Item onClick={() => this.updatePost(forum.id, false)} eventKey="Members">Members</Dropdown.Item>
                        <Dropdown.Item onClick={() => this.updatePost(forum.id, true)} eventKey="All Users">All Users</Dropdown.Item>
                    </DropdownButton>
                    </div>
                </Col>
                <Col>
                    <div>
                    {"Can View Posts"}
                    <DropdownButton title={this.state.perms.get(forum.id).canView ? "All Users" : "Members"}>
                        <Dropdown.Item onClick={() => this.updateView(forum.id, false)} eventKey="Members">Members</Dropdown.Item>
                        <Dropdown.Item onClick={() => this.updateView(forum.id, true)} eventKey="All Users">All Users</Dropdown.Item>
                    </DropdownButton>
                    </div>
                </Col>
                <Col>
                    <Button onClick={() => this.revert(forum.id)}>Revert</Button>
                    <Button onClick={() => this.update(forum.id)}>Update</Button>
                </Col>
            </Row>
            </Container>
        );
    }

    render() {
        if (this.state.subforums === null) return "";

        return (
            <ListGroup>
                {this.state.subforums.map(forum => {
                    return (
                        <ListGroup.Item>
                            <this.Subforum forum={forum}/>
                        </ListGroup.Item>
                    );
                })}
            </ListGroup>
        );
    }
}

export default ModifyRoles
