import React, {Component} from 'react';

// props: upvote, downvote, _userVotes, type ('posts' or 'comments'), postID, impID
export class Voting extends Component {
    constructor(props){
        super(props)
        this.state = {
          count: 0 + this.props.upvotes - this.props.downvotes,
      }
        // determine start state of the current users votes
        let userUrl = "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users/" + localStorage.getItem("userId")
        let is_upvote = null
        this.props._userVotes.postsVotes.forEach( (list) => {
          if (list.user === userUrl) {
            is_upvote = list.isUpvote
          }
        })
        if (is_upvote) {
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

    send_vote = () => {
      let destUrl = "/api/" + this.props.type + "/" + this.props.postID + "/vote"
      let is_upvote = null
      console.log(this.state.is_upvote)
      console.log(this.state.is_downvote)

      if (this.state.is_upvote == true) {console.log("AA"); is_upvote = true}
      else if (this.state.is_downvote == true) {console.log("BB"); is_upvote = false}
      else {is_upvote = null}
      console.log(is_upvote)

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
    }).then(responseJson => { 
        console.log(responseJson);
    }).catch(error => this.setState({
        message: "Error posting post: " + error
    }));
  }

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
        }, () => {this.send_vote()})
      } 
    }

render() {
    return (
      <div>
        {<button onClick={this.upvote}>
          +
        </button>}
        <span>{this.state.count}</span>
        {<button onClick={this.downvote}>
          -
        </button>}
      </div>
    );
  }
}
export default Voting;