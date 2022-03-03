/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * Contact: support@slack.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationsListIm {
    #[serde(rename = "created")]
    pub created: f32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "is_im")]
    pub is_im: bool,
    #[serde(rename = "is_org_shared")]
    pub is_org_shared: bool,
    #[serde(rename = "is_user_deleted")]
    pub is_user_deleted: bool,
    #[serde(rename = "priority")]
    pub priority: f32,
    #[serde(rename = "user")]
    pub user: String,
}

impl ConversationsListIm {
    pub fn new(created: f32, id: String, is_im: bool, is_org_shared: bool, is_user_deleted: bool, priority: f32, user: String) -> ConversationsListIm {
        ConversationsListIm {
            created,
            id,
            is_im,
            is_org_shared,
            is_user_deleted,
            priority,
            user,
        }
    }
}


