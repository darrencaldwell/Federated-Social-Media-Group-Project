import React, {Component} from 'react';
import { Card, Col, Row, Container, Dropdown } from 'react-bootstrap';
import {Link} from 'react-router-dom';
import {ThreeDots} from 'react-bootstrap-icons';    
import '../styling/container-pages.css';

/*
We want a card similiar to our existing card used in forums and subforums, but to include a
drop down menu for deletion, role changing, subscription
*/

// props: link, name, forumID, subforumID, isSubscribed, isModerator, isCreator
export class ForumCard extends Component {

    constructor(props) {
        super(props)
        this.state = {
            // if we have a subforum id then this is a dropdown for a subforum
            type: (this.props.subforumID === undefined) ? "forum" : "subforum"
        }
    }

    render() {
        return (
            <Card className="forum" >  {/*each forum is displayed as a card with className forum */}
                <Card.Link as={Link} to={this.props.link}>
                    <Card.Body className="pb-3 pt-0"> 
                        <Container className="pr-0 d-flex flex-row justify-content-end">
                            <Dropdown>  
                                <Dropdown.Toggle as={CustomToggle} variant="success" id="dropdown-basic"/>
                                <Dropdown.Menu>
                                    {!this.props.isSubscribed && <Dropdown.Item href="#/action-1">Subscribe</Dropdown.Item>}

                                    {this.props.isSubscribed && <Dropdown.Item href="#/action-1">UnSubcribe</Dropdown.Item>}

                                    {(!this.props.isModerator || this.props.isCreatoor) && 
                                    <Dropdown.Item as={Link} 
                                    to={`/editperms/${this.state.type}/${this.props.subforumID ? this.props.subforumID : this.props.forumID}`}>
                                        Edit Permissions
                                        </Dropdown.Item>}

                                    {this.props.isCreator && <Dropdown.Item href="#/action-3">Delete</Dropdown.Item>}
                                </Dropdown.Menu>
                            </Dropdown>
                        </Container>
                        <Card.Text className="forum-body">
                                {this.props.name}
                        </Card.Text>                    
                    </Card.Body>
                </Card.Link> 
            </Card>
        )
    }
}   
export default ForumCard;

// needed to stop clicking dropdown go to the forum/subforum
const CustomToggle = React.forwardRef(({ children, onClick }, ref) => (
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