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

    delete = (e) => {
        e.preventDefault()
        let url = (this.props.subforumID !== undefined) ? 
            `/local/subforums/${this.props.subforumID}`:
            `/local/forums/${this.props.forumID}`;

        if (window.confirm('Are you sure you wish to delete this comment?\n THIS CANNOT BE UNDONE!')) {
            fetch(url, {
                method: "DELETE",
                withCredentials: true,
                credentials: 'include',
                headers: {
                    'Authorization': "Bearer " + localStorage.getItem('token'), //need the auth token
                    'Content-Type': 'application/json',
                    'redirect': 1
                }
            }).then(responseJson => {
                if (responseJson.status === 200) {
                    this.props.refresh();
                }
            })
        }
    }

    render() {
        return (
            <Container bsPrefix="pr-0 d-flex flex-row admin-dropdown">
                <Dropdown className="admin-dropdown" color="black">  
                    <Dropdown.Toggle as={CustomToggle} variant="success" id="dropdown-basic"/>
                    <Dropdown.Menu>
                        {(!this.props.isModerator || this.props.isCreator) && 
                            <Dropdown.Item as={Link} to={this.props.permsLink}>
                                Edit Permissions
                            </Dropdown.Item>}
                        {this.props.subforumID === undefined &&
                            <Dropdown.Item as={Link} to={`/1/${this.props.forumID}/chat`}>
                                Chat
                            </Dropdown.Item>}

                        {(this.props.isCreator || this.props.impID !== 1) &&
                                <Dropdown.Item onClick={this.delete}>Delete</Dropdown.Item>}
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
