import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';
import Post from '../components/Post';

class Expanded extends React.Component{
    constructor(props) {
        const subforumID = fetchSubforumId(this.props.match.params.id);
        const forumID = fetchForumId(subforumID);
    }

    // fetch the subforum ID from the post
    fetchSubforumId(postID) {
        try {
            let res = await fetch(('/api/posts/${postID}') // we're making a request to this url
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
            return result.subforumId; // we just want the subforum id from the json
        } catch (e) {
            console.log(e);
            console.log("Failed to fetch forum ID");
        }
    }

    // as above, but for the forum
    fetchForumId(subforumID) {
        try {
            let res = await fetch(('/api/subforums/${subforumID}')
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
            let result = await res.json();
            return result.forumId;
        } catch (e) {
            console.log(e);
            console.log("Failed to fetch subforum ID");
        }
    }

    render() {
        return(
            // display the forum list, subforum list and post side-by-side
            <div className="rows">
                <ForumList/>
                <SubforumList forumID={this.forumID}/>
                <Post postID={this.props.match.params.id}/>
            </div>
        );
    }
}