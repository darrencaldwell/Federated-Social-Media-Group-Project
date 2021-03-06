import React from "react";
import axios from "axios";
import {Card, Container} from "react-bootstrap";
import PostPreview from '../PostPreview';

class AccountPosts extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            postList: [],
            impId: 1
        }
    }

    componentDidMount = async() =>{
        axios.get('api/users/' + localStorage.getItem('userId') + '/posts')
            .then(res => {
                this.setState({
                    postList: res.data._embedded.postList
                })
                console.log('posts', this.state.postList)
            }).catch(err => {
            if (err.response) {
                alert(err.response.message())
            }
        })
    }

    render() {
        // const start_url = "https://cs3099user-b5.host.cs.st-andrews.ac.uk/1/"
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
                        <PostPreview key={post.id} 
                                     post={post} 
                                     impID='1' 
                                     forumID={JSON.stringify(parseInt(post._links.forum.href.split("/").pop()))} 
                                     subforumID={JSON.stringify(parseInt(post._links.subforum.href.split("/").pop()))}/>
                        // <Card>
                        //     <Card.Body>
                        //         <Card.Title>{post.postTitle}</Card.Title>
                        //         <Card.Subtitle>Post Id: {post.id}</Card.Subtitle>
                        //         <Card.Text>{post.postContents}</Card.Text>
                        //         {console.log('PATH TO POSTS',start_url +
                        //             JSON.stringify(parseInt(post._links.forum.href.split("/").pop())) + '/' +
                        //             JSON.stringify(parseInt(post._links.subforum.href.split("/").pop())) + '/' +
                        //             JSON.stringify(parseInt(post._links.self.href.split("/").pop()
                        //             )))}
                        //         <Card.Link href={start_url +
                        //             JSON.stringify(parseInt(post._links.forum.href.split("/").pop())) + '/' +
                        //             JSON.stringify(parseInt(post._links.subforum.href.split("/").pop())) + '/' +
                        //             JSON.stringify(parseInt(post._links.self.href.split("/").pop()
                        //             ))}>Take me to the post {post.postId}</Card.Link>
                        //     </Card.Body>
                        // </Card>
                    ))}
                </Container>
            )
        }
    }
}

export default AccountPosts