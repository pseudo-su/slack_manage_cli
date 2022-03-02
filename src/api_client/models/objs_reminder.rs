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
pub struct ObjsReminder {
    #[serde(rename = "complete_ts", skip_serializing_if = "Option::is_none")]
    pub complete_ts: Option<i32>,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "recurring")]
    pub recurring: bool,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
    #[serde(rename = "user")]
    pub user: String,
}

impl ObjsReminder {
    pub fn new(creator: String, id: String, recurring: bool, text: String, user: String) -> ObjsReminder {
        ObjsReminder {
            complete_ts: None,
            creator,
            id,
            recurring,
            text,
            time: None,
            user,
        }
    }
}

