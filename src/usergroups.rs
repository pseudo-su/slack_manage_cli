use std::result::{Result};

use slack_api::usergroups;
use slack_api::usergroups_users;

use crate::users;
use crate::app_error::AppError;

impl From<usergroups_users::UpdateError<reqwest::Error>> for AppError {
  fn from(e: usergroups_users::UpdateError<reqwest::Error>) -> Self {
      return AppError {
          message: format!("Error updating users in usergroup: {}", e.to_string()),
      };
  }
}

impl From<usergroups::ListError<reqwest::Error>> for AppError {
  fn from(e: usergroups::ListError<reqwest::Error>) -> Self {
      return AppError {
          message: format!("Error fetching usergroups: {}", e.to_string()),
      };
  }
}

// TODO: convert to use a formatter and not use println!()
pub fn update_usergroup_members(client: &reqwest::Client, token: &str, groups: Vec<users::FilteredGroup>) -> Result<(), AppError> {
  // get list of usergroups
  let request = usergroups::ListRequest{
    ..usergroups::ListRequest::default()
  };
  let usergroups = usergroups::list(client, token, &request)?.usergroups.ok_or(AppError{ message: "No user groups found".to_owned() })?;

  for group in groups {
    let usergroup = usergroups
      .iter()
      .find(|g| g.name == group.group_name);

    if usergroup.is_none() {
      let name = group.group_name.unwrap_or("--".to_owned());
      return Err(AppError{
        message: format!("Group `{}` doesn't exist and cannot be updated", name)
      });
    }

    let usergroup_id = usergroup
      .and_then(|g| g.id.as_ref())
      .ok_or(AppError {
        message: "".to_owned(),
      })?;

    // create or update
    let userid_csv = group.members
      .iter()
      .filter_map(|u| u.id.to_owned())
      .collect::<Vec<String>>()
      .join(",");


    let request = usergroups_users::UpdateRequest{
      usergroup: usergroup_id.as_ref(),
      users: userid_csv.as_ref(),
      ..usergroups_users::UpdateRequest::default()
    };
    // usergroups_users::update(client, token, &request)?;
    print!("Updating user group `{}` with users `{}`", request.usergroup, request.users)
  }

  Ok(())
}
