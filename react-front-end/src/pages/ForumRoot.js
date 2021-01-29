import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';

class Expanded extends React.Component{
    constructor(props) {
        const forumID = this.props.match.params.id;
    }

    render() {
        return(
            <div className="rows">
                <ForumList/>
                <SubforumList forumID={this.forumID}/>
                {/*Forum information goes here*/}
            </div>
        );
    }
}