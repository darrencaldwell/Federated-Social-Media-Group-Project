import React from 'react';
import '../styling/container-pages.css';
import ForumList from '../components/ForumList';
import SubforumList from '../components/SubforumList';
import PostList from '../components/PostList';


export default class SubforumRoot extends React.Component{
    constructor(props) {
        super(props);
        this.state = {
            forumID : {}
        }
    }

    ComponentDidMount = async() => {
        let url = '/api/subforums/' + this.props.match.params.id;
        try {
            let res = await fetch((url)
                , {
                    method: 'GET',
                    withCredentials: true,
                    credentials: 'include',
                    headers: {
                        'Authorization': "Bearer " + localStorage.getItem('token'),
                        'Content-Type': 'application/json',
                        'Accept': 'application/json'
                    }
                }
            );
            let result = await res.json();
            this.setState({forumID : result.forumId});
        } catch (e) {
            console.log(e);
            console.log("Failed to fetch forum ID");
        }
    }

    render() {
        return(
            <div className="columns">
                <ForumList/>
                <SubforumList forumID={this.state.forumID}/>
                <PostList subforumID={this.props.match.params.id}/>
            </div>
        );
    }
}