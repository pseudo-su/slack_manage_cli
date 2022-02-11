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
pub struct ObjsBotProfileIcons {
    #[serde(rename = "image_36")]
    pub image_36: String,
    #[serde(rename = "image_48")]
    pub image_48: String,
    #[serde(rename = "image_72")]
    pub image_72: String,
}

impl ObjsBotProfileIcons {
    pub fn new(image_36: String, image_48: String, image_72: String) -> ObjsBotProfileIcons {
        ObjsBotProfileIcons {
            image_36,
            image_48,
            image_72,
        }
    }
}


