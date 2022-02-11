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
pub struct ObjsChannel {
    #[serde(rename = "accepted_user", skip_serializing_if = "Option::is_none")]
    pub accepted_user: Option<String>,
    #[serde(rename = "created")]
    pub created: i32,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "is_archived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "is_channel")]
    pub is_channel: bool,
    #[serde(rename = "is_frozen", skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    #[serde(rename = "is_general", skip_serializing_if = "Option::is_none")]
    pub is_general: Option<bool>,
    #[serde(rename = "is_member", skip_serializing_if = "Option::is_none")]
    pub is_member: Option<bool>,
    #[serde(rename = "is_moved", skip_serializing_if = "Option::is_none")]
    pub is_moved: Option<i32>,
    #[serde(rename = "is_mpim")]
    pub is_mpim: bool,
    #[serde(rename = "is_non_threadable", skip_serializing_if = "Option::is_none")]
    pub is_non_threadable: Option<bool>,
    #[serde(rename = "is_org_shared")]
    pub is_org_shared: bool,
    #[serde(rename = "is_pending_ext_shared", skip_serializing_if = "Option::is_none")]
    pub is_pending_ext_shared: Option<bool>,
    #[serde(rename = "is_private")]
    pub is_private: bool,
    #[serde(rename = "is_read_only", skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[serde(rename = "is_shared")]
    pub is_shared: bool,
    #[serde(rename = "is_thread_only", skip_serializing_if = "Option::is_none")]
    pub is_thread_only: Option<bool>,
    #[serde(rename = "last_read", skip_serializing_if = "Option::is_none")]
    pub last_read: Option<String>,
    #[serde(rename = "latest", skip_serializing_if = "Option::is_none")]
    pub latest: Option<serde_json::Value>,
    #[serde(rename = "members")]
    pub members: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "name_normalized")]
    pub name_normalized: String,
    #[serde(rename = "num_members", skip_serializing_if = "Option::is_none")]
    pub num_members: Option<i32>,
    #[serde(rename = "pending_shared", skip_serializing_if = "Option::is_none")]
    pub pending_shared: Option<Vec<String>>,
    #[serde(rename = "previous_names", skip_serializing_if = "Option::is_none")]
    pub previous_names: Option<Vec<String>>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<f32>,
    #[serde(rename = "purpose")]
    pub purpose: Box<crate::api_client::models::ObjsChannelPurpose>,
    #[serde(rename = "topic")]
    pub topic: Box<crate::api_client::models::ObjsChannelPurpose>,
    #[serde(rename = "unlinked", skip_serializing_if = "Option::is_none")]
    pub unlinked: Option<i32>,
    #[serde(rename = "unread_count", skip_serializing_if = "Option::is_none")]
    pub unread_count: Option<i32>,
    #[serde(rename = "unread_count_display", skip_serializing_if = "Option::is_none")]
    pub unread_count_display: Option<i32>,
}

impl ObjsChannel {
    pub fn new(created: i32, creator: String, id: String, is_channel: bool, is_mpim: bool, is_org_shared: bool, is_private: bool, is_shared: bool, members: Vec<String>, name: String, name_normalized: String, purpose: crate::api_client::models::ObjsChannelPurpose, topic: crate::api_client::models::ObjsChannelPurpose) -> ObjsChannel {
        ObjsChannel {
            accepted_user: None,
            created,
            creator,
            id,
            is_archived: None,
            is_channel,
            is_frozen: None,
            is_general: None,
            is_member: None,
            is_moved: None,
            is_mpim,
            is_non_threadable: None,
            is_org_shared,
            is_pending_ext_shared: None,
            is_private,
            is_read_only: None,
            is_shared,
            is_thread_only: None,
            last_read: None,
            latest: None,
            members,
            name,
            name_normalized,
            num_members: None,
            pending_shared: None,
            previous_names: None,
            priority: None,
            purpose: Box::new(purpose),
            topic: Box::new(topic),
            unlinked: None,
            unread_count: None,
            unread_count_display: None,
        }
    }
}


