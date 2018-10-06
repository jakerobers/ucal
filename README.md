# Custom Date Generator

Custom Date Generator calculates a set of dates provided a .cdg file, begin window and end window.

## Usage

A cdg file will have the following format:

```
[2018-01-01 00:00 a] New Years 	# an anually recurring event labeled as "New Years"
[2018-11-22 00:00 a 4thu]: Thanksgiving 	# an anually recurring event for the 4th Thursday of November
[2018-11-22 00:00 m 1wed,3fri]: Rust Meetup 	# a monthly recurring event for the 1st Wednesday and 3rd Friday of the month
[2018-10-05 21:00 w mon,tue,wed,thu,fri]: Pack lunch for work 	# Recur on the weekdays
```

Use cdg and provide it a "pane" to view your events:

```
cdg my_schedule.cdg --begin="2018-11-01" --end="2018-12-01"
```

Will generate:

```
```

