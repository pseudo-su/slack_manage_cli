/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UsersIdentitySchemaTempOneof02 : Schema for 'identity.basic,identity.avatar' scopes



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UsersIdentitySchemaTempOneof02 {
    #[serde(rename = "ok")]
    pub ok: crate::api_client::models::DefsOkTrue,
    #[serde(rename = "team")]
    pub team: Box<crate::api_client::models::UsersIdentitySchemaTempOneof00Team>,
    #[serde(rename = "user")]
    pub user: Box<crate::api_client::models::UsersIdentitySchemaTempOneof02User>,
}

impl UsersIdentitySchemaTempOneof02 {
    /// Schema for 'identity.basic,identity.avatar' scopes
    pub fn new(ok: crate::api_client::models::DefsOkTrue, team: crate::api_client::models::UsersIdentitySchemaTempOneof00Team, user: crate::api_client::models::UsersIdentitySchemaTempOneof02User) -> UsersIdentitySchemaTempOneof02 {
        UsersIdentitySchemaTempOneof02 {
            ok,
            team: Box::new(team),
            user: Box::new(user),
        }
    }
}

