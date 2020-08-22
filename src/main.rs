use std::fs;
use dotenv;

mod commands;
mod usergroups;
mod app_error;
mod channels;
mod users;
mod cli_opts;
use cli_opts::{Opts,GlobalOpts};
use app_error::AppError;
use structopt::StructOpt;

fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();

    let opts = Opts::from_args();

    let token: Result<String, AppError> = match opts.global_opts {
        GlobalOpts {
            token_filepath: Some(filepath),
            ..
        } => {
            filepath.canonicalize()
                .and_then(|filepath| fs::read_to_string(&filepath))
                .map(|t| t.trim().to_owned())
                .map_err(|e|
                    AppError{ message: format!("{}", e)}
                )
        },
        GlobalOpts{
            token: Some(token),
            ..
        } => Ok(token.to_owned()),
        _ => Err(AppError{ message: "--token or --token-file is required".to_owned()})
    };

    let token = token?;
    // let verbose = opts.verbose;
    // let dry_run = opts.dry_run;

    match opts.command {
        cli_opts::Command::ListMembers(opts) => {
            // { email_filter, username_filter, sort_by }
            commands::list_members(
                token,
                opts.query_opts.email_filter.unwrap_or(vec![]),
                opts.query_opts.username_filter.unwrap_or(vec![]),
                opts.query_opts.sort_by,
            )
        },
        cli_opts::Command::AddMembersToChannel(opts) => {
            // { channel_name, email_filter, username_filter, sort_by }
            commands::invite_members_to_channel(
                token,
                opts.channel_name,
                opts.query_opts.email_filter.unwrap_or(vec![]),
                opts.query_opts.username_filter.unwrap_or(vec![]),
                opts.query_opts.sort_by,
            )
        },
        cli_opts::Command::UpdateUsergroupMembers(opts) => {
            // { email_filter, username_filter, sort_by }
            commands::update_usergroup_members(
                token,
                opts.query_opts.email_filter.unwrap_or(vec![]),
                opts.query_opts.username_filter.unwrap_or(vec![]),
                opts.query_opts.sort_by,
            )
        },
    }
}
