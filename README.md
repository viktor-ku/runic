# runic

> Stupid string to duration converter

## General usage

Two mods are available: using [timeout](#timeout) _and_/_or_ using [exact time](#exact-time). 

Order of things does not matter, feel free to mix everything however you want.

Quick example for the _timeout_:

```bash
"pause in 3 min"
```

Quick example for the _exact time_:

```bash
"I should leave at 6pm"
```

In the latter, it will calculate duration between your local "now" and 
specified _exact time_.

#### To Message, Or not to Message
 
The following timers are exactly the same, all three of them will return
the same duration back, the only difference is that the last one has some text
for the user.

```bash
"4 minutes"
"4m" # You have different methods to write the length (See below for more)
"remind me that I have a tea in the kitchen in 4 min"
```

### Learn by Example

#### The simplest timer you can launch

```bash
"my first timer ever for 30 seconds"
```

_This will simply return 30 seconds!_

#### You can combine multiple durations

These durations might be of any length as long as it makes sense to you!

```bash
"1 minute 30s .5m 3600 seconds"
```

_This will return an hour and 2 minutes!_

#### Exact times can be used

Duration between current time and specified value will be
calculated and added:

_consider it is 8 am now (08:00)_

```bash
"remind me to join the meeting call at 9:15"
```

_This will return an hour and 15 minutes!_

#### You can use `-` in front of the duration 

It will tell the timer that you want to move the target one hour towards your current time:

```bash
"2h -1h -30m"
```

_This will return 30 minutes!_

##### It works great when using with exact times

_consider it is 4pm now (16:00)_

```bash
"originally event starts at 6pm was moved 1 hour I need -2 hours to get there"
```

_This will return an hour!_

## Timeout

Timer is a total of all specified timeouts combined. You can specify
timeouts using keywords for _hours_, _minutes_ and _seconds_.

_Note_ that the space between the timeout and keyword is optional.

### Hours

- `hours`
- `hour`
- `hrs`
- `hr`
- `h`

E.g.

```bash
"10h"
"10 hr"
"10 hours"
```

### Minutes

- `minutes`
- `minute`
- `mins`
- `min`
- `m`

E.g.

```bash
"10m"
"10 min"
"10 minutes"
```

### Seconds

- `seconds`
- `second`
- `secs`
- `sec`
- `s`

E.g.

```bash
"10s"
"10 sec"
"10 seconds"
```

### Floating numbers

You can use `.` in your timeouts. For example, when you need to quickly specify
2 hours 30 minutes, but you don't want to write it down, you can do this:

```bash
"the long and boring way 2 hours 30 minutes"
"slightly shorter but still boring way 2h 30m"
"or in short 2.5h"
```

Or, possibly the shortest way to describe 30 minutes is:

```bash
".5h and it will be done"
```

## Exact Time

The word _`at`_ is a keyword. It marks the beginning of the _exact time_
expression and is used to specify some time of interest explicitly. 
Under the hood it calculates the duration between the local _now_ 
and the target time:

```bash
"should finish at 19:30"
"should finish at 7:30pm"
```

Just like with timeouts, the space between the time and "am"/"pm"
is optional.

```bash
"at 1am"
"at 1 am"
```

Minutes part is optional and is going to be set to `0` by default, so
the following timers are equal:

```bash
"at 22"
"at 22:00"
"at 10pm"
"at 10:00 pm"
```

### Using other timezones to compare

The word _`utc`_ (case insensitive) is a keyword. It's used at 
the end of _exact time_ expressions to point that the _time I 
am targeting is actually in a different timezone than me_.

Giving the following example - _10 am UTC_, programm will 
figure out whatever it means for your current local time and 
calcualte an appropriate timeout for the action.

```bash
"at 10am UTC"
```

You have four variants how to specify `utc` keyword:

#### Just utc

Using just `utc` is an alias for `utc+0` (or `utc-0` for that matter).

The following examples are all the same:

```bash
"at 10am utc"
"at 10am utc+0"
"at 10am utc-0"
```

#### Short variant

Short variant is when you only want to specify a target hour.

```bash
"at 10am utc+1"
"at 10am utc-3"
"at 10am utc+10"
"at 10am utc-11"
```

#### Long variants

Specify an hour, following by the _optional_ colon, following by the minutes.

Note that you have to use _two_ digits for the hours and _two_
digits for the minutes always in the long variants.

Examples with the colon:

```bash
"at 10am utc+01:00"
"at 10am utc+11:30"
"at 10am utc-03:30"
"at 10am utc-00:45"
```

Examples without the colon (just 4 digits):

```bash
"at 10am utc+0100"
"at 10am utc+1130"
"at 10am utc-0330"
"at 10am utc-0045"
```

### Where a day starts and ends?

In case specified time is in the past relative to the current 24h day, 
it carries out to the next day, for example:

_Consider it is October 1st, 11:30pm (23:30)_

```bash
"definitely go to sleep at 2am"
```

There already was point in time when _2am_ of October 1st occurred, so
it carries out to the next day, October the 2nd, effectively setting your
timer for 2 hours 30 minutes.

The same happens with more distant points in time, like this:

_consider it is October 1st, 4:15pm (16:15)_

```bash
"at 4:15am"
```
