# ru-client

A client Android application to test ru-server features.

## Building

In order to build the client the Android Studio can be used or the gradle build
tool, if the command line is being used.

### Notifications

Just like the *ru-server* a [Firebase Cloud Messaging - FCM](https://firebase.google.com)
account must be used so that an application can be added inside a project in
order to create the _google-services.json_ file, which enables the notification
feature inside the application.

This file must be put inside the _app_ directory when building the application.

## Features

When executed for the first time or in case the server had to be restarted the
user will be prompted with a login screen, asking for an username and a password.

This screen will also let a new user to be created if desired to.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/login.png" width="320" height="550" />
</p>

After the login the user is presented with a screen providing access to the
main features. They are available through tabs as shown in the nex screenshot.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/main_screen.png" width="320" height="550" />
</p>

This screen also lets the user read its own tweets in the first tab (Personal).
These tweets can be created by clicking the message icon locate at the bottom
right corner of the application.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/new_tweet_button.png" width="128" height="550" />
</p>

When clicked this options will open a dialog letting the user enter a new tweet
message.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/new_tweet_dialog.png" width="320" height="550" />
</p>

The application main window also gives access to a menu, locate at the top
right corner (the three dots). And it shows options giving access to the
follow and block features.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/menu_options.png" width="320" height="550" />
</p>

The Follow option will open a dialog to let the user enter the username of
the user he wants to follow.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/follow_dialog.png" width="320" height="550" />
</p>

The Block option will open a dialog to le the user enter the username of the
user he wants to block.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/block_dialog.png" width="320" height="550" />
</p>

The second tabs presents tweets from all followed users sorted by the messages
date/time. Each message will be shown with its date/time, author name and
its number of likes.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/timeline.png" width="320" height="550" />
</p>

When a message is clicked a menu is opened displaying options that a user
can execute on it.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/timeline_options.png" width="320" height="550" />
</p>

**Attention**: Both _pingback_ and _retweet_ features aren't available yet.

The third tab presents a list of followed users where each row, when clicked,
will give access to an options menu with options to send/receive direct
messages to an user and to block him.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/following_options.png" width="320" height="550" />
</p>

The Direct Messages (DM) menu option, when activated, will open a new
windows where the user can chat with the selected contact. This new
window look preetty much like a chat one.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/dm.png" width="320" height="550" />
</p>

The last tab presents a list of blocked users where each row, when clicked,
will give access to another options menu with an option to unblock the
selected user.

<p align="center" >
<img src="https://raw.githubusercontent.com/rsfreitas/ru-tweet/master/ru-client/images/blocked_options.png" width="320" height="550" />
</p>
