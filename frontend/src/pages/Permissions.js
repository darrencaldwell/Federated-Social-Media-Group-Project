import React, {Component} from 'react';
import {Tabs, Tab, Container, InputGroup, Dropdown, DropdownButton} from "react-bootstrap";
import {Link} from 'react-router-dom'

// props: match.params.impID match.params.forumID
class Permissions extends Component {

    constructor(props) {
        super(props);
        this.state = {
        }
    }

    // Runs when the component is loaded, fetching the list of implementations to load into state
    componentDidMount = async () => {
        try {
            // the url needs the post id from the props
            let url = '/local/implementations';
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({impList: result._embedded.implementationList }); // we store the json for the post in the state
            // the url needs the post id from the props

            url = '/local/implementations/' + this.state.impID; // get the name of the current implementation
            res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );

            result = await res.json(); // we know the result will be json
            this.setState({impName: result.name }); // we store the json for the post in the state

        } catch (e) {
        }
    }

    render() {
        return (
        <Container>
            <Tabs defaultActiveKey="creator" transition={false}>
                <Tab eventKey="creator" title="Creator">
                    <Container>
                        <InputGroup>
                            <DropdownButton title="Object">
                                <Dropdown.Item>Subforum</Dropdown.Item>
                                <Dropdown.Item></Dropdown.Item>
                                <Dropdown.Item>Subforum</Dropdown.Item>
                            </DropdownButton>
                        </InputGroup>
                    </Container>
                </Tab>
                <Tab eventKey="moderator" title="Moderator">
                    moderator permissions
                </Tab>
                <Tab eventKey="user" title="User">
                    user permissions
                </Tab>
                <Tab eventKey="guest" title="Guest">
                    guest permissions
                </Tab>
            </Tabs>
        </Container>
        );
    }
}

export default Permissions;
