// use slack_api::sync::{User};
// use slack_api::sync::channels;
// use slack_api::sync::channels::{ListRequest};

// use crate::app_error::AppError;

// impl From<channels::ListError<reqwest::Error>> for AppError {
//   fn from(e: channels::ListError<reqwest::Error>) -> Self {
//         return AppError {
//             message: format!("Error fetching channels: {}", e.to_string()),
//         };
//     }
// }

// pub fn get_channel(client: &reqwest::blocking::Client, token: &str, channel_name: &str) -> Result<String, AppError> {
//   let request = ListRequest{
//     exclude_archived: Some(true),
//     exclude_members: Some(true)
//   };
//   let resp = channels::list(client, token, &request)?;
//   if let Some(channels) = resp.channels {
//     let channel = channels.iter().find(|channel| {
//         channel.name.clone().map(|n| n == channel_name).unwrap_or(false)
//     });
//     if let Some(channel) = channel {
//       return Ok(channel.id.clone().unwrap());
//     }
//   }
//   Err(AppError{ message: format!("Unable to find channel {}", channel_name).to_owned() })
// }

// pub fn invite_user_to_channel(client: &reqwest::blocking::Client, token: &str, member: &User, channel_id: &str) -> Result<(), channels::InviteError<reqwest::Error>> {
//     let user_id = member.id.clone().unwrap();
//     let request = channels::InviteRequest{
//       channel: channel_id,
//       user: user_id.as_ref(),
//     };

//     channels::invite(client, token, &request)?;

//     Ok(())
// }

// // TODO: user a formatter and don't use println
// pub fn add_members_to_channel(client: &reqwest::blocking::Client, token: &str, members: Vec<User>, channel_name: &str) -> Result<(), AppError> {
//   for (i, member) in members.iter().enumerate() {
//     let name = member.name.clone().unwrap_or("--".to_owned());
//     let email = member.profile.clone()
//         .map(|p| p.email.unwrap_or("--".to_owned()))
//         .unwrap_or("--".to_owned());
//     let channel_id = get_channel(&client, token.as_ref(), channel_name.as_ref())?;

//     match invite_user_to_channel(&client, token.as_ref(), member, channel_id.as_ref()) {
//         Ok(_) => println!("{}. Inviting {} ({}) to #{}", i + 1, name, email, channel_name),
//         Err(err) => println!("{}. {} ({}), {}", i + 1, name, email, err),
//     };
//   }
//   Ok(())
// }
