
import React, {Component} from 'react';
import '../styling/container-pages.css';

// returns the time between a given time and now
// props: createdTime, modifiedTime (to determine if we need to show modified time)
class TimeSince extends Component {

    render() {

        function getTimeSince(diff) {
            let second = 1000 // from ms
            let minute = second*60
            let hour = minute*60
            let day = hour*24
            let week = day*7
            let tmp

            if (diff >= week) {
                tmp = Math.round(diff/week)
                date_string = tmp + (tmp > 1 ? " weeks ago" : " week ago")
            }
            else if (diff >= day) {
                tmp = Math.round(diff/day)
                date_string = tmp + (tmp > 1 ? " days ago" : " day ago")
            }
            else if (diff >= hour) {
                tmp = Math.round(diff/hour)
                date_string = tmp + (tmp > 1 ? " hours ago" : " hour ago")
            }
            else if (diff >= minute) {
                tmp = Math.round(diff/minute)
                date_string = tmp + (tmp > 1 ? " minutes ago" : " minute ago")
            }
            else if (diff >= second) {
                tmp = Math.round(diff/second)
                date_string = tmp + (tmp > 1 ? " seconds ago" : " second ago")
            }
            else if (diff > 0) {
                date_string = 'now'
            }
            else {
                date_string = null
            }
            return date_string
        }

        let date_string

        if (this.props.createdTime && this.props.modifiedTime) {
            let date_created = this.props.createdTime
            let date_modified = this.props.modifiedTime

            if (date_created - date_modified === 0) {
                date_string = null
            }
            else {
                let now = new Date().getTime()
                let diff = Math.abs(now - date_modified)
                date_string = getTimeSince(diff)
                date_string = date_string ? 'edited ' + date_string : null
            }

        }
        else {
            let date_created = this.props.createdTime
            let now = new Date().getTime()
            let diff = Math.abs(now - date_created)
            date_string = getTimeSince(diff)
        }
        return (
            <div>
                {date_string}
            </div>
        )
    }
}
export default TimeSince