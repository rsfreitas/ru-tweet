
use std::collections::HashMap;
use uuid::Uuid;

use crate::data::user::User;

// A container to hold information about connected/active users, keeping a
// different ID value for each one of them.
pub struct Session {
    active_users: HashMap<String, User>,
}

impl Session {
    // Creates a Session container.
    pub fn create() -> Session {
        Session{
            active_users: HashMap::new(),
        }
    }

    // Adds an user inside the current Session container.
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

    pub fn get_id(&self, username: &str) -> Option<String> {
        for (id, user) in self.active_users.iter() {
            if user.name.eq(username) {
                return Some(id.to_string())
            }
        }

        None
    }

    pub fn get_username(&self, id: &str) -> Option<String> {
        match self.active_users.get(id) {
            None => None,
            Some(u) => Some(u.name.to_string())
        }
    }

    pub fn get_token(&self, id: &str) -> Option<String> {
        match self.active_users.get(id) {
            None => None,
            Some(u) => Some(u.token.to_string())
        }
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

    pub fn is_id_from_user(&self, id: &str, username: &str) -> bool {
        match self.active_users.get(id) {
            None => false,
            Some(u) => u.name.eq(username)
        }
    }

    pub fn set_user_token(&mut self, id: &str, username: &str, token: &str) {
        /*
         * Replaces current User with its token so we can send notifications
         * when required to.
         */
        self.active_users.insert(id.to_string(), User::new_with_token(&username, token));
    }
}

