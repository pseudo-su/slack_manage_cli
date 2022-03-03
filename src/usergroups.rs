// use std::result::{Result};

// use slack_api::sync::usergroups;
// use slack_api::sync::{User, usergroups_users};

// use crate::app_error::AppError;

// impl From<usergroups_users::UpdateError<reqwest::Error>> for AppError {
//   fn from(e: usergroups_users::UpdateError<reqwest::Error>) -> Self {
//       return AppError {
//           message: format!("Error updating users in usergroup: {}", e.to_string()),
//       };
//   }
// }

// impl From<usergroups::ListError<reqwest::Error>> for AppError {
//   fn from(e: usergroups::ListError<reqwest::Error>) -> Self {
//       // panic!("{}", match e {
//       //   usergroups::ListError::MalformedResponse(e) => e.to_string(),
//       //   _ => "".to_owned(),
//       // });
//       return AppError {
//           message: format!("Error fetching usergroups: {}", e.to_string()),
//       };
//   }
// }

// // TODO: convert to use a formatter and not use println!()
// pub fn update_usergroup_members(client: &reqwest::blocking::Client, token: &str, group_name: String, members: Vec<User>) -> Result<(), AppError> {
//   // get list of usergroups
//   let request = usergroups::ListRequest{
//     ..usergroups::ListRequest::default()
//   };
//   let usergroups = usergroups::list(client, token, &request)?.usergroups.ok_or(AppError{ message: "No user groups found".to_owned() })?;

//   let usergroup = usergroups
//     .iter()
//     .find(|&g| g.name == Some(group_name.to_owned()))
//     .ok_or(AppError{
//       message: format!("Unable to update usergroup members: Group `{}` doesn't exist", group_name),
//     })?;

//   let usergroup_id = usergroup.clone().id
//     .ok_or(AppError {
//       message: "Unable to update usergroup with unknown ID".to_owned(),
//     })?;

//   // create or update
//   let userid_csv = members
//     .iter()
//     .filter_map(|u| u.id.to_owned())
//     .collect::<Vec<String>>()
//     .join(",");

//   let request = usergroups_users::UpdateRequest{
//     usergroup: usergroup_id.as_ref(),
//     users: userid_csv.as_ref(),
//     // TODO: https://github.com/slack-rs/slack-rs-api/issues/98
//     // include_count: Some(false),
//     ..usergroups_users::UpdateRequest::default()
//   };
//   // usergroups_users::update(client, token, &request)?;
//   print!("Updating user group `{} ({})` with users `{}`", usergroup.clone().name.unwrap_or("--".to_owned()), request.usergroup, request.users);

//   usergroups_users::update(client, token, &request)?;

//   Ok(())
// }
