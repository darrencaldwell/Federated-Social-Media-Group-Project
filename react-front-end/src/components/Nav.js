import React from 'react';
import {Link} from 'react-router-dom';


class Nav extends React.Component {

    render() {

        let buttons;

        // If user is logged in (prop), display logout and make post buttons
        if (this.props.isLoggedIn) {
            buttons = (
                <ul className="navbar-nav ml-auto">
                    <li className="nav-item">
                        <Link to={'/login'} onClick={this.props.logout} className="nav-link">Logout</Link>
                    </li>
                    <li className="nav-item">
                        <Link to={'/makePost'} className="nav-link">Make Post</Link>
                    </li>
                    <li className="nav-item">
                        <Link to={'/posts'} className="nav-link">List of posts</Link>
                    </li>
                </ul>)
        } else { // If user is not logged in, display login and register buttons
            buttons = (
                <ul className="navbar-nav ml-auto">
                    <li className="nav-item">
                        <Link to={'/login'} className="nav-link">Login</Link>
                    </li>
                    <li className="nav-item">
                        <Link to={'/register'} className="nav-link">Register</Link>
                    </li>
                </ul>)
        }
        return (
            <nav className="navbar navbar-expand navbar-light fixed-top">
                <div className="container">
                    <Link to={'/'} className="navbar-brand">Home</Link>
                    <div className="collapse navbar-collapse">
                        {buttons}
                    </div>
                </div>

            </nav>
        );

    }
}

export default Nav;
