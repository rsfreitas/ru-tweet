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

![Login screen](images/login.png?raw=true)

After the login the user is presented with a screen providing access to the
main features. They are available through tabs as shown in the nex screenshot.

![Main screen](images/main_screen.png?raw=true)

This screen also lets the user read its own tweets in the first tab (Personal).
These tweets can be created by clicking the message icon locate at the bottom
right corner of the application.

![New tweet button](images/new_tweet_button.png?raw=true)

When clicked this options will open a dialog letting the user enter a new tweet
message.

![New tweet message](images/new_tweet_dialog.png?raw=true)

The application main window also gives access to a menu, locate at the top
right corner (the three dots). And it shows options giving access to the
follow and block features.

![Menu options](images/menu_options.png?raw=true)

The Follow option will open a dialog to let the user enter the username of
the user he wants to follow.

![Follow dialog](images/follow_dialog.png?raw=true)

The Block option will open a dialog to le the user enter the username of the
user he wants to block.

![Block dialog](images/block_dialog.png?raw=true)

The second tabs presents tweets from all followed users sorted by the messages
date/time. Each message will be shown with its date/time, author name and
its number of likes.

![Timeline](images/timeline.png?raw=true)

When a message is clicked a menu is opened displaying options that a user
can execute on it.

![Timeline message options](images/timeline_options.png?raw=true)

*Attention*: Both _pingback_ and _retweet_ features aren't available yet.

The third tab presents a list of followed users where each row, when clicked,
will give access to an options menu with options to send/receive direct
messages to an user and to block him.

![Following options](images/following_options.png?raw=true)

The Direct Messages (DM) menu option, when activated, will open a new
windows where the user can chat with the selected contact. This new
window look preetty much like a chat one.

![Direct menssages window](images/dm.png?raw=true)

The last tab presents a list of blocked users where each row, when clicked,
will give access to another options menu with an option to unblock the
selected user.

![Block options](images/blocked_options.png?raw=true)

