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
pub struct ConversationsListChannelPurpose {
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "last_set")]
    pub last_set: f32,
    #[serde(rename = "value")]
    pub value: String,
}

impl ConversationsListChannelPurpose {
    pub fn new(creator: String, last_set: f32, value: String) -> ConversationsListChannelPurpose {
        ConversationsListChannelPurpose {
            creator,
            last_set,
            value,
        }
    }
}


