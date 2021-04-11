import React from "react";
import Fab from '@material-ui/core/Fab';
import {Link} from 'react-router-dom';
import ArrowBackIcon from '@material-ui/icons/ArrowBack';
import '../styling/container-pages.css'

// props: url
class BackButton extends React.Component {
    // static contextTypes = {
    //     router: () => true, // replace with PropTypes.object if you use them
    // }

    render() {
        return (
            <div className="back">
                <Fab as={Link} href={this.props.url} className="icon-left">
                    {/*onClick={this.context.router.history.goBack}*/}
                        <ArrowBackIcon />
                </Fab>
            </div>
        )
    }
}

export default BackButton
