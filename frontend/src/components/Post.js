import React, {Component} from 'react';
//import { BrowserRouter as Router, Link } from 'react-router-dom';
import Comments from './Comments';
// import CreatePost from './CreatePost.js';
import BackButton from './BackButton';
// import '../styling/Post.css';
import {Card, Container, Spinner} from "react-bootstrap";
import {Link} from 'react-router-dom';
import Voting from './Voting';
import TimeSince from './TimeSince';

// props: match.params.impID, match.params.postID, match.params.subforumID, match.params.forumID, match.params.commentID
export class Post extends Component {

    constructor(props) {
        super(props);
        const expanded = (typeof this.props.match.params.commentID != 'undefined'); // it's an expanded comment if the url has the comment id
        this.state = {
            expanded: expanded,
            loading: true, // Set to true if loading
            post: {}, // the post is stored here once loaded
            post_author: {},
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

        const backURL = "/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID;
        const url = this.state.expanded ? ('/api/comments/' + this.props.match.params.commentID + '/comments')
                                        : ('/api/posts/' + this.props.match.params.postID + '/comments');
        console.log('post', this.state.post)
        const parsed_user_link = btoa(this.state.post._links.user.href)

        return (
            <Container className="post-container">
                <BackButton url={backURL}/>
                <div className="mt-3">
                    <Card border="dark">
                        <Card.Body>
                        <div className="post-comment-voting-container">
                            <Voting className="voting"
                                upvotes={this.state.post.upvotes} 
                                downvotes={this.state.post.downvotes} 
                                _userVotes={this.state.post._userVotes}
                                type="posts"
                                postID={this.props.match.params.postID}
                                impID={this.props.match.params.impID}
                            ></Voting>
                            <div className="voting-adj">
                            <Card.Title>{this.state.post.postTitle}</Card.Title>
                                {/*<Route exact path="/user/:id" component={() => <UserAccount user={this.state.post_author}/>}/>*/}
                            <Card.Subtitle className="text-muted">
                                {/*<Account user_id={this.state.post_author.userId}/>*/}
                                {/*{console.log('info on post author',this.state.post_author)}*/}
                                {/*{console.log('before linking to /user/'+this.state.post_author.userId)}*/}
                                {/*{console.log('user link', this.state.post_author._links.self.href)}*/}

                                {/*<Route name="" path="/users/:id" handler={User} />*/}
                                {/*<UserAccount user={this.state.post_author}/>*/}
                                {/*Post made by: <Link to={{pathname:"/users/" + this.state.post_author.userId, props:{user: this.state.post_author}}}>{this.state.post_author.user}</Link>*/}
                                {/*<Route exact path="/user/:id" component={() => <UserAccount user={this.state.post_author}/>}/>*/}
                                {/*Post made by: <Card.Link to={'/user/' + this.state.post_author} user={this.state.post_author}>{this.state.post_author}</Card.Link>*/}
                                <Card.Link href={'/user/' + parsed_user_link}>{this.state.post.username}</Card.Link>
                                {/*Post made by: <Card.Link href={"/user/" + this.state.post_author.userId}>{this.state.post_author.username}</Card.Link> on TIME*/}
                            </Card.Subtitle>
                            </div>
                            </div>
                            <Card.Body>
                               <Card.Text>{this.state.post.postContents}</Card.Text>

                               <Card.Link as={Link} to={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID + "/new"}> Create Comment</Card.Link>
                            </Card.Body>
                        </Card.Body>
                    </Card>
                </div>

                {/*<CreateComment url={url}/>*/}

                {/*<Dropdown className="mt-3">*/}
                {/*<Dropdown.Toggle variant="light" id="dropdown-comments">View Comments</Dropdown.Toggle>*/}
                {/*<Dropdown.Menu>*/}
                <Comments url={url} impID={this.props.match.params.impID} expanded={this.state.expanded}
                        posturl={"/" + this.props.match.params.impID + "/" + this.props.match.params.forumID + "/" + this.props.match.params.subforumID + "/" + this.props.match.params.postID}/>
                {/*</Dropdown.Menu>*/}
                {/*</Dropdown>*/}

            </Container>
        )
    }
}
export default Post
