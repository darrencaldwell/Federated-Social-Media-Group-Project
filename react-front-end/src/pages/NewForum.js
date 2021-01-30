import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import CreateForum from '../components/CreateForum';

export default class NewForum extends React.Component{
    render() {
        return(
            <div className="columns">
                <ForumList/>
                <CreateForum/>
            </div>
        );
    }
}