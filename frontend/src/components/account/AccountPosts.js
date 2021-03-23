import React from "react";
import axios from "axios";
import {Card, Container} from "react-bootstrap";

class AccountPosts extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            postList: []
        }
    }

    componentDidMount() {
        axios.get('api/users/' + localStorage.getItem('userId') + '/posts')
            .then(res => {
                this.setState({
                    postList: res.data._embedded.postList
                })
                console.log(this.state.postList)
                // console.log(this.state.commentList[0]._links)
                // console.log(this.state.commentList[0]._links.post)
            }).catch(err => {
            alert("something went wrong")
        })
    }

    render() {
        // console.log(this.state.postList)
        if (this.state.postList.length === 0) {
            return (
                <Container>
                    <Card>
                        <Card.Body>
                            You have made no posts.
                        </Card.Body>
                    </Card>
                </Container>
            )
        } else {
            return (
                <Container>
                    <Card.Title>Your Posts</Card.Title>
                    {this.state.postList.map((post) => (
                        <Card>
                            <Card.Body>
                                <Card.Title>{post.postTitle}</Card.Title>
                                <Card.Subtitle>Post Id: {post.id}</Card.Subtitle>
                                <Card.Text>{post.postContents}</Card.Text>
                                <Card.Link href={post._links.self.href}>Take me to the post {post.postId}</Card.Link>
                                {/*<Card.Link href={post._links.self.href}>Take me to my comment</Card.Link>*/}
                            </Card.Body>
                        </Card>
                    ))}
                </Container>
            )
        }
    }
}

export default AccountPosts