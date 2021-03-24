import React from "react";
import axios from "axios";
import {Card, Container} from "react-bootstrap";

class AccountPosts extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            postList: [],
            impId: 1
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
        const start_url = "https://cs3099user-b5.host.cs.st-andrews.ac.uk/1/"
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
                        // {const forum_Id = post._links.self.href.pop()}
                        <Card>
                            <Card.Body>
                                <Card.Title>{post.postTitle}</Card.Title>
                                <Card.Subtitle>Post Id: {post.id}</Card.Subtitle>
                                <Card.Text>{post.postContents}</Card.Text>
                                {console.log('PATH TO POSTS',start_url +
                                    JSON.stringify(parseInt(post._links.forum.href.split("/").pop())) + '/' +
                                    JSON.stringify(parseInt(post._links.subforum.href.split("/").pop())) + '/' +
                                    JSON.stringify(parseInt(post._links.self.href.split("/").pop()
                                    )))}
                                <Card.Link href={start_url +
                                    JSON.stringify(parseInt(post._links.forum.href.split("/").pop())) + '/' +
                                    JSON.stringify(parseInt(post._links.subforum.href.split("/").pop())) + '/' +
                                    JSON.stringify(parseInt(post._links.self.href.split("/").pop()
                                    ))}>Take me to the post {post.postId}</Card.Link>
                            </Card.Body>
                        </Card>
                    ))}
                </Container>
            )
        }
    }
}

export default AccountPosts