import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import CreateForum from '../components/CreateForum';

class Expanded extends React.Component{
    constructor(props) {
        const forumID = this.props.match.params.id;
    }

    render() {
        return(
            <div className="rows">
                <ForumList/>
                <CreateForum/>
            </div>
        );
    }
}