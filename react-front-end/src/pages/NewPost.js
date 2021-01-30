import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';
import CreatePost from '../components/CreatePost';

export default class NewPost extends React.Component{

    render() {
        return(
            <div className="columns">
                <ForumList/>
                <SubforumList forumID={this.props.match.params.forumID}/>
                <CreatePost subforumID={this.props.match.params.subforumID}/>
            </div>
        );
    }
}