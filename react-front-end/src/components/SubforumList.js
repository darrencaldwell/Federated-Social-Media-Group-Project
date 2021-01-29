import React, {Component} from 'react';
import {Alert, Container, Spinner} from "react-bootstrap";

// props: forumID
class SubforumList extends Component {

    constructor(props) {
        super(props)
        this.state = {
            subforumList: {} // the list of subforums will be stored here, once loaded
        }
    }

    // When the component loads, fetch the list of subforums
    componentDidMount = async () => {
        try {
            // this is the url to fetch forums from, no IDs required
            let url = "/api/subforums/${this.props.forumID}";

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
            this.setState({forumList: result._embedded.subforumList} ); // and we store that json in the state

        } catch (e) {
        }
    }


    render() {
        return (
            <Container>
                {/*Use the map function to apply the html to all forums in the list */}
                {this.state.subforumList.map((subforum) => (
                    <Card className="subforum" >  {/*each forum is displayed as a card with className forum */}
                        <Card.Body>
                            {/*The card consists of the name of the forum, which links to the forum itself */}
                            <Card.Link href={subforum._links.self.href.replace('/api', '')}>
                                {subforum.subforumName}
                            </Card.Link> 
                        </Card.Body>                    
                    </Card>
                    ))}
            </Container>)
    }
}

export default ViewPosts;
