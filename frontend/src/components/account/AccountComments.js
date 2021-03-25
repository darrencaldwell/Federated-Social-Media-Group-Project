import React from "react";
import axios from "axios";
import {Card, Container} from "react-bootstrap";

class AccountComments extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            commentList: []
        }
    }

    componentDidMount() {
        axios.get('api/users/' + localStorage.getItem('userId') + '/comments')
            .then(res => {
                this.setState({
                    commentList: res.data._embedded.commentList
                })
                // will remove once modified time feature is done
                // console.log(this.state.commentList)
                // console.log(this.state.commentList[0]._links)
                // console.log(this.state.commentList[0]._links.post)
            }).catch(err => {
            alert("something went wrong")
        })
    }


    render() {
        // console.log(this.state.commentList[0]._links.post)

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
                                {/*{console.log('PATH TO COMMENTS',start_url +*/}
                                {/*    JSON.stringify(parseInt(post._links.forum.href.split("/").pop())) + '/' +*/}
                                {/*    JSON.stringify(parseInt(post._links.subforum.href.split("/").pop())) + '/' +*/}
                                {/*    JSON.stringify(parseInt(post._links.self.href.split("/").pop()*/}
                                {/*    )))}*/}
                                {/*<Card.Link href={start_url +*/}
                                {/*JSON.stringify(parseInt(post._links.forum.href.split("/").pop())) + '/' +*/}
                                {/*JSON.stringify(parseInt(post._links.subforum.href.split("/").pop())) + '/' +*/}
                                {/*JSON.stringify(parseInt(post._links.self.href.split("/").pop()*/}
                                {/*))}>Take me to the post {post.postId}</Card.Link>*/}
                            </Card.Body>
                        </Card>
                    ))}
                </Container>


            )
        }
    }
}

export default AccountComments