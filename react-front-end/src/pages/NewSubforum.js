import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';
import CreateSubforum from '../components/CreateSubforum';

export default class NewSubforum extends React.Component{

    render() {
        return(
            <div className="columns">
                <ForumList/>
                <SubforumList forumID={this.props.match.params.forumID}/>
                <CreateSubforum forumID={this.props.match.params.forumID}/>
            </div>
        );
    }
}