import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';

class Expanded extends React.Component{

    render() {
        return(
            <div className="rows">
                <ForumList/>
            </div>
        );
    }
}