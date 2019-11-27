
use chrono::Utc;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::oid::ObjectId;
use mongodb::Bson;

use crate::data::tweet::{Tweet, Kind};
use crate::data::dm::DM;

pub struct Database {
    client: Client,
}

impl Database {
    pub fn create(host: &str, port: u16) -> Option<Database> {
        let db = match Client::connect(host, port) {
            Err(_) => return None,
            Ok(db) => db
        };

        Some(Database{
            client: db,
        })
    }

    pub fn add_user(&self, username: &str, password: &str) {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": username,
            "password": password,
        };

        coll.insert_one(doc.clone(), None).ok();
    }

    pub fn delete_user(&self, username: &str) -> bool {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": username,
        };

        match coll.delete_one(doc.clone(), None) {
            Err(_) => false,
            Ok(r) => match r.deleted_count {
                0 => false,
                _ => true
            }
        }
    }

    pub fn user_exists(&self, username: &str) -> bool {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": username,
        };

        match coll.find_one(Some(doc.clone()), None) {
            Err(_) => false,
            Ok(d) => match d {
                None => false,
                Some(_) => true
            }
        }
    }

    pub fn check_user_and_password(&self, username: &str, password: &str) -> bool {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": username,
            "password": password,
        };

        match coll.find_one(Some(doc.clone()), None) {
            Err(_) => false,
            Ok(d) => match d {
                None => false,
                Some(_) => true
            }
        }
    }

    pub fn add_tweet(&self, from: &str, content: &str, kind: Kind) -> Option<String> {
        let coll = self.client.db("rutweet").collection("tweet");
        let doc = doc!{
            "from": from,
            "content": content,
            "kind": kind.to_bson(),
            "timestamp": Utc::now(),
        };

        let res = match coll.insert_one(doc.clone(), None) {
            Err(_) => Bson::Null,
            Ok(r) => r.inserted_id.unwrap()
        };

        match res {
            Bson::ObjectId(oid) => Some(oid.to_hex()),
            _ => None
        }
    }

    pub fn delete_tweet(&self, from: &str, id: &str) -> bool {
        let coll = self.client.db("rutweet").collection("tweet");

        match ObjectId::with_string(id) {
            Err(_) => false,
            Ok(oid) => {
                let doc = doc!{
                    "from": from,
                    "_id": oid,
                };

                match coll.delete_one(doc.clone(), None) {
                    Err(_) => false,
                    Ok(r) => match r.deleted_count {
                        0 => false,
                        _ => true
                    }
                }
            }
        }
    }

    fn to_string(b: &Bson) -> Option<String> {
        match b {
            Bson::String(s) => Some(s.to_string()),
            Bson::ObjectId(oid) => Some(oid.to_hex()),
            _ => None
        }
    }

    pub fn get_tweet(&self, from: &str, id: &str) -> Option<Tweet> {
        let coll = self.client.db("rutweet").collection("tweet");

        if let Ok(id) = ObjectId::with_string(id) {
            let doc = doc!{
                "from": from,
                "_id": id,
            };

            if let Ok(d) = coll.find_one(Some(doc.clone()), None) {
                if let Some(item) = d {
                    let like = match item.get("like") {
                        None => 0,
                        Some(i) => i.as_i32().unwrap()
                    };

                    let kind = match item.get("kind") {
                        None => Kind::Simple,
                        Some(b) => match Kind::from_bson(b) {
                            None => Kind::Simple,
                            Some(k) => k
                        }
                    };

                    let timestamp = match item.get("timestamp") {
                        None => Utc::now().to_rfc3339(),
                        Some(d) => d.as_utc_date_time().unwrap().to_rfc3339()
                    };

                    return Some(Tweet::new(&Database::to_string(item.get("from").unwrap()).unwrap(),
                                           &Database::to_string(item.get("content").unwrap()).unwrap(),
                                           &Database::to_string(item.get("_id").unwrap()).unwrap(),
                                           like,
                                           kind,
                                           &timestamp))
                }
            }
        }

        None
    }

    pub fn list_tweet(&self, from: &str) -> Vec<Tweet> {
        let coll = self.client.db("rutweet").collection("tweet");
        let doc = doc!{
            "from": from,
        };

        let mut tweets = vec![];
        let cursor = coll.find(Some(doc.clone()), None).unwrap();

        for result in cursor {
            if let Ok(item) = result {
                let like = match item.get("like") {
                    None => 0,
                    Some(i) => i.as_i32().unwrap()
                };

                let kind = match item.get("kind") {
                    None => Kind::Simple,
                    Some(b) => match Kind::from_bson(b) {
                        None => Kind::Simple,
                        Some(k) => k
                    }
                };

                let timestamp = match item.get("timestamp") {
                    None => Utc::now().to_rfc3339(),
                    Some(d) => d.as_utc_date_time().unwrap().to_rfc3339()
                };

                tweets.push(Tweet::new(&Database::to_string(item.get("from").unwrap()).unwrap(),
                                       &Database::to_string(item.get("content").unwrap()).unwrap(),
                                       &Database::to_string(item.get("_id").unwrap()).unwrap(),
                                       like,
                                       kind,
                                       &timestamp))
            }
        }

        tweets
    }

    fn is_in_array(&self, key: &str, name: &str, array: &str) -> bool {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": key,
        };

        if let Ok(d) = coll.find_one(Some(doc.clone()), None) {
            if let Some(item) = d {
                let b = match item.get(array) {
                    None => vec![],
                    Some(dbv) => dbv.as_array().unwrap().to_vec()
                };

                return b.contains(&Bson::from(name))
            }
        }

        false
    }

    fn add_into_string_array(&self, username: &str, name: &str, array: &str) -> bool {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": username,
        };

        if let Ok(d) = coll.find_one(Some(doc.clone()), None) {
            if let Some(item) = d {
                let mut a = match item.get(array) {
                    None => vec![],
                    Some(dbv) => dbv.as_array().unwrap().to_vec()
                };

                if !a.contains(&Bson::from(name)) {
                    a.push(Bson::from(name));
                }

                if let Ok(_) = coll.update_one(doc,
                                               doc!{"$set": {
                                                   array: a.into_iter()
                                                           .map(Bson::from)
                                                           .collect::<Vec<_>>()
                                               }}, None)
                {
                    return true
                }
            }
        }

        false
    }

    fn delete_from_string_array(&self, username: &str, name: &str, array: &str) -> bool {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": username,
        };

        if let Ok(d) = coll.find_one(Some(doc.clone()), None) {
            if let Some(item) = d {
                let mut f = match item.get(array) {
                    None => vec![],
                    Some(dbv) => dbv.as_array().unwrap().to_vec()
                };

                if f.contains(&Bson::from(name)) {
                    f.retain(|b| !b.as_str().unwrap().eq(name));
                }

                if let Ok(_) = coll.update_one(doc,
                                               doc!{"$set": {
                                                   array: f.into_iter()
                                                           .map(Bson::from)
                                                           .collect::<Vec<_>>()
                                               }}, None)
                {
                    return true
                }
            }
        }

        false
    }

    fn get_string_array(&self, key: &str, array: &str) -> Option<Vec<String>> {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": key,
        };

        if let Ok(d) = coll.find_one(Some(doc.clone()), None) {
            if let Some(item) = d {
                if let Some(dbv) = item.get(array) {
                    let v = dbv.as_array().unwrap().to_vec();
                    let f = v.iter().fold(vec![],
                                          |mut acc, e| {
                                              acc.push(Database::to_string(e).unwrap());
                                              acc
                                          });

                    return Some(f)
                }
            }
        }

        None
    }

    pub fn follow_user(&self, username: &str, followed: &str) -> bool {
        let added = self.add_into_string_array(username, followed, "following");

        if added {
            self.add_into_string_array(followed, username, "followers");
        }

        added
    }

    pub fn unfollow_user(&self, username: &str, followed: &str) -> bool {
        let deleted = self.delete_from_string_array(username, followed, "following");

        if deleted {
            self.delete_from_string_array(followed, username, "followers");
        }

        deleted
    }

    pub fn get_following(&self, username: &str) -> Option<Vec<String>> {
        self.get_string_array(username, "following")
    }

    pub fn get_followers(&self, username: &str) -> Option<Vec<String>> {
        self.get_string_array(username, "followers")
    }

    pub fn get_username_from_message(&self, id: &str) -> Option<String> {
        let coll = self.client.db("rutweet").collection("tweet");

        match ObjectId::with_string(id) {
            Err(_) => (),
            Ok(id) => {
                let doc = doc!{
                    "_id": id,
                };

                if let Ok(d) = coll.find_one(Some(doc.clone()), None) {
                    if let Some(item) = d {
                        return Some(Database::to_string(item.get("from").unwrap()).unwrap());
                    }
                }
            }
        };

        None
    }

    pub fn increment_tweet_like(&self, id: &str) -> bool {
        let coll = self.client.db("rutweet").collection("tweet");

        match ObjectId::with_string(id) {
            Err(_) => false,
            Ok(id) => {
                let doc = doc!{
                    "_id": id,
                };

                match coll.find_one(Some(doc.clone()), None) {
                    Err(_) => false,
                    Ok(d) => match d {
                        None => false,
                        Some(item) => {
                            let like = match item.get("like") {
                                None => 1,
                                Some(l) => l.as_i32().unwrap() + 1
                            };

                            match coll.update_one(doc,
                                                  doc!{"$set": {
                                                      "like": like
                                                  }},
                                                  None)
                            {
                                Err(_) => false,
                                Ok(_) => true
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn add_dm(&self, from: &str, to: &str, content: &str) -> Option<String> {
        let coll = self.client.db("rutweet").collection("dm");
        let doc = doc!{
            "from": from,
            "to": to,
            "content": content,
            "timestamp": Utc::now(),
        };

        let res = match coll.insert_one(doc.clone(), None) {
            Err(_) => Bson::Null,
            Ok(r) => r.inserted_id.unwrap()
        };

        match res {
            Bson::ObjectId(oid) => Some(oid.to_hex()),
            _ => None
        }
    }

    pub fn list_dm(&self, from: &str, to: &str) -> Vec<DM> {
        let coll = self.client.db("rutweet").collection("dm");
        let doc = doc!{
            "from": from,
            "to": to,
        };

        let mut dms = vec![];
        let cursor = coll.find(Some(doc.clone()), None).unwrap();

        /*
         * We retrieve the DMs with the same Tweet structure since we're
         * only using destination and content.
         */
        for result in cursor {
            if let Ok(item) = result {
                let timestamp = match item.get("timestamp") {
                    None => Utc::now().to_rfc3339(),
                    Some(d) => d.as_utc_date_time().unwrap().to_rfc3339()
                };

                dms.push(DM::new(from, to,
                                 &Database::to_string(item.get("content").unwrap()).unwrap(),
                                 &timestamp,
                                 &Database::to_string(item.get("_id").unwrap()).unwrap()))
            }
        }

        dms
    }

    pub fn is_following(&self, username: &str, following: &str) -> bool {
        self.is_in_array(username, following, "following")
    }

    pub fn is_blocked(&self, username: &str, blocked: &str) -> bool {
        self.is_in_array(username, blocked, "blocked")
    }

    pub fn block_user(&self, username: &str, blocked: &str) -> bool {
        self.add_into_string_array(username, blocked, "blocked")
    }

    pub fn unblock_user(&self, username: &str, blocked: &str) -> bool {
        self.delete_from_string_array(username, blocked, "blocked")
    }

    pub fn get_blocking(&self, username: &str) -> Option<Vec<String>> {
        self.get_string_array(username, "blocked")
    }
}

