import React from "react";
import axios from "axios";
import {Card, Container} from "react-bootstrap";

class AccountComments extends React.Component {

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
                    {this.state.commentList.map((comment) => (
                        <Card>
                            <Card.Body>
                                <Card.Title>Comment by {comment.username}</Card.Title>
                                <Card.Subtitle>Comment Id: {comment.id} Post Id: {comment.postId}</Card.Subtitle>
                                <Card.Text>{comment.commentContent}</Card.Text>
                                

                                {console.log('post stuff', comment)}
                                {console.log('PATH TO POSTS',start_url +
                                    JSON.stringify(parseInt(comment._links.forum.href.split("/").pop())) + '/' +
                                    JSON.stringify(parseInt(comment._links.subforum.href.split("/").pop())) + '/' +
                                    JSON.stringify(parseInt(comment._links.post.href.split("/").pop()
                                    )))}
                                <Card.Link href={start_url +
                                JSON.stringify(parseInt(comment._links.forum.href.split("/").pop())) + '/' +
                                JSON.stringify(parseInt(comment._links.subforum.href.split("/").pop())) + '/' +
                                JSON.stringify(parseInt(comment._links.post.href.split("/").pop()
                                ))}>Take me to the comment {comment.id}</Card.Link>
                            </Card.Body>
                        </Card>
                    ))}
                </Container>


            )
        }
    }
}

export default AccountComments