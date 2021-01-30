import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';
import CreateSubforum from '../components/CreateSubforum';

export default class NewSubforum extends React.Component{
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
                <CreateSubforum forumID={this.state.forumID}/>
            </div>
        );
    }
}