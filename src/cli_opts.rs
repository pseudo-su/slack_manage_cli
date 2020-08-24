use std::path::PathBuf;
use clap::{ AppSettings };
use structopt::{StructOpt};
use regex::Regex;
use crate::users;
use users::{UserFilterOn, UserFilter};

#[derive(StructOpt, Debug)]
#[structopt(
  rename_all = "kebab-case",
  setting = AppSettings::InferSubcommands,
)]
pub struct Opts {
    #[structopt(flatten)]
    pub global_opts: GlobalOpts,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
#[structopt(
  rename_all = "kebab-case",
  // TODO: turning this on breaks the global=true
  // group = ArgGroup::with_name("slack-api-token").required(true),
)]
pub struct GlobalOpts {
  /// Print extra detail
  #[structopt(short, long, global= true)]
  pub verbose: bool,

  /// Print detail without performing any changes
  #[structopt(short, long, global = true)]
  pub dry_run: bool,
  
  /// OAuth Access Token
  #[structopt(long, group = "slack-api-token", env = "SLACK_OAUTH_ACCESS_TOKEN", global = true)]
  pub token: Option<String>,

  // Oauth Access Token filename
  #[structopt(long = "token-file", group="slack-api-token", parse(from_os_str), global = true)]
  pub token_filepath: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
  ListMembers(ListMembers),
  AddMembersToChannel(AddMembersToChannel),
  UpdateUsergroupMembers(UpdateUsergroupMembers),
}

#[derive(StructOpt, Debug)]
pub struct MemberQueryOpts {
  #[structopt(long)]
  pub email_match: Option<Vec<Regex>>,
  #[structopt(long)]
  pub email_nomatch: Option<Vec<Regex>>,

  #[structopt(long)]
  pub username_match: Option<Vec<Regex>>,
  #[structopt(long)]
  pub username_nomatch: Option<Vec<Regex>>,

  #[structopt(long)]
  pub sort_by: Option<users::SortUsersBy>,

  #[structopt(long)]
  pub include_bots: bool,

  #[structopt(long)]
  pub include_restricted: bool,

  #[structopt(long)]
  pub include_ultra_restricted: bool,

  #[structopt(long)]
  pub skip_full_members: bool,

}

impl MemberQueryOpts {
  pub fn into_filters(&self) -> Vec<UserFilter> {
    // Username
    let username_match: Vec<UserFilter> = self.username_match
      .iter()
      .flatten()
      .map(
          |r| UserFilter{filter_on: UserFilterOn::Username, regex: r.to_owned(), should_match: true }
      )
      .collect();
    let username_nomatch: Vec<UserFilter> = self.username_nomatch
      .iter()
      .flatten()
      .map(
          |r| UserFilter{filter_on: UserFilterOn::Username, regex: r.to_owned(), should_match: false }
      )
      .collect();

    // Email
    let email_match: Vec<UserFilter> = self.email_match
      .iter()
      .flatten()
      .map(
          |r| UserFilter{filter_on: UserFilterOn::Email, regex: r.to_owned(), should_match: true }
      )
      .collect();
    let email_nomatch: Vec<UserFilter> = self.email_nomatch
      .iter()
      .flatten()
      .map(
          |r| UserFilter{filter_on: UserFilterOn::Email, regex: r.to_owned(), should_match: false }
      )
      .collect();
    
      // join and return
      vec![username_match, username_nomatch, email_match, email_nomatch].iter()
        .flatten()
        .map(|f| f.clone()).collect()
  }
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct ListMembers{
  #[structopt(flatten)]
  pub query_opts: MemberQueryOpts,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct AddMembersToChannel{
  #[structopt(index = 1)]
  pub channel_name: String,

  #[structopt(flatten)]
  pub query_opts: MemberQueryOpts,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct UpdateUsergroupMembers{
  #[structopt(index = 1)]
  pub usergroup_name: String,

  #[structopt(flatten)]
  pub query_opts: MemberQueryOpts,
}
