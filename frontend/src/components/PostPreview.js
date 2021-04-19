import React, {Component} from 'react';
import {Card, Container, Dropdown} from "react-bootstrap";
import CardActionArea from '@material-ui/core/CardActionArea';
import Avatar, {Cache} from 'react-avatar';
import {Link} from 'react-router-dom';
import Voting from './Voting';
import TimeSince from './TimeSince';
import {ThreeDots} from 'react-bootstrap-icons';    
// import '../styling/individualPost.css';
import '../styling/container-pages.css';
import ReactMarkdown from 'react-markdown';

// for react avatar
const cache = new Cache({

    // Keep cached source failures for up to 7 days
    sourceTTL: 7 * 24 * 3600 * 1000,

    // Keep a maximum of 0 entries in the source cache (we don't care about remembering broken links!)
    sourceSize: 0
});

// props: post, impID, forumID, subforumID, update()
export class PostPreview extends Component {
    constructor(props) {
        super(props);
        this.state = {
            can_lock: false,
            can_delete: false,
        }
    }

    componentDidMount = () => {
        this.checkPerms();
        this.getPic();
    }

    checkPerms = () => {
        if (this.props.impID === "1") {
            fetch(`/local/posts/${this.props.post.id}/can_lock`, {
                method: "GET",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                    'Content-Type': 'application/json',
                }

            }).then(responseJson => { // log the response for debugging
                if (responseJson.status === 202) {
                    this.setState({can_lock: true})
                }
            });

            fetch(`/local/posts/${this.props.post.id}/can_delete`, {
                method: "GET",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                    'Content-Type': 'application/json',
                }

            }).then(responseJson => { // log the response for debugging
                if (responseJson.status === 202) {
                    this.setState({can_delete: true});
                }
            });
        }

        if (this.props.post.userId === localStorage.getItem("userId"))
            this.setState({can_delete: true});
    }

    lock = () => {
        let accounts = {
            roles: ["Guest", "User"]
        };

        fetch(`/local/posts/${this.props.post.id}/lock`, {
            method: "POST",
            withCredentials: true,
            credentials: 'include',
            body: JSON.stringify(accounts),
            headers: {
                'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                'Content-Type': 'application/json',
            }

        }).then(_ => { // log the response for debugging
            this.props.update();
        });

    }

    delete = () => {
        if (window.confirm('Are you sure you wish to delete this comment?\n THIS CANNOT BE UNDONE!')) {
            fetch(`/api/posts/${this.props.post.id}`, {
                method: "DELETE",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                    'Content-Type': 'application/json',
                }

            }).then(_ => { // log the response for debugging
                this.props.update();
            });
        }
    }

    genDropDown = () => {
        if (!(this.state.can_lock || this.state.can_delete)) return;

        return (
            <Container className="pr-0 d-flex flex-row justify-content-end admin-dropdown">
                <Dropdown className="admin-dropdown">  
                    <Dropdown.Toggle as={CustomToggle} variant="success" id="dropdown-basic"/>
                    <Dropdown.Menu>
                        {this.state.can_lock && <Dropdown.Item onClick={this.lock}>Lock</Dropdown.Item>}
                        {this.state.can_delete && <Dropdown.Item onClick={this.delete}>Delete</Dropdown.Item>}
                    </Dropdown.Menu>
                </Dropdown>
            </Container>
        );
    }

    // Runs when the component is loaded, fetching the details of the user who created the comment
    getPic = async () => {
        try {
            // the url to make the request to is given by the parent
            let url = "/api/users/" + this.props.post.userId;
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect-url': this.props.post._links.user.href
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({profilePicture: result.profileImageURL, user: result, loading: false});

        } catch (e) {
            console.log("GET_USER " + e);
            this.setState({loading: false, profilePicture: ""});
        }
    }

    render() {

        return (
            <Card border="dark" className="mt-3 post-preview" >
                <Card.Body>
                    <div className="post-columns">
                        <div className="post-comment-voting-container">
                            <Voting className="voting" upvotes={this.props.post.upvotes} 
                                downvotes={this.props.post.downvotes} 
                                _userVotes={this.props.post._userVotes}
                                type="posts"
                                postID={this.props.post.id}
                                impID={this.props.impID}
                            ></Voting>
                            {/*Links to the post itself, to view/make comments. Removing the /api part directs you to the correct app page. */}
                            <div className="voting-adj">
                                <CardActionArea>
				    <Link to={'/' + this.props.impID + '/' + this.props.forumID + '/' + this.props.subforumID + '/' + this.props.post.id}>
                                    <Avatar cache={cache} size="50" round={true} src={this.state.profilePicture}
                                        name={this.props.post.username}/>
                                    {"  "} {this.props.post.username}
				    </Link>
                                </CardActionArea>
                                <Card.Subtitle className="text-muted mt-1 time-since">
                                    <TimeSince createdTime={this.props.post.createdTime}/>
                                </Card.Subtitle>
                                <Card.Subtitle className="text-muted mt-1 time-since">
                                    <TimeSince createdTime={this.props.post.createdTime} modifiedTime={this.props.post.modifiedTime}/>
                                </Card.Subtitle>
                            </div>
                        </div>
                            
                        <CardActionArea style={{ textDecoration: 'none' }}>
			    <Link to={'/' + this.props.impID + '/' + this.props.forumID + '/' + this.props.subforumID + '/' + this.props.post.id}>
				    <Card.Body className="comment-body">
					<Card.Title className="post-title"> {this.props.post.postTitle}</Card.Title>
					<ReactMarkdown className="post-body">{this.props.post.postContents}</ReactMarkdown>     {/*Use the body from the prop as the body */}
				    </Card.Body>
			    </Link>
                        </CardActionArea>

                        {this.genDropDown()}

                        {/*
                        <ButtonGroup vertical className="buttons">
                            <ButtonGroup>
                                <Button className="button edit-button" title="Edit"
                                   href={"/" + this.props.impID + "/" + this.props.forumID + "/" + this.props.subforumID + "/" + this.props.post.id + "/edit"}>🖉</Button>
                                <Button className='button delete-button' title="Delete" onClick={this.delete} href="#">🗑</Button>
                            </ButtonGroup>
                            <a className="button reply-button" title="Comment"
                               href={"/" + this.props.impID + "/" + this.props.forumID + "/" + this.props.subforumID + "/" + this.props.post.id + "/new"}>Comment</a>
                        </ButtonGroup>
                        */}
                    </div>
                </Card.Body>
            </Card>
        )
    }
}

export default PostPreview

/* eslint-disable jsx-a11y/anchor-is-valid */
const CustomToggle = React.forwardRef(({ children, onClick }, ref) => (
    <a
      href=""
      ref={ref}
      onClick={e => {
        e.preventDefault();
        onClick(e);
      }}
      style={{ zIndex: 2, position: "relative" }}
    >
  
      {children}
      <ThreeDots />
  
    </a>
  ));
