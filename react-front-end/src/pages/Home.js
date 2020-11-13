import React from 'react';

class Home extends React.Component {

    render() {

        // If username exists, get username from token and output message
        if (localStorage.getItem('username')) {
            return <h2>Welcome {localStorage.getItem('username')}</h2>
        }

        // Otherwise just return a home page
        return (
            <div>
                <h1> Home Page </h1>
            </div>
        );
    }
}

export default Home;
