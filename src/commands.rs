use crate::api_client::apis::configuration::Configuration;
use crate::app_error::AppError;
use crate::users::{fetch_users, filter_members, UserFilterConfig};
// use crate::usergroups;
// use crate::channels;
use crate::cli_opts::{MemberQueryOpts};

pub async fn list_members(
  token: String,
  query_opts: MemberQueryOpts
) -> Result<(), AppError> {
  let client_config = Configuration::default();

  let filter_config = UserFilterConfig{
    filters: query_opts.into_filters(),
    skip_bots: !query_opts.include_bots,
    skip_restricted: !query_opts.include_restricted,
    skip_ultra_restricted: !query_opts.include_ultra_restricted,
    skip_full_members: query_opts.skip_full_members,
  };
  let members = fetch_users(&client_config, token.as_ref(), query_opts.sort_by).await?;

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
    skip_bots: !query_opts.include_bots,
    skip_restricted: !query_opts.include_restricted,
    skip_ultra_restricted: !query_opts.include_ultra_restricted,
    skip_full_members: query_opts.skip_full_members,
  };
  // let members = fetch_users(&client, token.as_ref(), query_opts.sort_by)?;

  // let result = filter_members(members, &filter_config);

  // TODO: uncomment
  // channels::add_members_to_channel(&client, &token, result.members, &channel_name)?;

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
      skip_bots: !query_opts.include_bots,
      skip_restricted: !query_opts.include_restricted,
      skip_ultra_restricted: !query_opts.include_ultra_restricted,
      skip_full_members: query_opts.skip_full_members,
  };
  // let members = fetch_users(&client, token.as_ref(), query_opts.sort_by)?;
  // let result = filter_members(members, &filter_config);
  // println!("{}", result);

  // TODO: uncomment
  // usergroups::update_usergroup_members(&client, &token, group_name, result.members)?;

  return Ok(());
}
