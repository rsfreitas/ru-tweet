
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::oid::ObjectId;
use mongodb::Bson;

use crate::data::tweet::{Tweet, Kind};

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
            "_id": ObjectId::with_string(id).unwrap(),
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

    pub fn get_tweet(&self, from: &str, id: &str) -> Option<Tweet> {
        let coll = self.client.db("rutweet").collection("tweet");
        let doc = doc!{
            "from": from,
            "_id": ObjectId::with_string(id).unwrap(),
        };

        match coll.find_one(Some(doc.clone()), None) {
            Err(_) => None,
            Ok(d) => match d {
                None => None,
                Some(item) => {
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

                    Some(Tweet::new(&Database::to_string(item.get("from").unwrap()).unwrap(),
                                    &Database::to_string(item.get("content").unwrap()).unwrap(),
                                    &Database::to_string(item.get("_id").unwrap()).unwrap(),
                                    like,
                                    kind))
                }
            }
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


                tweets.push(Tweet::new(&Database::to_string(item.get("from").unwrap()).unwrap(),
                                       &Database::to_string(item.get("content").unwrap()).unwrap(),
                                       &Database::to_string(item.get("_id").unwrap()).unwrap(),
                                       like,
                                       kind))
            }
        }

        tweets
    }

    pub fn follow_user(&self, username: &str, followed: &str) -> bool {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": username,
        };

        match coll.find_one(Some(doc.clone()), None) {
            Err(_) => false,
            Ok(d) => match d {
                None => false,
                Some(item) => {
                    let mut f = match item.get("following") {
                        None => vec![],
                        Some(dbv) => dbv.as_array().unwrap().to_vec()
                    };

                    if !f.contains(&Bson::from(followed)) {
                        f.push(Bson::from(followed));
                    }

                    match coll.update_one(doc,
                                          doc!{"$set": {
                                              "following": f.into_iter().map(Bson::from).collect::<Vec<_>>()
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

    pub fn unfollow_user(&self, username: &str, followed: &str) -> bool {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": username,
        };

        match coll.find_one(Some(doc.clone()), None) {
            Err(_) => false,
            Ok(d) => match d {
                None => false,
                Some(item) => {
                    let mut f = match item.get("following") {
                        None => vec![],
                        Some(dbv) => dbv.as_array().unwrap().to_vec()
                    };

                    if f.contains(&Bson::from(followed)) {
                        f.retain(|b| !b.as_str().unwrap().eq(followed));
                    }

                    match coll.update_one(doc,
                                          doc!{"$set": {
                                              "following": f.into_iter().map(Bson::from).collect::<Vec<_>>()
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

    pub fn get_following(&self, username: &str) -> Option<Vec<String>> {
        let coll = self.client.db("rutweet").collection("users");
        let doc = doc!{
            "name": username,
        };

        match coll.find_one(Some(doc.clone()), None) {
            Err(_) => None,
            Ok(d) => match d {
                None => None,
                Some(item) => {
                    match item.get("following") {
                        None => None,
                        Some(dbv) => {
                            let v = dbv.as_array().unwrap().to_vec();
                            let f = v.iter().fold(vec![],
                                                  |mut acc, e| {
                                                      acc.push(Database::to_string(e).unwrap());
                                                      acc
                                                  });

                            Some(f)
                        }
                    }
                }
            }
        }
    }

    pub fn increment_tweet_like(&self, id: &str) -> bool {
        let coll = self.client.db("rutweet").collection("tweet");
        let doc = doc!{
            "_id": ObjectId::with_string(id).unwrap(),
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

