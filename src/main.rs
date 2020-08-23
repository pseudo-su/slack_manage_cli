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

    let root_opts = Opts::from_args();

    let token: Result<String, AppError> = match root_opts.global_opts {
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

    match root_opts.command {
        cli_opts::Command::ListMembers(command_ops) => {
            commands::list_members(token, command_ops.query_opts)?
        },
        cli_opts::Command::AddMembersToChannel(command_opts) => {
            commands::add_members_to_channel(token, command_opts.channel_name, command_opts.query_opts)?
        },
        cli_opts::Command::UpdateUsergroupMembers(command_opts) => {
            commands::update_usergroup_members(token, command_opts.usergroup_name, command_opts.query_opts)?
        },
    };

    Ok(())
}
