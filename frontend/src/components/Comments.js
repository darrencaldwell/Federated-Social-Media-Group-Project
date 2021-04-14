import {React, Component} from 'react'
import {Link} from 'react-router-dom'
import {Card, Container, Button, ButtonGroup} from "react-bootstrap";
import CardActionArea from '@material-ui/core/CardActionArea';
import Avatar, {Cache} from 'react-avatar';
import ReactMarkdown from 'react-markdown';
import Voting from './Voting'
import TimeSince from './TimeSince';
import '../styling/container-pages.css';

// for react avatar
const cache = new Cache({

    // Keep cached source failures for up to 7 days
    sourceTTL: 7 * 24 * 3600 * 1000,

    // Keep a maximum of 0 entries in the source cache (we don't care about remembering broken links!)
    sourceSize: 0
});

// props: comment (json), posturl, impID, level
class Comment extends Component {

    constructor(props) {
        super(props);
        this.delete = this.delete.bind(this); // bind this so it can override onClick
        this.state = {
            loading: true, // Set to true if loading
            parentID: this.props.comment._links.parentComment.href.substring(this.props.comment._links.parentComment.href.lastIndexOf('/') + 1)
        }
    }

    componentDidUpdate = (prevProps) => {
        if (this.props.url !== prevProps.url) {
            this.componentDidMount();
        }
    }

    // Runs when the component is loaded, fetching the details of the user who created the comment
    componentDidMount = async () => {
        try {
            // the url to make the request to is given by the parent
            let url = "/api/users/" + this.props.comment.userId
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect-url': this.props.comment._links.user.href
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

        if (window.confirm('Are you sure you wish to delete this comment?\n THIS CANNOT BE UNDONE!')) {
            // this is the HTML request
            fetch("/api/comments/" + this.props.comment.id, {
                method: "DELETE",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                    'Content-Type': 'application/json',
                    'redirect': this.props.impID
                }

            }).then(responseJson => { // log the response for debugging
                console.log(responseJson);
                if (responseJson.status === 200) {
                    alert("Successfully deleted comment.");
                    window.location.href = this.props.posturl;
                }
            }).catch(error => this.setState({ // catch any error
                message: "Error posting post: " + error
            }));
        }
    }

    render() {
        if (this.state.loading) {
            return null
        }

        const parsed_user_link = btoa(this.props.comment._links.user.href)

        return (
            <Card border="dark small-separator">
                <Card.Body>
                    <div className="comment-columns">
                        <div className="post-comment-voting-container">
                            <Voting className="voting-post"
                                    upvotes={this.props.comment.upvotes}
                                    downvotes={this.props.comment.downvotes}
                                    _userVotes={this.props.comment._userVotes}
                                    type="comments"
                                    postID={this.props.comment.id}
                                    impID={this.props.impID}
                            ></Voting>
                            <div className="voting-adj">
                                <CardActionArea href={'/user/' + parsed_user_link}>
                                    <Avatar cache={cache} size="50" round={true} src={this.state.profilePicture}
                                        name={this.props.comment.username}/>
                                    {"  "} {this.props.comment.username}
                                </CardActionArea>
                                <Card.Subtitle className="text-muted mt-1 time-since">
                                    <TimeSince createdTime={this.props.comment.createdTime}/>
                                </Card.Subtitle>
                                <Card.Subtitle className="text-muted mt-1 time-since">
                                    <TimeSince createdTime={this.props.comment.createdTime}
                                               modifiedTime={this.props.comment.modifiedTime}/>
                                </Card.Subtitle>
                            </div>
                        </div>
                        <ReactMarkdown className="mt-3 comment-body">{this.props.comment.commentContent}</ReactMarkdown>
                        <ButtonGroup vertical className="buttons">
                            <ButtonGroup>
                                <Button className="button edit-button" title="Edit"
                                   href={this.props.posturl + "/" + this.props.comment.id + "/edit"}>🖉</Button>
                                <Button className='button delete-button' title="Delete" onClick={this.delete} href="#">🗑</Button>
                            </ButtonGroup>
                            <a className="button reply-button" title="Reply"
                               href={this.props.posturl + "/" + this.props.comment.id + "/new"}>Reply</a>
                        </ButtonGroup>
                    </div>
                </Card.Body>
                <Comments url={"/api/comments/" + this.props.comment.id + "/comments"}
                          impID={this.props.impID} posturl={this.props.posturl}
                          level={this.props.level + 1} commentID={this.props.comment.id}
                          parentID={this.state.parentID}/>

            </Card>
        )
    }
}

// props: url, posturl, impID, level, commentID, parentID
export default class Comments extends Component {
    constructor(props) {
        super(props);
        const root = (typeof this.props.level == 'undefined'); // it's a root comment if the comment ID is undefined
        const level = root ? (0)
                           : (this.props.level);
        this.state = {
            level: level,
            commentList: [] // the list of comments will be stored here
        }
    }

    componentDidUpdate = (prevProps) => {
        if (this.props.url !== prevProps.url) {
            this.componentDidMount();
        }
    }

    // Runs when the component is loaded, fetching the list of comments into state
    componentDidMount = async () => {
        try {
            // the url to make the request to is given by the parent
            let url = this.props.url;
            let res = await fetch(url
                , {
                    method: 'get', // we're making a GET request

                    withCredentials: true, // we're using authorisation with a token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect': this.props.impID
                    }
                }
            );

            let result = await res.json(); // we know the result will be json
            this.setState({commentList: result._embedded.commentList}); // we store the json for the list of comments in the state

        } catch (e) {
            console.log(e);
        }
    }

    render() {
        if (this.state.expanded) {  // provide a link to return to the post. Might not need this anymore as back button now goes back to post when expanded.
            return (
                <Container>
                    <Button className="button" as={Link} to={this.props.posturl}>Return</Button>
                    <Comments url={this.props.url} impID={this.props.impID} expanded={false}
                              posturl={this.props.posturl}/>
                </Container>
            )
        } else if (this.state.level >= 3) { // to prevent cramped elements due to heavy nesting
            return (
                <Container>
                    <a className="button expand-button" href={this.props.posturl + "/" + this.props.parentID}>Expand</a>
                </Container>
            )
        } else {

            // if there are comments, display them
            return (
                <Container>
                    {/*map is used to apply this html for each comment in the list */}
                    {this.state.commentList.map((comment) => (
                        // the Comment element above is used for this, which takes the comment json
                        <Comment key={comment.id} comment={comment} impID={this.props.impID} level={this.state.level}
                                 posturl={this.props.posturl}/>
                    ))}
                </Container>
            )
        }
    }
}
