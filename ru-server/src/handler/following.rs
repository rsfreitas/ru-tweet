
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// follow handler.
//
// Must receive:
// {
//  "from": string,
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
    let mut following = vec![];

    // data is right?
    // user is logged?
    // get following

    if message.from.is_empty() || message.follow.is_empty() {
        code = 1;
    } else if !session.read().unwrap().is_logged_with_id(&message.from) {
        code = 2;
    } else if !db.user_exists(&message.follow) {
        code = 3;
    } else {
        let name = session.read().unwrap().get_username(&message.from).unwrap();
        following = match db.get_following(&name) {
            None => vec![],
            Some(f) => f
        };
    }

    let answer = match code {
        0 => Answer::new_with_following(code, following),
        c @ _ => Answer::new(c)
    };

    Json(answer)
}

