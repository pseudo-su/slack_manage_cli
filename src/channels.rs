use slack_api::{User};
use slack_api::channels;
use slack_api::channels::{ListRequest};

use crate::app_error::AppError;

impl From<channels::ListError<reqwest::Error>> for AppError {
    fn from(_: channels::ListError<reqwest::Error>) -> Self {
        return AppError {
            message: "Error fetching channels".to_owned(),
        };
    }
}

pub fn get_channel(client: &reqwest::Client, token: &str, channel_name: &str) -> Result<String, AppError> {
  let request = ListRequest{
    exclude_archived: Some(true),
    exclude_members: Some(true)
  };
  let resp = channels::list(client, token, &request)?;
  if let Some(channels) = resp.channels {
    let channel = channels.iter().find(|channel| {
        channel.name.clone().map(|n| n == channel_name).unwrap_or(false)
    });
    if let Some(channel) = channel {
      return Ok(channel.id.clone().unwrap());
    }
  }
  Err(AppError{ message: format!("Unable to find channel {}", channel_name).to_owned() })
}

pub fn invite_user_to_channel(client: &reqwest::Client, token: &str, member: &User, channel_id: &str) -> Result<(), channels::InviteError<reqwest::Error>> {
    let user_id = member.id.clone().unwrap();
    let request = channels::InviteRequest{
      channel: channel_id,
      user: user_id.as_ref(),
    };

    channels::invite(client, token, &request)?;

    Ok(())
}
