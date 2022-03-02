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
pub struct ObjsResources {
    #[serde(rename = "excluded_ids", skip_serializing_if = "Option::is_none")]
    pub excluded_ids: Option<Vec<Vec<String>>>,
    #[serde(rename = "excluded_ids_temp_oneof_01", skip_serializing_if = "Option::is_none")]
    pub excluded_ids_temp_oneof_01: Option<Vec<Vec<String>>>,
    #[serde(rename = "ids")]
    pub ids: Vec<Vec<String>>,
    #[serde(rename = "ids_temp_oneof_01", skip_serializing_if = "Option::is_none")]
    pub ids_temp_oneof_01: Option<Vec<Vec<String>>>,
    #[serde(rename = "wildcard", skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<bool>,
}

impl ObjsResources {
    pub fn new(ids: Vec<Vec<String>>) -> ObjsResources {
        ObjsResources {
            excluded_ids: None,
            excluded_ids_temp_oneof_01: None,
            ids,
            ids_temp_oneof_01: None,
            wildcard: None,
        }
    }
}

