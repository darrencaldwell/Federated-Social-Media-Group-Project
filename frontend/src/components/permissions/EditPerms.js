import React from "react";
import {Button, Container} from "react-bootstrap";
import AssignRoles from './AssignRoles';
import ModifyRoles from './ModifyRoles';


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
                activeComponent: <AssignRoles/>
          })
        }
    }
        
    modify_roles = () => {
        if (this.state.activeComponent !== <ModifyRoles/>) { 
            this.setState({
                activeComponent: <ModifyRoles/>
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
