import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';

export default class Root extends React.Component{

    render() {
        return(
            <div className="columns">
                <ForumList/>
            </div>
        );
    }
}