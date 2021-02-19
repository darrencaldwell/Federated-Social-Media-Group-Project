import React, {Component} from 'react';
import {Card, Container} from "react-bootstrap";
import '../styling/container-pages.css';

// props: match.params.impID
export default class ForumList extends Component {

    constructor(props) {
        super(props)
        this.state = {
            forumList: [] // the list of forums will be stored here, once loaded
        }
    }

    // When the component loads, fetch the list of forums
    componentDidMount = async () => {
        try {
            // this is the url to fetch forums from, no IDs required
            let url = "/api/forums";

            let res = await fetch(url, 
                {
                    method: 'get',  // we're making a GET request

                    withCredentials: true,  // we want to use authorisation
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json',
                        'redirect': this.props.match.params.impID
                    }
                }
            );

            if (res.ok) {
                let result = await res.json(); // we know the result will be json
                this.setState({forumList: result._embedded.forumList} ); // and we store that json in the state
            } else {
                alert("Error: " + res.statusText);
            }

        } catch (e) {
            console.log("Error", e.stack);
            console.log("Error", e.name);
            console.log("Error", e.message);
        }
    }


    render() {
        return (
            <div className="forum-container">

                <Container className="forumlist">
                {/*Use the map function to apply the html to all forums in the list */}
                {this.state.forumList.map((forum) => (
                    <Card className="forum" >  {/*each forum is displayed as a card with className forum */}
                        <Card.Body>
                            {/*The card consists of the name of the forum, which links to the forum itself */}
                            <Card.Link href={'/' + this.props.match.params.impID + '/' + forum.id}>
                                {forum.forumName}
                            </Card.Link> 
                        </Card.Body>                    
                    </Card>
                    ))}
                </Container>
                
                <a className="button" href={"/" + this.props.match.params.impID + "/new"}>
                    New Forum
                </a>

            </div>)
    }
}
