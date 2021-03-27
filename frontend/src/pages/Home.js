import React, {Component} from 'react';
import {Container, Jumbotron, Button, Dropdown, Card} from "react-bootstrap";
import {Link} from 'react-router-dom';
import "./../styling/home.css";

// props: match.params.impID
class Home extends Component {

    constructor(props) {
        super(props);
        const root = (typeof this.props.match == 'undefined'); // it's a root comment if match is undefined
        const impID = root ? ('1') // 1 is the local id
                         : (this.props.match.params.impID);
        this.state = {
            impID: impID,
            impList : [],
            impName : {}
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

        // If username exists, get username from token and output message
        if (localStorage.getItem('username')) {
            return (
                <Container className="jumbotron" fluid>
                    <h1 className="display-3">Welcome {localStorage.getItem('username')}</h1>
                    {/* <p className="lead">
                        The home page currently doesn't contain anything useful but hopefully
                        it will in the future.
                    </p> */}
                    <Card>
                        <Card.Body>
                            {"You are currently in implementation: " + this.state.impName}
                        </Card.Body>
                    </Card>
                    <Link to={'/account'}><Button variant='light' as="input" type="button" value="Go to your account"/>{' '}</Link>
                    <Link to={'/' + this.state.impID + '/forums'}><Button variant='light' as="input" type="button" value="Go to forums"/>{' '}</Link>
                    <Card>
                        <Card.Body>
                            choose a different implementation:
                        </Card.Body>
                    <Dropdown>
                        {this.state.impList.map((impl) => (
                            <Dropdown.Item key={impl.id} href={"/" + impl.id} onClick={() => this.props.changeImp({id: impl.id, name: impl.name})}>{impl.name}</Dropdown.Item>
                        ))}
                    </Dropdown>
                    </Card>
                </Container>
            )
        }

        // Otherwise just return a home page
        return (
            <Container className="jumbotron" fluid>
                <Jumbotron>
                    <h1 className="display-3">Welcome</h1>
                    <p className="lead">
                        The home page currently doesn't contain anything useful but hopefully
                        it will in the future.
                    </p>
                    <Link to={'/login'}><Button variant='light' as="input" type="button"
                                                value="Go to login page"/>{' '}</Link>
                    <Link to={'/register'}><Button variant='light' as="input" type="button"
                                                   value="Go to register page"/></Link>
                </Jumbotron>
            </Container>
        );
    }
}

export default Home;
