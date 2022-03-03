use crate::{
  app_error::AppError,
  api_client::apis::conversations_api,
  api_client::{apis::configuration::Configuration, models::{ConversationsListChannel, ConversationsListConversationType, UsersListMember}},
};
use crate::api_client::models::OneOfconversationsListChannelconversationsListIm as AnyConversation;

impl From<crate::api_client::apis::Error<crate::api_client::apis::conversations_api::ConversationsListError>> for AppError {
  fn from(e: crate::api_client::apis::Error<crate::api_client::apis::conversations_api::ConversationsListError>) -> Self {
        return AppError {
            message: format!("Error fetching channels: {}", e.to_string()),
        };
    }
}

pub async fn get_channel(client_config: &Configuration, token: &str, channel_name: &str) -> Result<String, AppError> {

  let exclude_archived = Some(true);
  let types = Some(
    vec![
      ConversationsListConversationType::PublicChannel,
      // TODO: include private channels?
      // ConversationsListConversationType::PrivateChannel,
    ]
  );
  let limit = None;
  let cursor = None;

  let resp = conversations_api::conversations_list(client_config, Some(token), exclude_archived, types, limit, cursor).await?;

  if !resp.ok {
    return Err(AppError{ message: "Error fetching conversations".to_owned() });
  }

  let name = resp.channels.iter()
    .find(|&channel| {
      match channel {
        AnyConversation::ConversationsListChannel(ConversationsListChannel{name, ..}) => name.eq(channel_name),
        _ => false,
      }
    })
    .and_then(|channel| channel.as_conversations_list_channel())
    .and_then(|channel| Some(channel.name.clone()));

  if let Some(name) = name {
    Ok(name)
  } else {
    Err(AppError{ message: format!("Unable to find channel {}", channel_name).to_owned() })
  }
}

pub async fn invite_user_to_channel(client: &Configuration, token: &str, member: &UsersListMember, channel_id: &str) -> Result<(), AppError> {
    // let user_id = member.id.clone();
    // let request = channels::InviteRequest{
    //   channel: channel_id,
    //   user: user_id.as_ref(),
    // };

    // converstaions_api::invite(client, token, &request)?;

    println!("TODO: invite_user_to_channel not implemented");

    Ok(())
}

// TODO: user a formatter and don't use println
pub async fn add_members_to_channel(client_config: &Configuration, token: &str, members: Vec<UsersListMember>, channel_name: &str) -> Result<(), AppError> {
  for (i, member) in members.iter().enumerate() {
    let name = member.name.clone();
    let email = member.profile.clone().email.unwrap_or("".to_owned());

    let channel_id = get_channel(&client_config, token.as_ref(), channel_name.as_ref()).await?;

    match invite_user_to_channel(&client_config, token.as_ref(), member, channel_id.as_ref()).await {
        Ok(_) => println!("{}. Inviting {} ({}) to #{}", i + 1, name, email, channel_name),
        Err(err) => println!("{}. {} ({}), {}", i + 1, name, email, err.message),
    };
  }
  Ok(())
}
