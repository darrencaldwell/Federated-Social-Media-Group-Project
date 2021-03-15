import React from 'react'
import Avatar from 'react-avatar';
import Uploady from "@rpldy/uploady";
import UploadButton from "@rpldy/upload-button";
import {Container} from "react-bootstrap";
import '../../styling/display-pic.css'

class DisplayPicture extends React.Component {
    render() {
        return (
            <Container className="displayPic">
                <Avatar round={true} src={"/api/users/" + localStorage.getItem("userId") + "/profilepicture"} name={localStorage.getItem("username")}/>
                <Container>
                    <Uploady
                    destination={{ 
                        url: "/local/users/" + localStorage.getItem("userId") + "/profilepicture",
                        headers: {"Authorization": localStorage.getItem("token")}
                    }}>
                        <UploadButton/>
                    </Uploady>
                </Container>
            </Container>
        )
    }
}

export default DisplayPicture
