use crate::app_error::AppError;
use crate::users::{fetch_users, filter_members, UserFilterConfig};
use crate::usergroups;
use crate::channels;
use crate::cli_opts::{MemberQueryOpts};

pub fn list_members(
  token: String,
  query_opts: MemberQueryOpts
) -> Result<(), AppError> {
  let client = reqwest::blocking::Client::new();

  let filter_config = UserFilterConfig{
    filters: query_opts.into_filters(),
      ..Default::default()
  };
  let members = fetch_users(&client, token.as_ref(), query_opts.sort_by)?;

  let result = filter_members(members, &filter_config);
  println!("{}", result);
  return Ok(());
}

pub fn add_members_to_channel(
  token: String,
  channel_name: String,
  query_opts: MemberQueryOpts
) -> Result<(), AppError> {
  let client = reqwest::blocking::Client::new();

  let filter_config = UserFilterConfig{
    filters: query_opts.into_filters(),

      ..Default::default()
  };
  let members = fetch_users(&client, token.as_ref(), query_opts.sort_by)?;

  let result = filter_members(members, &filter_config);

  channels::add_members_to_channel(&client, &token, result.members, &channel_name)?;

  return Ok(());
}

pub fn update_usergroup_members(
  token: String,
  group_name: String,
  query_opts: MemberQueryOpts
) -> Result<(), AppError> {
  let client = reqwest::blocking::Client::new();

  let filter_config = UserFilterConfig{
      filters: query_opts.into_filters(),
      ..Default::default()
  };
  let members = fetch_users(&client, token.as_ref(), query_opts.sort_by)?;
  let result = filter_members(members, &filter_config);
  println!("{}", result);

  usergroups::update_usergroup_members(&client, &token, group_name, result.members)?;

  return Ok(());
}
