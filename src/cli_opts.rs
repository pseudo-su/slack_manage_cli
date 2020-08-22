use std::path::PathBuf;
use clap::{ AppSettings };
use structopt::{StructOpt};
use regex::Regex;
use std::error::Error;
use crate::users;

fn parse_regex_group(s: &str) -> Result<users::RegexGroupFilter, Box<dyn Error>> {
  let pos = s
    .find('=');
    // .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;

  if let Some(pos) = pos {
    let group_name: Option<String> = Some(s[..pos].to_owned());
    let regex = s[pos + 1..].to_owned();

    return Ok(users::RegexGroupFilter{
      group_name,
      regex: Regex::new(regex.as_str())?,
    })
  }

  Ok(users::RegexGroupFilter{
    group_name: None,
    regex: Regex::new(s)?,
  })
}

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
pub struct MemberGroupQueryOpts {
  /// File Containing Oauth Access Token
  #[structopt(short, long, parse(try_from_str = parse_regex_group), number_of_values = 1)]
  pub email_filter: Option<Vec<users::RegexGroupFilter>>,

  /// File Containing Oauth Access Token
  #[structopt(short, long, parse(try_from_str = parse_regex_group), number_of_values = 1)]
  pub username_filter: Option<Vec<users::RegexGroupFilter>>,

  #[structopt(short, long)]
  pub sort_by: Option<users::SortUsersBy>,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct ListMembers{
  #[structopt(flatten)]
  pub query_opts: MemberGroupQueryOpts,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct AddMembersToChannel{
  #[structopt(index = 1)]
  pub channel_name: String,

  #[structopt(flatten)]
  pub query_opts: MemberGroupQueryOpts,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct UpdateUsergroupMembers{

  #[structopt(flatten)]
  pub query_opts: MemberGroupQueryOpts,
}
