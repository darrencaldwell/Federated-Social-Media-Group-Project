import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';
import Post from '../components/Post';

export default class ExpandedPost extends React.Component{

    render() {
        return(
            // display the forum list, subforum list and post side-by-side
            <div className="columns">
                <ForumList/>
                <SubforumList forumID={this.props.match.params.forumID}/>
                <Post postID={this.props.match.params.postID} forumID={this.props.match.params.forumID} subforumID={this.props.match.params.subforumID}/>
            </div>
        );
    }
}