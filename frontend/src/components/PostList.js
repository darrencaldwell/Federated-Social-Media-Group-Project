import React, {Component} from 'react';
import PostPreview from './PostPreview';
import {Alert, Container, Spinner, Button} from "react-bootstrap";
import {Link} from 'react-router-dom';
import '../styling/container-pages.css';

// props: match.params.impID, match.params.forumID, match.params.subforumID
class PostList extends Component {

    constructor(props) {
        super(props)
        this.state = {
            loading: true, // Set to true if loading
            postList: {}, // Stores list of posts
            listingPosts: false, // Set to true when rendering a list of posts
            update: false,

            // post listing has been moved to keep design in line with the new navigation structure
            // listingPost: false // Set to true when rendering a specific post
        }
    }

    getPosts = async () => {
        try {
            // while fetching the list of posts, show a loading graphic
            this.setState({loading: true, listingPosts: false, listingPost: false});
            // the url needs the subforum id from the props
            let url = '/api/subforums/' + this.props.match.params.subforumID + '/posts';
            let res = await fetch(url
                , {
                    method: 'get',
                    withCredentials: true,
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Accept': 'application/json',
                        'redirect': this.props.match.params.impID
                    }
                }
            );
            let result_posts = await res.json();

            this.setState({postList: result_posts._embedded.postList, loading: false, listingPosts: true, update: false});
        } catch (e) {
            this.setState({loading: false, update: false});
        }
    }

    // Runs when the component is loaded, fetching the list of posts into state
    componentDidMount = () => {
        this.getPosts();
    }

    // url is not a prop!
    componentDidUpdate = (oldProps) => {
        if ( this.props.match.params.forumID !== oldProps.match.params.forumID ||
             this.props.match.params.subforumID !== oldProps.match.params.subforumID ||
             this.state.update) {
            this.getPosts();
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

        } else if (this.state.listingPosts) {
            // If we are rendering a list of posts go through the returned array of posts and display them.
            return (
                <div className="post-wrapper">
                    <Container bsPrefix="postlist">
                        {/*map is used to apply this html for each post in the list */}
                        {this.state.postList.map((post) => (
                            // the PostPreview element is used for this, which takes the post id and the post json
                            <PostPreview update={()=>this.setState({update: true})} key={post.id} post={post} impID={this.props.match.params.impID} forumID={this.props.match.params.forumID} subforumID={this.props.match.params.subforumID}/>
                        ))}
                        <div className="separator"/>
                    </Container>
                    <Button bsPrefix="button" as={Link} to={'/' + this.props.match.params.impID + '/' + this.props.match.params.forumID + '/' + this.props.match.params.subforumID + '/new'}>
                        New Post
                    </Button>
                </div>)


        // } else if (this.state.listingPost) {
        //     // If we are rendering a singular post display everything like comments and other things to be implemented later
        //     return (
        //         <Container>
        //             <Post post={this.state.post} comments={this.state.comments._embedded}
        //                   loadPosts={this.componentDidMount}/>
        //         </Container>
        //     )

        } else {
            // if not loading, and not listing posts, an error must have happened
            return (
                <Container>
                    <Alert>Error has occurred.</Alert>
                </Container>
            )
        }
    }
}

export default PostList;
