import React, {Component} from 'react';
import {Card, Container} from "react-bootstrap";
import '../styling/container-pages.css';

// props: forumID
export default class SubforumList extends Component {

    constructor(props) {
        super(props)
        this.state = {
            subforumList: [], // the list of subforums will be stored here, once loaded
            forumName: {},
            forumLink: {}
        }
    }

    // When the component loads, fetch the list of subforums
    componentDidMount = async () => {
        try {
            let url = "/api/forums/" + this.props.forumID + "/subforums";

            let res = await fetch(url, 
                {
                    method: 'get',  // we're making a GET request

                    withCredentials: true,  // we want to use authorisation
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({subforumList: result._embedded.subforumList} ); // and we store that json in the state


            let url2 = "/api/forums/" + this.props.forumID;

            let res2 = await fetch(url2, 
                {
                    method: 'get',  // we're making a GET request

                    withCredentials: true,  // we want to use authorisation
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );

            let result2 = await res2.json(); // we know the result will be json
            this.setState({forumName: result2.forumName }); // and we store that json in the state
        } catch (e) {
        }
    }


    render() {
        var name = this.state.forumName;
        return (
            <div className="subforum-container">
                <a className="button forum-info forum-info-container" href={'/' + this.props.forumID} body={this.state.forumName}>
                    forum name would go here but I can't get it to work
                </a>
                <Container className="subforumlist">
                    {/*Use the map function to apply the html to all forums in the list */}
                    {this.state.subforumList.map((subforum) => (
                        <Card className="subforum" >  {/*each forum is displayed as a card with className forum */}
                            <Card.Body>
                                {/*The card consists of the name of the forum, which links to the forum itself */}
                                <Card.Link href={'/' + this.props.forumID + '/' + subforum.id}>
                                    {subforum.subforumName}
                                </Card.Link> 
                            </Card.Body>                    
                        </Card>
                    ))}
                </Container>
                <a className="button" href={"/" + this.props.forumID + "/new"}>
                    New Subforum
                </a>
            </div>)
    }
}
