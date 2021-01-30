import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';

export default class ForumRoot extends React.Component{

    render() {
        return(
            <div className="columns">
                <ForumList/>
                <SubforumList forumID={this.props.match.params.forumID}/>
                Forum information goes here
            </div>
        );
    }
}