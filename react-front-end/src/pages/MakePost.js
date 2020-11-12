import React from 'react';

class Title extends React.Component {
  constructor(props) {
    super(props);
    this.handleChange = this.handleChange.bind(this);
  }

  handleChange(e) {
    this.props.onChange(e.target.value);  }

  handleBlur() {
    if (this.props.value === '') {
      this.props.onChange(this.props.default);
    }
  }

  render() {
    const value = this.props.value;    
    return (
      <div>
        <input          
          value={value}          
          onChange={this.handleChange} 
          onFocus={() => this.props.onChange('')}
          onBlur={() => this.handleBlur()}/>    
          </div>
    );
  }
}

class Text extends React.Component {
  constructor(props) {
    super(props);
    this.handleChange = this.handleChange.bind(this);
  }

  handleChange(e) {
    this.props.onChange(e.target.value);  }

    handleBlur() {
      if (this.props.value === '') {
        this.props.onChange(this.props.default);
      }
    }

  render() {
  return (
    <div>
    <textarea 
      className="text" value={this.props.value} onChange={this.handleChange}
      onFocus={() => this.props.onChange('')}
      onBlur={() => this.handleBlur()}>
    </textarea>
    </div>
  );
  }
}

function SendButton(props) {
  return (
    <button 
      className="sendButton" 
      onClick={(props.onClick)} >
      {props.value}
    </button>
  );
}

class Page extends React.Component {
constructor(props) {
  super(props);
  this.changeTitle = this.changeTitle.bind(this);
  this.changeBody = this.changeBody.bind(this);
  this.state = {
    buttonText: 'Create Post',
    titleText: 'Title',
    defaultTitle: 'Title',
    bodyText: 'Put the body of your post here',
    defaultBody: 'Put the body of your post here',
  };
}

handleClick() {
  // if no text has been entered, it will return to default before the button is pressed
  if(this.state.titleText === this.state.defaultTitle || 
      this.state.bodyText === this.state.defaultBody) {
    alert('Please enter a title and body');
  } else {
      fetch("api/subforums/1/posts", {
          method: "POST",
          withCredentials: true,
          credentials: 'include',
          headers: {
              'Authorization': localStorage.getItem('token'),
              'Content-Type': 'application/json'
          },
          body: JSON.stringify({
      "postTitle":this.state.titleText,
      "postMarkup":this.state.bodyText,
      "userId":localStorage.getItem('userId') //somehow find out the user id
    })
      }).then(responseJson => {
          console.log(responseJson);
      }).catch(error => this.setState({
          message: "Error postintg post: " + error
      }));
  }
}

changeTitle(v) {
  this.setState({titleText: v})
}

changeBody(v) {
  this.setState({bodyText: v})
}

renderTitle() {
  return (
    <Title 
      value = {this.state.titleText}
      default = {this.state.defaultTitle}
      onChange={this.changeTitle}
    />
  );
}

renderText() {
  return (
    <Text 
      value = {this.state.bodyText}
      default = {this.state.defaultBody}
      onChange={this.changeBody}
    />
  );
}

renderButton() {
  return (
      <SendButton
        value = {this.state.buttonText}
        onClick={() => this.handleClick()}
      />
  );
}

render() {
  return (
    <div>
      <div className="title">
        {this.renderTitle()}
      </div>
      <div className="text">
        {this.renderText()}
      </div>
      <div className="sendButton">
        {this.renderButton()}
      </div>
    </div>
  );
}
}

export default Page
