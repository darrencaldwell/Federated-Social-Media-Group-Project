import React from "react";
import {Button, Container, Form, FormControl, InputGroup} from "react-bootstrap";


// props: username_results
class RoleSelector extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            selected_user_id: this.props.username_results[0].userId,
            selected_imp_id: this.props.username_results[0].implId,
            selected_role: 'Guest'

        }
        this.handleUserChange = this.handleUserChange.bind(this);
        this.handleRoleChange = this.handleRoleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
    }

    handleUserChange(event) {
        let string = event.target.value
        let indexLastHash = string.lastIndexOf('#')
        let imp_id = string.substring(indexLastHash + 1)
        let user_id = string.slice(0, indexLastHash)

        this.setState({
            selected_user_id: user_id,
            selected_imp_id: imp_id
        });
      }

    handleRoleChange(event) {
        this.setState({
            selected_role: event.target.value,
        });
    }
    
    // set role
    handleSubmit(event) {
        event.preventDefault();
        alert(`${this.state.selected_user_id}#${this.state.selected_imp_id} with role ${this.state.selected_role}`);
        fetch(`/local/forums/${this.props.forumID}/permissions/users`, {
            method: "PATCH",
            withCredentials: true,
            credentials: 'include',
            headers: {
                'Authorization': "Bearer " + localStorage.getItem('token'), // need to get the auth token from localStorage
                'Content-Type': 'application/json',
                'redirect': 1
            },
            body: JSON.stringify({
                "role": this.state.selected_role,
                "user": this.state.selected_user_id,
                "impId": parseInt(this.state.selected_imp_id)
            })
        }).then(responseJson => {
            console.log(responseJson);
            if (responseJson.status === 200) {
                alert("Successfully modified the user's role!");
                window.location.href = this.state.backURL;
            }
        }).catch(error => this.setState({
            message: "Error modifying role: " + error
        }));
    }

    render() {
        return (
            <Form onSubmit={this.handleSubmit} >
                <label htmlFor="basic-url">Select user's permission to modify</label>
                <InputGroup className="mb-3">
                    <Form.Control as="select" onChange={this.handleUserChange} value={this.state.selected_user}>
                        {this.props.username_results.map((user) => (
                            <option value={`${user.userId}#${user.implId}`}
                                    key={`${user.userId}#${user.implId}`}>
                                {user.username}#{user.userId} from {user.implName}#{user.implId} | Current Role(s): {user.roles} 
                            </option>
                        ))}
                    </Form.Control>
                </InputGroup>
                <label htmlFor="basic-url">Select new role</label>
                <InputGroup className="mb-3">
                    <Form.Control as="select" onChange={this.handleRoleChange} value={this.state.selected_role}>
                        <option value='Guest'>Guest</option>
                        <option value='User'>User</option>
                        <option value='Moderator'>Moderator</option>
                    </Form.Control>
                </InputGroup>
                <Button variant="primary" type="submit">
                    Submit
                </Button>
            </Form>
        )
    }
}

class AssignRoles extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            username_to_search: null,
            username_results: null
        }
        this.update_username_to_search = this.update_username_to_search.bind(this);
        //this.search_username = this.search_username.bind(this);
    }

    update_username_to_search(event) {
        this.setState({search_username: event.target.value})
    }

    search_username = async (event) => {
        event.preventDefault();
        // TODO: GET REQ to get all matching usernames
        let res = await fetch(`/local/forums/${this.props.forumID}/userIdentity/${this.state.search_username}`, 
            {
                method: 'get',  // we're making a GET request

                withCredentials: true,  // we want to use authorisation
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'),
                    'Accept': 'application/json',
                    'redirect': 1
                }
            }
        );
        if (res.ok) {
            let result = await res.json() // we know the result will be json
            this.setState( {username_results: result} ); // and we store that json in the state
        } else {
            alert("Error: " + res.statusText);
        }        
    }


    render() {

        let role_select;
        if (!this.state.username_results) {
            role_select = null
        } else if (this.state.username_results.length > 0) {
            role_select = <RoleSelector forumID={this.props.forumID } username_results={this.state.username_results}/>
        } else if (this.state.username_results.length === 0) {
            role_select = <p1>No user's found with that username!</p1>;
        }

        return (
            // need user search by username, return id + implName
            <Container>

            <Form onSubmit={this.search_username}>
                <label htmlFor="basic-url">Search for user by username</label>
                <InputGroup className="mb-3">
                    <FormControl
                    placeholder="Username"
                    onChange={this.update_username_to_search}
                    />
                    <InputGroup.Append>
                    <Button type="submit" variant="outline-secondary">Search</Button>
                    </InputGroup.Append>
                </InputGroup>
            </Form>

            {role_select}

            </Container>
        );
    }
} export default AssignRoles
