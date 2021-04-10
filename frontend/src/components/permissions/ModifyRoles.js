import React from "react";
import {Button, Card, Container, Nav, Form, FormGroup, Jumbotron, InputGroup, FormControl} from "react-bootstrap";

class ModifyRoles extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            search_username: null
        }
        this.updateSearchUsername = this.updateSearchUsername.bind(this);
    }

    updateSearchUsername(event) {
        this.setState({search_username: event.target.value})
        console.log(event)
    }

    render() {
        return (
            // need user search by username, return id + implName
            <Card>
                <Card.Text>Search for user to add</Card.Text>
                <Form onSubmit={this.searchUsername}>
                    <FormGroup controllId="username">
                    <Form.Control type="username" placeholder="Enter username" onChange={this.updateSearchUsername} />
                    </FormGroup>
                    <Button variant="primary" type="submit">
                        Search
                    </Button>
                </Form>

                <Card.Text>Select user's permission to modify</Card.Text>
            </Card>            
            // select user, select role, submit


            // current roles
            // username + id + impl -> role
        );
    }
}

export default ModifyRoles
