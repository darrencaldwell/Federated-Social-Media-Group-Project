import React from 'react'
import {Container, Image} from "react-bootstrap";
import dp from '../../images/display-pic.png'
import '../../styling/display-pic.css'

class DisplayPicture extends React.Component {
    // constructor() {
    //     super();
    //     this.state = {
    //         uploadedPicture: false
    //     }
    // }

    /**
     * Doesn't do anything as of yet
     * Nee
     * @param e
     */
    handleImageUpload = e => {
        const [file] = e.target.files

        if (file) {
            console.log(file)
        }
    }

    render() {
        if (this.props.uploadedPicture) {
            return (
                <Container className="displayPic">
                    <p1>Picture Yay</p1>

                </Container>
            )
        } else {
            return (
                <Container className="displayPic">
                    <Image src={dp}/>
                    <Container>
                        <input className="uploadPicButton"
                               type="file"
                               accept="image/*" onChange={this.handleImageUpload}
                        />
                    </Container>
                </Container>
            )
        }
    }
}

export default DisplayPicture