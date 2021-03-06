import React from "react";
import {Button, Container} from "react-bootstrap";
import AssignRoles from './AssignRoles';
import ModifyRoles from './ModifyRoles';
import ViewRoles from './ViewRoles';


// props: type, name
class EditPerms extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            activeComponent: null
        }
    }

    assign_roles = () => {
        if (this.state.activeComponent !== <AssignRoles/>) {
          this.setState({
                activeComponent: <AssignRoles forumID={this.props.match.params.forumID}/>
          })
        }
    }
        
    modify_roles = () => {
        if (this.state.activeComponent !== <ModifyRoles/>) { 
            this.setState({
                activeComponent: <ModifyRoles forumID={this.props.match.params.forumID}/>
            })
        }
    }

    view_roles = () => {
        if (this.state.activeComponent !== <ViewRoles/>) { 
            this.setState({
                activeComponent: <ViewRoles forumID={this.props.match.params.forumID}/>
            })
        }
    }

    componentDidMount = async () => {
    }

    render() {
        return (
            <Container className="pt-4">
                    <h1>
                        Editing permissions for {this.props.match.params.type}: {this.props.match.params.name}
                    </h1>
                    <Button className="mr-3" variant="primary" onClick={this.view_roles}>View Roles</Button>
                    <Button className="mr-3" variant="primary" onClick={this.assign_roles}>Assign Roles</Button>
                    <Button variant="primary" onClick={this.modify_roles}>Modify Roles</Button>

                    <Container className="mt-4">
                        {this.state.activeComponent}
                    </Container>


            </Container>
        );
    }
} 
export default EditPerms
