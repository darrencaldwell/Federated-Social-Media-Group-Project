import React from 'react';
import {Link} from 'react-router-dom';


class Nav extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {

        let buttons;

        if (this.props.isLoggedIn) {
            buttons = (
                <ul className="navbar-nav ml-auto">
                    <li className="nav-item">
                        <Link to={'/login'} onClick={this.props.logout} className="nav-link">Logout</Link>
                    </li>
                </ul>)
        } else {
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
