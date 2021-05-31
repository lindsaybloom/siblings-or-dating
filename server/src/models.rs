use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    #[serde(rename = "imgUrl")]
    pub image_url: String,
    pub dating: bool,
}

impl Post {
    #[allow(dead_code)]
    pub fn new(image_url: String, dating: bool) -> Self {
        Self {
            id: None,
            image_url,
            dating,
        }
    }
}
