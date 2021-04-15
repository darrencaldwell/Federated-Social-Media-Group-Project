import React, {Component} from 'react';
import {Card, Container, Button} from "react-bootstrap";
import {Link} from 'react-router-dom';
import AdminDropDown from './AdminDropDown';
import '../styling/container-pages.css';

function ForumCard({currID, forum, impID, refresh}) {
    const styling = (forum.id === currID) ? "current"
                                          : "other";

    return (
        <Card className={"forum " + styling} >  {/*each forum is displayed as a card with className forum */}
            <Card.Link as={Link} to={'/' + impID + '/' + forum.id}>
                <Card.Body className={"forum-body " + styling}>
                    <Card.Text className="forum-name">{forum.forumName}</Card.Text>
                    { impID === "1" && <AdminDropDown
                        refresh={refresh}
                        className="admin-dropdown"
                        forumID={forum.id}
                        permsLink={`/editperms/forum/${forum.id}/${forum.forumName}`}/>
                    }
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

    refresh = () => {
        this.fetchForums();
    }

    // When the component loads, fetch the list of forums
    componentDidMount = () => {
        this.fetchForums();
    }

    fetchForums = async () => {
        let forums_result
        try {
            // this is the url to fetch forums from, no IDs required
            let url = "/api/forums";
            let result;

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
                forums_result = await res.json(); // we know the result will be json
                //this.setState({forumList: result._embedded.forumList} ); // and we store that json in the state
            } else {
                alert("Error: " + res.statusText);
            }
            
            if (this.props.match.params.impID == 1) {
                forums_result._embedded.forumList.forEach( async (forum) => {
                    let url = `/local/forums/${forum.id}/roles`
   
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
                       forum.roles = result
                   } else {
                       console.log("Error: " + res.statusText);
                   }
                })
                this.setState({forumList: forums_result._embedded.forumList} ); // and we store that json in the state
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
        console.log(this.state.forumList)
        return (
            <div className="forum-container">

                <Container className="forumlist">
                {/*Use the map function to apply the html to all forums in the list */}
                {this.state.forumList.map((forum) => ( 
                    <ForumCard 
                        key={forum.id}
                        currID={this.state.currForumID}
                        forum={forum}
                        impID={this.props.match.params.impID}
                        refresh={this.refresh}/>  //each forum is displayed as a card with className forum 
                        // <ForumCard key={forum.id} 
                        // link={`/${this.props.match.params.impID}/${forum.id}`} 
                        // forumID={forum.id}
                        // name={forum.forumName}/>
                    ))}
                </Container>
                
                <Button as={Link} bsPrefix="button" to={"/" + this.props.match.params.impID + "/new"}>
                    New Forum
                </Button>

            </div>)
    }
}
