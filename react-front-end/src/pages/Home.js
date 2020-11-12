import React from 'react';

class Home extends React.Component {

    render() {

        if (localStorage.getItem('username')) {
            return <h2>Welcome {localStorage.getItem('username')}</h2>
        }

        return (
            <div>
                <h1> Home Page </h1>
            </div>
        );
    }
}

export default Home;
