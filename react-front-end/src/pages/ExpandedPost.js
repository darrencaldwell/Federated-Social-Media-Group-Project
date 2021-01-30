import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';
import Post from '../components/Post';

export default class ExpandedPost extends React.Component{
    constructor(props) {
        super(props);
        this.state = {
            subforumID : {},
            forumID : {}
        }
    }

    // fetch the subforum and forum ID from the post
    componentDidMount = async() => {
        let postURL = '/api/posts/' + this.props.match.params.id;
        try {
            let res = await fetch((postURL) // we're making a request to this url
                , {
                    method: 'GET', // this is a GET request

                    withCredentials: true, // we're using authorisation, with an auth token in local storage
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );

            let result = await res.json(); // we know the result is json
            this.setState( {subforumID : result.subforumId} ); // we just want the subforum id from the json

            let subforumURL = '/api/subforums/' + result.subforumId;

            res = await fetch((subforumURL)
                , {
                    method: 'GET',
                    withCredentials: true,
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );
            result = await res.json();
            this.setState( {forumID : result.forumId} );
        } catch (e) {
            console.log(e);
        }
    }

    render() {
        return(
            // display the forum list, subforum list and post side-by-side
            <div className="columns">
                <ForumList/>
                <SubforumList forumID={this.state.forumID}/>
                <Post postID={this.props.match.params.id} subforumID={this.state.subforumID}/>
            </div>
        );
    }
}