/// 
// #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOfconversationsListChannelconversationsListGroupconversationsListIm {
    ConversationsListChannel(crate::api_client::models::ConversationsListChannel),
    ConversationsListGroup(crate::api_client::models::ConversationsListGroup),
    ConversationsListIm(crate::api_client::models::ConversationsListIm),
}
