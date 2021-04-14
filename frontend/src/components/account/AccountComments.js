import {React, Component} from 'react';
import axios from "axios";
import {Card, Container, Button, ButtonGroup} from "react-bootstrap";
import CardActionArea from '@material-ui/core/CardActionArea';
import Avatar, {Cache} from 'react-avatar';
import Voting from '../Voting'
import TimeSince from '../TimeSince';
import '../../styling/container-pages.css';
import ReactMarkdown from 'react-markdown';

// for react avatar
const cache = new Cache({

    // Keep cached source failures for up to 7 days
    sourceTTL: 7 * 24 * 3600 * 1000,

    // Keep a maximum of 0 entries in the source cache (we don't care about remembering broken links!)
    sourceSize: 0
});

class AccountComments extends Component {

    constructor(props) {
        console.log('comment got here 1')
        super(props);
        this.state = {
            commentList: []
        }
    }

    componentDidMount = async() => {
        console.log('comment got here 2')
        axios.get('api/users/' + localStorage.getItem('userId') + '/comments')
            .then(res => {
                this.setState({
                    commentList: res.data._embedded.commentList
                })
            }).catch(err => {
            alert("something went wrong")
        })
    }


    render() {
        const start_url = "https://cs3099user-b5.host.cs.st-andrews.ac.uk/1/"
        console.log('comment got here 3')
        if (this.state.commentList.length === 0) {
            return (
                <Container>
                    <Card>
                        <Card.Body>
                            You have made no comments.
                        </Card.Body>
                    </Card>
                </Container>
            )
        } else {
            return (
                <Container>
                    <Card.Title>Your Comments</Card.Title>
                    {/* this is just a crude copy-paste, but it should work. Ideally this should use the Comments component. */}
                    {this.state.commentList.map((comment) => (
                        <Card border="dark small-separator">
                        <Card.Body>
                            <div className="comment-columns">
                                <div className="post-comment-voting-container">
                                    <Voting className="voting-post"
                                            upvotes={comment.upvotes}
                                            downvotes={comment.downvotes}
                                            _userVotes={comment._userVotes}
                                            type="comments"
                                            postID={comment.id}
                                            impID="1"
                                    ></Voting>
                                    <div className="voting-adj">
                                        <CardActionArea href={'/user/' + btoa(comment._links.user.href)}>
                                            <Avatar cache={cache} size="50" round={true} src={this.state.profilePicture}
                                                name={comment.username}/>
                                            {"  "} {comment.username}
                                        </CardActionArea>
                                        <Card.Subtitle className="text-muted mt-1 time-since">
                                            <TimeSince createdTime={comment.createdTime}/>
                                        </Card.Subtitle>
                                        <Card.Subtitle className="text-muted mt-1 time-since">
                                            <TimeSince createdTime={comment.createdTime}
                                                       modifiedTime={comment.modifiedTime}/>
                                        </Card.Subtitle>
                                    </div>
                                </div>
                                <ReactMarkdown className="mt-3 comment-body">{comment.commentContent}</ReactMarkdown>
                                <ButtonGroup vertical className="buttons">
                                    <ButtonGroup>        
                                        <Button className="button edit-button"
                                           href={"/1" + JSON.stringify(parseInt(comment._links.forum.href.split("/").pop())) + 
                                                  "/" + JSON.stringify(parseInt(comment._links.subforum.href.split("/").pop())) +
                                                  "/" + comment.postId + 
                                                  "/" + comment.id + "/edit"}>🖉</Button>
                                        <Button className='button delete-button' onClick={() => {
                                            if (window.confirm('Are you sure you wish to delete this comment?\n THIS CANNOT BE UNDONE!')) this.delete()}}
                                           href={"."}>🗑</Button>
                                    </ButtonGroup>
                                    <a className="button reply-button"
                                       href={"/1" + JSON.stringify(parseInt(comment._links.forum.href.split("/").pop())) + 
                                              "/" + JSON.stringify(parseInt(comment._links.subforum.href.split("/").pop())) +
                                              "/" + comment.postId + 
                                              "/" + comment.id + "/new"}>Reply</a>
                                </ButtonGroup>
                            </div>
                        </Card.Body>
        
                    </Card>


                        // <Card>
                        //     <Card.Body>
                        //         <Card.Title>Comment by {comment.username}</Card.Title>
                        //         <Card.Subtitle>Comment Id: {comment.id} Post Id: {comment.postId}</Card.Subtitle>
                        //         <Card.Text>{comment.commentContent}</Card.Text>
                        //         {/*{console.log('PATH TO COMMENTS',start_url +*/}
                        //         {/*    JSON.stringify(parseInt(post._links.forum.href.split("/").pop())) + '/' +*/}
                        //         {/*    JSON.stringify(parseInt(post._links.subforum.href.split("/").pop())) + '/' +*/}
                        //         {/*    JSON.stringify(parseInt(post._links.self.href.split("/").pop()*/}
                        //         {/*    )))}*/}
                        //         {/*<Card.Link href={start_url +*/}
                        //         {/*JSON.stringify(parseInt(post._links.forum.href.split("/").pop())) + '/' +*/}
                        //         {/*JSON.stringify(parseInt(post._links.subforum.href.split("/").pop())) + '/' +*/}
                        //         {/*JSON.stringify(parseInt(post._links.self.href.split("/").pop()*/}
                        //         {/*))}>Take me to the post {post.postId}</Card.Link>*/}
                        //     </Card.Body>
                        // </Card>
                    ))}
                </Container>


            )
        }
    }
}

export default AccountComments