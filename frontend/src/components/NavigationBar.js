import React from 'react';
import {Link} from 'react-router-dom';
import {Nav, Navbar, Dropdown, Button, ButtonGroup} from 'react-bootstrap'
import '../styling/navStyling.css'


class NavigationBar extends React.Component {
    constructor(props) {
        super(props);
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

    componentWillReceiveProps = (props) => {
        this.setState({imps: this.getImps(props)});
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

    getImps = (props) => {
        if (!props.isLoggedIn || props.currImp == null || props.imps == null) return "";
        return (
        <Dropdown as={ButtonGroup}>
          <Button as={Link} to={"/"+props.currImp.id+"/forums"}>{props.currImp.name}</Button>
          <Dropdown.Toggle split id="dropdown-split-basic" />
          <Dropdown.Menu>
            {props.imps.map(imp => {
                return (
                    <Dropdown.Item as={Link} to={"/"+imp.id} onClick={() => this.props.changeImp({name: imp.name, id: imp.id})}>{imp.name}</Dropdown.Item>
                );
            })}
          </Dropdown.Menu>
        </Dropdown>
        );
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
                    {/*<Nav.Link as={Link} to='/forums'>View Forums    </Nav.Link>   redundant*/}
                    <Nav.Link as={Link} to='/' onClick={() => this.props.logout()}>Logout</Nav.Link>
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
                        {this.state.imps}
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
