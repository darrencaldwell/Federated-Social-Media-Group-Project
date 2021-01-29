import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';
import PostList from './PostList';

//props: forumID
class Expanded extends React.Component{
    constructor(props) {
        const forumID = fetchForumId(this.props.match.params.id);
    }

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
            console.log("Failed to fetch forum ID");
        }
    }

    render() {
        return(
            <div className="rows">
                <ForumList/>
                <SubforumList forumID={this.forumID}/>
                <PostList subforumID={this.props.match.params.id}/>
            </div>
        );
    }
}