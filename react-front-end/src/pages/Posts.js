import React, { Component } from 'react';
import { BrowserRouter as Router, Link } from 'react-router-dom';

export class Posts extends Component {
    render() {
        return (
            <div>
                <h1>
                    {this.props.post.postTitle}
                </h1>
                <p>
                    {this.props.post.postContents}
                </p>
                <Router>
                    <p>
                        <Link to={this.props.post._links.self.href} onClick={() => this.props.expandPost(this.props.post._links.self.href)}>
                            Expand Post (only one that works rn).
                        </Link>
                    </p>

                </Router>
            </div>
        )
    }
}

export default Posts
