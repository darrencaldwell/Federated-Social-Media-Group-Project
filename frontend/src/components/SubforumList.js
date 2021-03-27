import React, {Component} from 'react';
import {Card, Container, Button} from "react-bootstrap";
import '../styling/container-pages.css';

// props: match.params.impID, match.params.forumID
export default class SubforumList extends Component {

    constructor(props) {
        super(props)
        this.state = {
            subforumList: [], // the list of subforums will be stored here, once loaded
            forum: {},
            forumName: {},
            forumLink: {}
        }
    }

    // When the component loads, fetch the list of subforums
    componentDidMount = async () => {
        try {
            // get the list of subforums

            let url = "/api/forums/" + this.props.match.params.forumID + "/subforums";

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

            let result = await res.json(); // we know the result will be json
            this.setState({subforumList: result._embedded.subforumList} ); // and we store that json in the state

            
            // get the forum name

            let url2 = "/api/forums/" + this.props.match.params.forumID;

            let res2 = await fetch(url2,
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

            let result2 = await res2.json(); // we know the result will be json
            this.setState({forum: result2 }); // and we store that json in the state
        } catch (e) {
        }
    }


    render() {
        //var name = this.state.forumName;
        return (
            <div className="subforum-container">
                <Button className="button forum-info forum-info-container" href={'/' + this.props.match.params.forumID}>
                {this.state.forum.forumName} 
                </Button>
                <Container className="subforumlist">
                    {/*Use the map function to apply the html to all forums in the list */}
                    {this.state.subforumList.map((subforum) => (
                        <Card key={subforum.id} className="forum" >  {/*each forum is displayed as a card with className forum */}
                            <Card.Link href={'/' + this.props.match.params.impID + '/' + this.props.match.params.forumID + '/' + subforum.id}>
                                <Card.Body className="forum-body">
                                {/*The card consists of the name of the forum, which links to the forum itself */}
                                    {subforum.subforumName}
                                </Card.Body>
                            </Card.Link>
                        </Card>
                    ))}
                </Container>
                <a className="button" href={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/new"}>
                    New Subforum
                </a>
            </div>)
    }
}
