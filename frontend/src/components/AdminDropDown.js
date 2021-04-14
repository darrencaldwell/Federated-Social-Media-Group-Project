import React, {Component} from 'react';
import {Container, Dropdown } from 'react-bootstrap';
import {Link} from 'react-router-dom';
import {ThreeDots} from 'react-bootstrap-icons';    
import '../styling/container-pages.css';

/*
We want a card similiar to our existing card used in forums and subforums, but to include a
drop down menu for deletion, role changing, subscription
*/

// props: link, name, forumID, subforumID, isSubscribed, isModerator, isCreator, id, 
export class AdminDropDown extends Component {

    constructor(props) {
        super(props)
        this.state = {
            // if we have a subforum id then this is a dropdown for a subforum
            // type: (this.props.subforumID === undefined) ? "forum" : "subforum"
        }
    }

    render() {
        return (
            <Container bsPrefix="pr-0 d-flex flex-row admin-dropdown">
                <Dropdown className="admin-dropdown" color="black">  
                    <Dropdown.Toggle as={CustomToggle} variant="success" id="dropdown-basic"/>
                    <Dropdown.Menu>
                        {!this.props.isSubscribed && <Dropdown.Item href="#/action-1">Subscribe</Dropdown.Item>}

                        {this.props.isSubscribed && <Dropdown.Item href="#/action-1">UnSubcribe</Dropdown.Item>}

                        {(!this.props.isModerator || this.props.isCreator) && 
                            <Dropdown.Item as={Link} to={this.props.permsLink}>
                                Edit Permissions
                            </Dropdown.Item>}

                        {this.props.isCreator && <Dropdown.Item href="#/action-3">Delete</Dropdown.Item>}
                    </Dropdown.Menu>
                </Dropdown>
            </Container>
        )
    }
}   
export default AdminDropDown;

// needed to stop clicking dropdown go to the forum/subforum
const CustomToggle = React.forwardRef(({ children, onClick }, ref) => (
    // eslint-disable-next-line
    <a
      href=""
      ref={ref}
      onClick={e => {
        e.preventDefault();
        onClick(e);
      }}
      style={{ zIndex: 2, position: "relative" }}
    >
  
      {children}
      <ThreeDots />
  
    </a>
  ));