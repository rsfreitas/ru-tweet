
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// unfollow handler.
//
// Must receive:
// {
//  "from": string,
//  "follow": string
// }
//
// It will always return a 200 code with an internal code of what really
// happened. It has the following format:
//
// {
//  "code": int
// }
//
#[post("/", format = "application/json", data = "<message>")]
pub fn handler(message: Json<Message>, session: State<RwLock<Session>>, db: State<Database>) -> Json<Answer> {
    let mut code = 0;

    if message.from.is_empty() || message.follow.is_empty() {
        code = 1; // invalid fields
    } else if !session.read().unwrap().is_logged_with_id(&message.from) {
        code = 2; // the user is not logged at the moment
    } else if !db.user_exists(&message.follow) {
        code = 3; // the user to be unfollowed does not exist
    } else {
        let name = session.read().unwrap().get_username(&message.from).unwrap();

        if !db.unfollow_user(&name, &message.follow) {
            code = 4; // database error
        }
    }

    Json(Answer::new(code))
}

