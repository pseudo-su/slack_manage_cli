use clap::{ ArgGroup };
use structopt::StructOpt;
use regex::Regex;
use crate::users;

fn token_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("token-group").required(true)
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case", raw(group = "token_arg_group()"))]
pub struct Opts {
    /// Print extra detail
    #[structopt(short, long)]
    pub verbose: bool,

    /// Print detail without performing any changes
    #[structopt(short, long)]
    pub dry_run: bool,

    #[structopt(subcommand)]
    pub command: Command,

    /// OAuth Access Token
    #[structopt(long, group = "token-group", env = "SLACK_OAUTH_ACCESS_TOKEN")]
    pub token: String,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
  #[structopt(name = "list-members")]
  ListMembers{
    /// File Containing Oauth Access Token
    #[structopt(short, long)]
    email_filter: Option<Vec<Regex>>,

    /// File Containing Oauth Access Token
    #[structopt(short, long)]
    username_filter: Option<Vec<Regex>>,

    #[structopt(short, long)]
    sort_by: Option<users::SortUsersBy>,
  },
  InviteMembers{
    #[structopt(index = 1)]
    channel_name: String,

    /// File Containing Oauth Access Token
    #[structopt(short, long)]
    email_filter: Option<Vec<Regex>>,

    /// File Containing Oauth Access Token
    #[structopt(short, long)]
    username_filter: Option<Vec<Regex>>,

    #[structopt(short, long)]
    sort_by: Option<users::SortUsersBy>,
  },
  UpdateUsergroupMembers{
    /// File Containing Oauth Access Token
    #[structopt(short, long)]
    email_filter: Option<Vec<Regex>>,

    /// File Containing Oauth Access Token
    #[structopt(short, long)]
    username_filter: Option<Vec<Regex>>,

    #[structopt(short, long)]
    sort_by: Option<users::SortUsersBy>,
  },
}

// fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<Error>>
// where
//     T: std::str::FromStr,
//     T::Err: Error + 'static,
//     U: std::str::FromStr,
//     U::Err: Error + 'static,
// {
//     let pos = s
//         .find('=')
//         .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
//     Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
// }

// #[derive(StructOpt, Debug)]
// struct Opt {
//     #[structopt(short = "D", parse(try_from_str = "parse_key_val"))]
//     defines: Vec<(String, i32)>,
// }
