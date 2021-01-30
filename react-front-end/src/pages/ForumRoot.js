import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';

export default class ForumRoot extends React.Component{
    constructor(props) {
        super(props);
        this.state = {
            forumID: this.props.match.params.id
        }
    }

    render() {
        return(
            <div className="columns">
                <ForumList/>
                <SubforumList forumID={this.state.forumID}/>
                Forum information goes here
            </div>
        );
    }
}