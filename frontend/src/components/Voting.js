import React, {Component} from 'react';

// props: count, is_downvote, is_upvote
export class Voting extends Component {
    constructor(props){
        super(props)
        this.state = {
            count: 0,
            is_downvote: false,
            is_upvote: false,
        }
    }

    upvote = () => {
      if (this.state.is_downvote) { // then undo users downvote
        this.setState({
          count: this.state.count + 2,
          is_upvote: true,
          is_downvote: false,
        })
      }
      else if (this.state.is_upvote) { // then undo upvote
        this.setState({
          count: this.state.count - 1,
          is_upvote: false,
          is_downvote: false,
        })
      }
      else { // just upvote
        this.setState({
          count: this.state.count + 1,
          is_upvote: true,
        })
      }
    }

    downvote = () => {
      if (this.state.is_upvote) { // then undo users upvote
        this.setState({
          count: this.state.count - 2,
          is_upvote: false,
          is_downvote: true,
        })
      }
      else if (this.state.is_downvote) { // then undo downvote
        this.setState({
          count: this.state.count + 1,
          is_upvote: false,
          is_downvote: false,
        })
      }
      else { // just downvote
        this.setState({
          count: this.state.count - 1,
          is_downvote: true,
        })
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