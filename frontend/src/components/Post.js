import React, {Component} from 'react';
//import { BrowserRouter as Router, Link } from 'react-router-dom';
import Comments from './Comments';
// import CreatePost from './CreatePost.js';
// import '../styling/Post.css';
import '../styling/container-pages.css';
import {Card, Container, Spinner, Button, ButtonGroup} from "react-bootstrap";
import CardActionArea from '@material-ui/core/CardActionArea';
import Avatar, {Cache} from 'react-avatar';
//import {Link} from 'react-router-dom';
import Voting from './Voting';
import TimeSince from './TimeSince';
import ReactMarkdown from 'react-markdown';
import BackButton from './BackButton';

// for react avatar
const cache = new Cache({

    // Keep cached source failures for up to 7 days
    sourceTTL: 1,

    // Keep a maximum of 0 entries in the source cache (we don't care about remembering broken links!)
    sourceSize: 0
});

// props: match.params.impID, match.params.postID, match.params.subforumID, match.params.forumID, match.params.commentID
export class Post extends Component {

    constructor(props) {
        super(props);
        const expanded = (typeof this.props.match.params.commentID != 'undefined'); // it's an expanded comment if the url has the comment id
        this.delete = this.delete.bind(this);
        this.state = {
            expanded: expanded,
            loading: true, // Set to true if loading
            post: {}, // the post is stored here once loaded
            post_author: {},
            subforumURL: "/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID
        }
    }

    componentDidUpdate = (prevProps) => {
        if (this.props.match.url !== prevProps.match.url) {
            console.log("new url: " + this.props.match.url);
            const expanded = (typeof this.props.match.params.commentID != 'undefined'); // it's an expanded comment if the url has the comment id
            this.setState({expanded: expanded});
        }
    }

    // Runs when the component is loaded, fetching the post into state
    componentDidMount = async () => {
        // get post
        try {
            this.setState({loading: true});
            // the url needs the post id from the props
            let url = '/api/posts/' + this.props.match.params.postID;
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect': this.props.match.params.impID
                    }
                }
            );
            let result_post = await res.json(); // we know the result will be json

            this.setState({post: result_post, loading: false}); // we store the json for the post in the state
        } catch (e) {
            console.log(e)
        }

        try {
            // the url to make the request to is given by the parent
            let url = "/api/users/" + this.state.post.userId
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect-url': this.state.post._links.user.href
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({profilePicture: result.profileImageURL, user: result, loading: false})

        } catch (e) {
            console.log("GET_USER " + e);
            this.setState({loading: false, profilePicture: ""})
        }
    }

    delete(e) { 
        e.preventDefault();
        if (window.confirm('Are you sure you wish to delete this post?\n THIS CANNOT BE UNDONE!')) {
            // this is the HTML request
            fetch("/api/posts/" + this.props.match.params.postID, {
                method: "DELETE",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                    'Content-Type': 'application/json',
                    'redirect': this.props.match.params.impID
                }

            }).then(responseJson => { // log the response for debugging
                console.log(responseJson);
                if (responseJson.status === 200) {
                    alert("Successfully deleted post.");
                    window.location.href = this.state.subforumURL;
                }
            }).catch(error => this.setState({ // catch any error
                message: "Error deleting post: " + error
            }));
        }
    }

    render() {
        // styling for the loading spinner - should be moved to a separate styling file if possible
        const spinnerStyle = { position: "fixed", top: "50%", left: "50%", transform: "translate(-50%, -50%)" }

        if (this.state.loading) {
            // while loading, show a loading spinner with the above style
            return (
                <Spinner animation="border" role="status" style={spinnerStyle}>
                    <span className="sr-only">Loading...</span>
                </Spinner>
            )
        }
        
        const url = this.state.expanded ? ('/api/comments/' + this.props.match.params.commentID + '/comments')
                                        : ('/api/posts/' + this.props.match.params.postID + '/comments');
        const parsed_user_link = btoa(this.state.post._links.user.href)

        return (
            <div className="post-wrapper">
            <Container bsPrefix="post-container">
            <div className="separator"/>
                {/* <div className="mt-3"> */}
                    <Card border="dark" className="post">
                        <Card.Body>
                            <div className="post-columns">
                                <div className="post-voting">
                                    <Voting className="voting"
                                        upvotes={this.state.post.upvotes} 
                                        downvotes={this.state.post.downvotes} 
                                        _userVotes={this.state.post._userVotes}
                                        type="posts"
                                        postID={this.props.match.params.postID}
                                        impID={this.props.match.params.impID}
                                    ></Voting>
                                    <div className="voting-adj">
                                        <CardActionArea href={'/user/' + parsed_user_link}>
                                            <Avatar cache={cache} size="50" round={true} src={this.state.profilePicture}
                                                name={this.state.post.username}/>
                                            {"  "} {this.state.post.username}
                                        </CardActionArea>
                                        <Card.Subtitle className="text-muted mt-1 time-since">
                                            <TimeSince createdTime={this.state.post.createdTime}/>
                                        </Card.Subtitle>
                                        <Card.Subtitle className="text-muted mt-1 time-since">
                                            <TimeSince createdTime={this.state.post.createdTime} modifiedTime={this.state.post.modifiedTime}/>
                                        </Card.Subtitle>
                                    </div>
                                </div>
                                <Card.Body className="post-text">
                                    <Card.Title className="post-title">{this.state.post.postTitle}</Card.Title>
                                    <ReactMarkdown className="post-body">{this.state.post.postContents}</ReactMarkdown>
                                </Card.Body>
                                {/* <div className="post-buttons"> */}
                                <ButtonGroup vertical className="post-buttons">
                                    <Button className="button edit-button" title="Edit" href={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID + "/edit"}>🖉</Button>
                                    <Button className="button delete-button" title="Delete" onClick={this.delete} href={this.state.subforumURL}>🗑</Button>
                                    {/*<a className="button edit-button" href={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID + "/edit"}>🖉</a>*/}
                                    {/*<a className='button delete-button' onClick={() => {*/}
                                    {/*    if (window.confirm('Are you sure you wish to delete this post?')) this.delete()*/}
                                    {/*}}*/}
                                    {/*   href={subforumURL}>🗑</a>*/}
                                </ButtonGroup>
                                {/* </div> */}
                            </div>
                        </Card.Body>
                    </Card>
                {/* </div> */}

                {/*<CreateComment url={url}/>*/}

                <div className="separator"/>

                {this.state.expanded ? <BackButton url={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID}/> : "" }

                {/*<Dropdown className="mt-3">*/}
                {/*<Dropdown.Toggle variant="light" id="dropdown-comments">View Comments</Dropdown.Toggle>*/}
                {/*<Dropdown.Menu>*/}
                <Comments url={url} impID={this.props.match.params.impID} expanded={this.state.expanded}
                        posturl={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID}/>
                {/*</Dropdown.Menu>*/}
                {/*</Dropdown>*/}
                
                <div className="separator"/>

            </Container>

            <a className="button" href={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID + "/new"}> Create Comment</a>

            </div>
        )
    }
}
export default Post
