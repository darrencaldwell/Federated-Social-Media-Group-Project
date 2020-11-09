import React from 'react';

class Home extends React.Component {
    render() {

        if (this.props.user) {
            return <h2>Welcome {this.props.user.firstName} {this.props.user.lastName}</h2>
        }
        return (
            <div>
                <h1> Home Page </h1>
            </div>
        );
    }
}

export default Home;