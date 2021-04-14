import React, {Component} from 'react';
import {Button, ButtonGroup} from "react-bootstrap";
import up_hollow from './../images/up_hollow.png'
import up_solid from './../images/up_solid.png'
import down_hollow from './../images/down_hollow.png'
import down_solid from './../images/down_solid.png'
import "./../styling/voting.css"


// props: upvote, downvote, _userVotes, type ('posts' or 'comments'), postID, impID
export class Voting extends Component {
    constructor(props){
        super(props)
        if (this.props._userVotes) {
          this.state = {
            count: 0 + this.props.upvotes - this.props.downvotes,
        }
          // determine start state of the current users votes
          let userUrl = "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users/" + localStorage.getItem("userId")
          let is_upvote = null
          this.props._userVotes.forEach( (list) => {
            if (list.user === userUrl) {
              is_upvote = list.isUpvote
            }
          })
          if (is_upvote === true) {
            this.state.is_downvote = false
            this.state.is_upvote = true
          } else if (is_upvote === null) {
            this.state.is_downvote = false
            this.state.is_upvote = false
          } else {
            this.state.is_downvote = true
            this.state.is_upvote = false
          }
        }
    }

    send_vote = () => {
      let destUrl = "/api/" + this.props.type + "/" + this.props.postID + "/vote"
      let is_upvote = null

      if (this.state.is_upvote === true) {is_upvote = true}
      else if (this.state.is_downvote === true) {is_upvote = false}
      else {is_upvote = null}

      fetch(destUrl, {
        method: "PUT",
        withCredentials: true,
        credentials: 'include',
        headers: {
            'Authorization': "Bearer " + localStorage.getItem('token'), // need to get the auth token from localStorage
            'Content-Type': 'application/json',
            'redirect': this.props.impID
        },
        body: JSON.stringify({
            "isUpvote": is_upvote
        })
    }).catch(error => this.setState({
        message: "Error posting post: " + error
    }));
  }

    getUpvoteImg = () => this.state.is_upvote ? up_solid : up_hollow
    getDownvoteImg = () => this.state.is_downvote ? down_solid : down_hollow
    
    getUpstyle = () => this.state.is_upvote ? "solid" : "hollow"
    getDownstyle = () => this.state.is_downvote ? "solid" : "hollow"

    upvote = () => {
      if (this.state.is_downvote) { // then undo users downvote
        this.setState({
          count: this.state.count + 2,
          is_upvote: true,
          is_downvote: false,
        }, () => {this.send_vote()})
      }
      else if (this.state.is_upvote) { // then undo upvote
        this.setState({
          count: this.state.count - 1,
          is_upvote: false,
          is_downvote: false,
        }, () => {this.send_vote()})
      }
      else { // just upvote
        this.setState({
          count: this.state.count + 1,
          is_upvote: true,
          is_downvote: false,
        }, () => {this.send_vote()})
      }
    }

    downvote = () => {
      if (this.state.is_upvote) { // then undo users upvote
        this.setState({
          count: this.state.count - 2,
          is_upvote: false,
          is_downvote: true,
        }, () => {this.send_vote()})
      }
      else if (this.state.is_downvote) { // then undo downvote
        this.setState({
          count: this.state.count + 1,
          is_upvote: false,
          is_downvote: false,
        }, () => {this.send_vote()})
      }
      else { // just downvote
        this.setState({
          count: this.state.count - 1,
          is_downvote: true,
          is_upvote: false,
        }, () => {this.send_vote()})
      } 
    }

render() {
    if (!this.props._userVotes) {
      return null
    }
    
    const upStyle = this.getUpstyle();
    const downStyle = this.getDownstyle();
    
    return (
      <ButtonGroup vertical className="voting-container">
        <Button bsPrefix={"vote_t " + upStyle} variant='clear' onClick={this.upvote}>🠭</Button>
        <div className="middle">{this.state.count}</div>
        <Button bsPrefix={"vote_b " + downStyle} variant='clear' onClick={this.downvote}>🠯</Button>
      </ButtonGroup>


      // <div className="voting-container">
      //   {<button className="vote_t" onClick={this.upvote}>
      //     <img src={upImage} alt="up arrow" width="20" height="30"></img>
      //   </button>}
      //   <div className="middle">{this.state.count}</div>
      //   {<button className="vote_b" onClick={this.downvote}>
      //     <img src={downImage} alt="down arrow" width="20" height="30"></img>
      //   </button>}
      // </div>
    );
  }
}
export default Voting;