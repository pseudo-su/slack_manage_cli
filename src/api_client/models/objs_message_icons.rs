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
pub struct ObjsMessageIcons {
    #[serde(rename = "emoji", skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(rename = "image_64", skip_serializing_if = "Option::is_none")]
    pub image_64: Option<String>,
}

impl ObjsMessageIcons {
    pub fn new() -> ObjsMessageIcons {
        ObjsMessageIcons {
            emoji: None,
            image_64: None,
        }
    }
}


