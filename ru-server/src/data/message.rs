
#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub password: String,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub from: String,

    #[serde(default)]
    pub content: String,

    #[serde(default)]
    pub follow: String,

    #[serde(default)]
    pub to: String,

    #[serde(default)]
    pub block: String,

    #[serde(default)]
    pub token: String,
}

