use crate::app_error::AppError;
use crate::users;
use crate::users::{fetch_users, filter_users, UserFilterConfig};
use crate::usergroups;
use crate::channels;

pub fn list_members(
  token: String,
  email_filter: Vec<users::RegexGroupFilter>,
  username_filter: Vec<users::RegexGroupFilter>,
  sort_by: Option<users::SortUsersBy>,
) -> Result<(), AppError> {
  let client = reqwest::Client::new();

  let members = fetch_users(&client, token.as_ref(), sort_by)?;
  let filter_config = UserFilterConfig{
      username_filter,
      email_filter,
      ..Default::default()
  };

  let filtered_users = filter_users(members, &filter_config);
  println!("{}", filtered_users);
  return Ok(());
}

pub fn invite_members_to_channel(
  token: String,
  channel_name: String,
  email_filter: Vec<users::RegexGroupFilter>,
  username_filter: Vec<users::RegexGroupFilter>,
  sort_by: Option<users::SortUsersBy>,
) -> Result<(), AppError> {
  let client = reqwest::Client::new();

  let members = fetch_users(&client, token.as_ref(), sort_by)?;

  let filter_config = UserFilterConfig{
      username_filter,
      email_filter,
      ..Default::default()
  };
  let filtered_users = filter_users(members, &filter_config);
  for group in filtered_users.groups {
    let members = group.members;
    channels::add_members_to_channel(&client, &token, members, &channel_name)?;
  }
  return Ok(());
}

pub fn update_usergroup_members(
  token: String,
  email_filter: Vec<users::RegexGroupFilter>,
  username_filter: Vec<users::RegexGroupFilter>,
  sort_by: Option<users::SortUsersBy>,
) -> Result<(), AppError> {
  let client = reqwest::Client::new();

  let members = fetch_users(&client, token.as_ref(), sort_by)?;
  let filter_config = UserFilterConfig{
      username_filter,
      email_filter,
      ..Default::default()
  };

  let filtered_users = filter_users(members, &filter_config);
  println!("{}", filtered_users);

  usergroups::update_usergroup_members(&client, &token, filtered_users.groups)?;
  return Ok(());
}
