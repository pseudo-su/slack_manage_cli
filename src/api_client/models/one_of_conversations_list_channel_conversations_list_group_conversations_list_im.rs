use enum_as_inner::EnumAsInner;

// // #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumAsInner)]
// #[serde(untagged)]
// pub enum OneOfconversationsListChannelconversationsListGroupconversationsListIm {
//     ConversationsListChannel(crate::api_client::models::ConversationsListChannel),
//     ConversationsListGroup(crate::api_client::models::ConversationsListGroup),
//     ConversationsListIm(crate::api_client::models::ConversationsListIm),
// }

// #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumAsInner)]
#[serde(untagged)]
pub enum OneOfconversationsListChannelconversationsListIm {
    ConversationsListChannel(crate::api_client::models::ConversationsListChannel),
    ConversationsListIm(crate::api_client::models::ConversationsListIm),
}
