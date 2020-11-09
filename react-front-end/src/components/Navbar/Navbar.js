import React from 'react';
import '../../styling/Navbar.css'
import {Link} from 'react-router-dom';
// import Login from "../../pages/Login";
// import Register from "../../pages/Register";

class Navbar extends React.Component {

    render() {
        return (
            <nav>
                <h1 className="navbar-logo">CS3099</h1>
                <ul className="nav-links">
                    <Link to="/">
                        <li>Home</li>
                    </Link>
                    <Link to="/login">
                        <li>Login</li>
                    </Link>
                    <Link to="/register">
                        <li>Register</li>
                    </Link>
                    <Link to="/posts">
                        <li>Posts</li>
                    </Link>
                    <Link to="/post">
                        <li>Post</li>
                    </Link>
                </ul>
            </nav>
        );
    }
}

export default Navbar;