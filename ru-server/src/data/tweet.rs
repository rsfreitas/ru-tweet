
use chrono::Utc;
use mongodb::Bson;

#[derive(Debug, Serialize, Deserialize)]
pub enum Kind {
    Simple,
    Retweet
}

impl Kind {
    pub fn to_bson(&self) -> Bson {
        let s = match self {
            Kind::Simple => "simple",
            Kind::Retweet => "retweet"
        };

        Bson::String(s.to_string())
    }

    pub fn from_bson(b: &Bson) -> Option<Kind> {
        match b {
            Bson::String(s) => {
                let k = s.to_string();
                match k.as_ref() {
                    "simple" => Some(Kind::Simple),
                    "retweet" => Some(Kind::Retweet),
                    _ => None,
                };
                None
            },
            _ => None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    from: String,
    content: String,
    id: String,
    timestamp: String,
    like: i32,
    kind: Kind,
}

impl Tweet {
    pub fn new(from: &str, content: &str, id: &str, like: i32, kind: Kind, timestamp: &str) -> Tweet {
        Tweet{
            from: from.to_string(),
            content: content.to_string(),
            id: id.to_string(),
            timestamp: timestamp.to_string(),
            like: like,
            kind: kind,
        }
    }

    pub fn new_empty() -> Tweet {
        let timestamp = Utc::now();

        Tweet{
            from: "".to_string(),
            content: "".to_string(),
            id: "".to_string(),
            timestamp: timestamp.to_rfc3339(),
            like: 0,
            kind: Kind::Simple,
        }
    }
}

