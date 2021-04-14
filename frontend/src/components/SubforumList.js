import React, {Component} from 'react';
import {Card, Container, Button} from "react-bootstrap";
import {Link} from 'react-router-dom';
import AdminDropDown from './AdminDropDown';
import '../styling/container-pages.css';

function SubforumCard({currID, subforum, impID, forumID}) {
    const styling = (subforum.id === currID) ? "current"
                                            : "other";
    
    return (
        <Card className={"forum " + styling} >  {/*each forum is displayed as a card with className forum */}
            <Card.Link as={Link} to={'/' + impID + '/' + forumID + '/' + subforum.id}>
                <Card.Body className={"forum-body " + styling}>
                    {/*The card consists of the name of the subforum, which links to the subforum itself */}
                    <Card.Text className="forum-name">{subforum.subforumName}</Card.Text>
                    <AdminDropDown permsLink={`/editperms/subforum/${subforum.id}/${subforum.subforumName}`}/>
                </Card.Body>                    
            </Card.Link> 
        </Card>
    )
}

// props: match.params.impID, match.params.forumID
export default class SubforumList extends Component {

    constructor(props) {
        super(props)
        this.state = {
            subforumList: [], // the list of subforums will be stored here, once loaded
            forum: {},
            forumName: {},
            forumLink: {},
            currSubforumID: (typeof this.props.match.params.subforumID == "undefined") ? -1
                                                                                       : this.props.match.params.subforumID,
        }
    }

    fetchSubforums = async () => {
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
            this.setState({subforumList: result._embedded.subforumList} ); // and we store that json in the state
        } catch (e) {
        }

    }

    // When the component loads, fetch the list of subforums
    componentDidMount = () => {
        this.fetchSubforums();
    }

    componentDidUpdate = (oldProps) => {
        if (this.props.match.params.forumID !== oldProps.match.params.forumID ||
            this.props.match.params.impID !== oldProps.match.params.impID) {

            this.fetchSubforums();
        }

        if(typeof this.props.match.params.subforumID !== "undefined" && 
           (typeof oldProps.match.params.subforumID === "undefined" || 
            this.props.match.params.subforumID !== oldProps.match.params.subforumID)) {
            this.setState({currSubforumID : this.props.match.params.subforumID});
        }
    }

    render() {
        //var name = this.state.forumName;
        return (
            <div className="subforum-container">
                <Button className="button forum-info forum-info-container" href={'/' + this.props.match.params.impID + '/' + this.props.match.params.forumID}>
                    {this.state.forum.forumName}
                </Button>
                <Container className="subforumlist">
                    {/*Use the map function to apply the html to all forums in the list */}
                    {this.state.subforumList.map((subforum) => (
                        <SubforumCard key={subforum.id} currID={this.state.currSubforumID} subforum={subforum} impID={this.props.match.params.impID} forumID={this.props.match.params.forumID}/>
                        // <ForumCard key={subforum.id} 
                        // link={`/${this.props.match.params.impID}/${this.props.match.params.forumID}/${subforum.id}`} 
                        // name={subforum.subforumName}
                        // forumID={this.props.match.params.forumID}
                        // subforumID={subforum.id}/>
                    ))}
                </Container>
                <Button as={Link} bsPrefix="button" to={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/new"}>
                    New Subforum
                </Button>
            </div>)
    }
}
