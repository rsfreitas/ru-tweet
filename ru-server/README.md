# ru-server

A rust twitter-like server.

## Dependencies

* rocket
* rocket_contrib
* serde
* serde_json
* serde_derive
* mongodb
* uuid
* chrono
* reqwest

## Features

* (basic) User authentication
* Add/Delete/List/Like tweets
* Follow/Unfollow users
* Block/Unblock users
* Notifications

## Building

Since we're using rocket as our REST framework the nightly rust compiler
must be used.

With the nightly rust compiler installed run the command to build the
application:
```
cargo build
```

## Running the application

Before executing the server a mongodb server must be executed and available
for it. So the first step is to put a mongodb database server in execution.

The code block inside the file _main.rs_ defines its host and port:
```rust
    let db = Database::create("localhost", 27017).unwrap();
```

## Using notifications

The server uses the [Firebase Cloud Messaging - FCM](https://firebase.google.com)
to send notifications when required to.

In order to use this feature you must have a firebase account with the cloud
messaging enabled. This will provide an *Authorization* key which must be
updated in the source code, inside the file _notification/mod.rs_
```rust
const KEY: &'static str = "put-yout-FCM-key-here";
```
replacing the constant value with it.

Without a valid key all notification request will receive a 403 from the
server.

