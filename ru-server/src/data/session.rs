
use std::collections::HashMap;
use uuid::Uuid;

use crate::data::user::User;

pub struct Session {
    active_users: HashMap<String, User>
}

impl Session {
    pub fn create() -> Session {
        Session{
            active_users: HashMap::new()
        }
    }

    pub fn add(&mut self, username: &str) -> Option<String> {
        let u = User::new(username);
        let uuid = Uuid::new_v4().to_hyphenated().to_string();
        let rc = uuid.clone();

        self.active_users.insert(uuid, u);
        Some(rc)
    }

    pub fn delete(&mut self, id: &str) -> bool {
        match self.active_users.remove(id) {
            Some(_) => true,
            None => false
        }
    }

    pub fn get_id(&self, username: &str) -> Option<&str> {
        for (id, user) in self.active_users.iter() {
            if id.eq(username) {
                return Some(&user.name)
            }
        }

        None
    }

    pub fn is_logged(&self, username: &str) -> bool {
        for user in self.active_users.values() {
            if user.name.eq(username) {
                return true
            }
        }

        false
    }

    pub fn is_logged_with_id(&self, id: &str) -> bool {
        self.active_users.contains_key(id)
    }
}

