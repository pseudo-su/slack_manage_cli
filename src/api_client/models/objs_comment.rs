/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjsComment {
    #[serde(rename = "comment")]
    pub comment: String,
    #[serde(rename = "created")]
    pub created: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "is_intro")]
    pub is_intro: bool,
    #[serde(rename = "is_starred", skip_serializing_if = "Option::is_none")]
    pub is_starred: Option<bool>,
    #[serde(rename = "num_stars", skip_serializing_if = "Option::is_none")]
    pub num_stars: Option<i32>,
    #[serde(rename = "pinned_info", skip_serializing_if = "Option::is_none")]
    pub pinned_info: Option<serde_json::Value>,
    #[serde(rename = "pinned_to", skip_serializing_if = "Option::is_none")]
    pub pinned_to: Option<Vec<String>>,
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<crate::api_client::models::ObjsReaction>>,
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    #[serde(rename = "user")]
    pub user: String,
}

impl ObjsComment {
    pub fn new(comment: String, created: i32, id: String, is_intro: bool, timestamp: i32, user: String) -> ObjsComment {
        ObjsComment {
            comment,
            created,
            id,
            is_intro,
            is_starred: None,
            num_stars: None,
            pinned_info: None,
            pinned_to: None,
            reactions: None,
            timestamp,
            user,
        }
    }
}


