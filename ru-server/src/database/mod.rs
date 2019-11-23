
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::Bson;

use crate::data::tweet::Tweet;

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

    pub fn add_tweet(&self, from: &str, content: &str) -> Option<String> {
        let coll = self.client.db("rutweet").collection("tweet");
        let doc = doc!{
            "from": from,
            "content": content,
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
        let doc = doc!{
            "from": from,
            "id": id,
        };

        match coll.delete_one(doc.clone(), None) {
            Err(_) => false,
            Ok(r) => match r.deleted_count {
                0 => false,
                _ => true
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

    pub fn list_tweet(&self, from: &str) -> Vec<Tweet> {
        let coll = self.client.db("rutweet").collection("tweet");
        let doc = doc!{
            "from": from,
        };

        let mut tweets = vec![];
        let cursor = coll.find(Some(doc.clone()), None).unwrap();

        for result in cursor {
            if let Ok(item) = result {
                tweets.push(Tweet::new(&Database::to_string(item.get("from").unwrap()).unwrap(),
                                       &Database::to_string(item.get("content").unwrap()).unwrap(),
                                       &Database::to_string(item.get("_id").unwrap()).unwrap()));
            }
        }

        tweets
    }
}

