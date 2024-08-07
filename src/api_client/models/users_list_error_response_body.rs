/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * Contact: support@slack.com
 * Generated by: https://openapi-generator.tech
 */

/// UsersListErrorResponseBody : Schema for error response from users.list method



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UsersListErrorResponseBody {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "ok")]
    pub ok: bool,
}

impl UsersListErrorResponseBody {
    /// Schema for error response from users.list method
    pub fn new(error: String, ok: bool) -> UsersListErrorResponseBody {
        UsersListErrorResponseBody {
            error,
            ok,
        }
    }
}


