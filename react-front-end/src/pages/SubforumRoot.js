import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';
import PostList from '../components/PostList';


export default class SubforumRoot extends React.Component{

    render() {
        return(
            <div className="columns">
                <ForumList/>
                <SubforumList forumID={this.props.match.params.forumID}/>
                <PostList forumID={this.props.match.params.forumID} subforumID={this.props.match.params.subforumID}/>
            </div>
        );
    }
}