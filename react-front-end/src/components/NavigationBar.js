import React from 'react';
import {Link} from 'react-router-dom';
import {Nav, Navbar} from 'react-bootstrap'


class NavigationBar extends React.Component {
    constructor() {
        super();
        this.state = {
            navExpanded: false
        }
    }

    setNavExpanded = (expanded) => {
        this.setState({navExpanded: expanded});
    }

    setNavClose = () => {
        this.setState({navExpanded: false});
    }

    container = React.createRef();
    state = {
        open: false,
    }

    componentDidMount() {
        document.addEventListener("mousedown", this.handleClickOutside);
    }
    componentWillUnmount() {
        document.removeEventListener("mousedown", this.handleClickOutside);
    }

    // When Navbar is open and somewhere outside of nav bar is clicked. Close the navbar
    handleClickOutside = event => {
        if (this.container.current && !this.container.current.contains(event.target)) {
            this.setState({
                navExpanded: false
            });
        }
    }

    handleLogout = () => { // used to update App state from within navbar
        localStorage.clear();
        this.props.setState(null)
    }

    render() {
        let buttons;

        // If user is logged in (prop), display logout and make post buttons
        if (this.props.isLoggedIn) {
            buttons = (
                <Nav className="mr-auto" onClick={this.setNavClose}>
                    <Nav.Link as={Link} to='/'>Home</Nav.Link>
                    {/*<Nav.Link as={Link} to='/communities'>My Communities</Nav.Link>*/}
                    <Nav.Link as={Link} to='/account'>My account</Nav.Link>
                    <Nav.Link as={Link} to='/forums'>View Forums    </Nav.Link>
                    <Nav.Link as={Link} to='/' onClick={() => localStorage.clear()}>Logout</Nav.Link>
                </Nav>
            )

        } else { // If user is not logged in, display login and register buttons
            buttons = (
                <Nav className="mr-auto" onClick={this.setNavClose}>
                    <Nav.Link as={Link} to='/'>Home</Nav.Link>
                    <Nav.Link as={Link} to='/login'>Login</Nav.Link>
                    <Nav.Link as={Link} to='/register'>Register</Nav.Link>
                </Nav>
            )
        }
        return (
            <div ref={this.container}>
                <div>
                    <Navbar collapseOnSelect expand="lg" bg="light" variant="light" fixed="top" onToggle={this.setNavExpanded} expanded={this.state.navExpanded}>
                        <Navbar.Brand as={Link} to='/'>CS3099 B5</Navbar.Brand>
                        <Navbar.Toggle aria-controls="basic-navbar-nav"/>
                        <Navbar.Collapse id="basic-navbar-nav">
                            <Nav className="ml-auto" onClick={this.setNavClose}>
                                {buttons}
                            </Nav>
                        </Navbar.Collapse>
                    </Navbar>
                </div>
            </div>
        );

    }
}

export default NavigationBar;
