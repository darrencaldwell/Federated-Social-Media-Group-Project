import React, { Component } from 'react'
import {Card} from "react-bootstrap";
import {Container} from 'react-bootstrap';
import '../styling/container-pages.css';

// props: comment (json), posturl
class Comment extends Component {
    
    render() {
        return (
            <div>
                <Card>
                    <Card.Body>
                        UserID:{this.props.comment.userId}-  {this.props.comment.commentContent}
                    </Card.Body>
                    <Comments url={"/api/comments" + this.props.comment.id + "/comments"} createurl={"/api/comments" + this.props.comment.id + "/comments"}/>
                    <a className="button create-forum-button" href={this.props.posturl + "/" + this.props.comment.id + "/new"}>
                        Create Comment
                    </a>
                </Card>
            </div>
        )
    }
}

// props: url, posturl
export default class Comments extends Component {
    constructor(props) {
        super(props);
        this.state = {
            commentList: {} // the list of comments will be stored here
        }
    }

    // Runs when the component is loaded, fetching the list of comments into state
    componentDidMount = async () => {
        try {
            // the url needs the post id from the props
            let url = this.props.url;
            
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
            this.setState({commentList: result._embedded.commentList }); // we store the json for the list of comments in the state

        } catch (e) {
            console.log(e);
        }
    }

    render() {
        if (this.state.commentList.length > 0) {

            // if there are comments, display them
            return(
                <Container>
                    {/*map is used to apply this html for each comment in the list */}
                    {this.state.commentList.map((comment) => (
                        // the Comment element above is used for this, which takes the comment json
                        <Comment comment={comment} posturl={this.props.posturl}/>
                    ))}
                </Container>
            )

        } else {    // otherwise, display a message saying there are no comments
            return(
                <Container>
                    <Card>
                        <Card.Body>
                            There are no comments.
                        </Card.Body>
                    </Card>
                </Container>
            )
        }
    }
}
