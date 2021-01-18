import React from "react";
import {Button} from "react-bootstrap";

class BackButton extends React.Component {
    static contextTypes = {
        router: () => true, // replace with PropTypes.object if you use them
    }

    render() {
        return (
            <Button
                className="button icon-left"
                onClick={this.context.router.history.goBack}>
                Back
            </Button>
        )
    }
}

export default BackButton