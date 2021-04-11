import React, {Component} from 'react';
import {Container, Jumbotron, Button, Card} from "react-bootstrap";
import {Link} from 'react-router-dom';
import "./../styling/home.css";

// props: match.params.impID
class Home extends Component {

    constructor(props) {
        super(props);
        this.state = {
            impID: (typeof this.props.match == 'undefined') ? ('1') // defaults to 1, the local id, if there's no impID in the url
                                                            : (this.props.match.params.impID),
            impList : [],
            impName : {},
            impText : (typeof this.props.currImp == "undefined") ? "Please choose an implementation" // handles the case where currImp is undefined, such as when the site is loading
                                                                 : "You are currently in implementation: " + this.props.currImp.name
        }
    }

    // Runs when the component is loaded, fetching the list of implementations to load into state
    componentDidMount = async () => {
        //try {
        //    // the url needs the post id from the props
        //    let url = '/local/implementations';
        //    let res = await fetch(url
        //        , {
        //            method: 'get', // we're making a GET request

        //            withCredentials: true, // we're using authorisation with a token in local storage
        //            credentials: 'include',
        //            headers: {
        //                'Authorization': "Bearer " + localStorage.getItem('token'),
        //                'Content-Type': 'application/json',
        //                'Accept': 'application/json'
        //            }
        //        }
        //    );

        //    let result = await res.json(); // we know the result will be json
        //    this.setState({impList: result._embedded.implementationList }); // we store the json for the post in the state
        //    // the url needs the post id from the props

        //    url = '/local/implementations/' + this.state.impID; // get the name of the current implementation
        //    res = await fetch(url
        //        , {
        //            method: 'get', // we're making a GET request

        //            withCredentials: true, // we're using authorisation with a token in local storage
        //            credentials: 'include',
        //            headers: {
        //                'Authorization': "Bearer " + localStorage.getItem('token'),
        //                'Content-Type': 'application/json',
        //                'Accept': 'application/json'
        //            }
        //        }
        //    );

        //    result = await res.json(); // we know the result will be json
        //    this.setState({impName: result.name }); // we store the json for the post in the state

        //} catch (e) {
        //}
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
                            {this.state.impText}
                        </Card.Body>
                    </Card>
                    <Link to={'/account'}><Button variant='light' as="input" type="button" value="Go to your account"/>{' '}</Link>
                    <Link to={'/' + this.state.impID + '/forums'}><Button variant='light' as="input" type="button" value="Go to forums"/>{' '}</Link>
                    <Card>
                        <Card.Body>
                            We now support CommonMark Markdown in the bodies of posts and comments! See the spec here: 
                            <a href="https://commonmark.org" target="_blank" rel="noreferrer"> https://commonmark.org/</a>
                        </Card.Body>
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
                        We now support CommonMark Markdown in the bodies of posts and comments! See the spec here: 
                        <a href="https://commonmark.org" target="_blank" rel="noreferrer"> https://commonmark.org/</a>
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
