import React, {Component} from 'react';
import {Card, Container, Button} from "react-bootstrap";
import {Link} from 'react-router-dom';
import '../styling/container-pages.css';

function ForumCard({currID, forum, impID}) {
    const styling = (forum.id === currID) ? "current"
                                          : "other";
    
    return (
        <Card className={"forum " + styling} >  {/*each forum is displayed as a card with className forum */}
            <Card.Link as={Link} to={'/' + impID + '/' + forum.id}>
                <Card.Body className={"forum-body " + styling}>
                    {/*The card consists of the name of the forum, which links to the forum itself */}
                        {forum.forumName}
                </Card.Body>                    
            </Card.Link> 
        </Card>
    )
}

// props: match.params.impID
export default class ForumList extends Component {

    constructor(props) {
        super(props)
        this.state = {
            forumList: [], // the list of forums will be stored here, once loaded
            currForumID: (typeof this.props.match.params.forumID == "undefined") ? -1
                                                                                 : this.props.match.params.forumID,
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

    componentDidUpdate(prevProps) {
        if(typeof this.props.match.params.forumID !== "undefined" && 
           (typeof prevProps.match.params.forumID === "undefined" ||
           this.props.match.params.forumID !== prevProps.match.params.forumID)){
            this.setState({currForumID : this.props.match.params.forumID});
        }
    }

    render() {
        return (
            <div className="forum-container">

                <Container className="forumlist">
                {/*Use the map function to apply the html to all forums in the list */}
                {this.state.forumList.map((forum) => ( 
                    <ForumCard key={forum.id} currID={this.state.currForumID} forum={forum} impID={this.props.match.params.impID}/>  //each forum is displayed as a card with className forum 
                    ))}
                </Container>
                
                <Button as={Link} bsPrefix="button" to={"/" + this.props.match.params.impID + "/new"}>
                    New Forum
                </Button>

            </div>)
    }
}
