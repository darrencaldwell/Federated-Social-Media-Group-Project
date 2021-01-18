import React from "react";
import {Button, Container} from "react-bootstrap";

class CreateComment extends React.Component {
    render() {
        return (
            <Container className="mt-3">
                <Button variant="light">Create Comment</Button>
            </Container>
        )
    }
}

export default CreateComment