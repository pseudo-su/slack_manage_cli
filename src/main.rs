use dotenv;
use regex::Regex;

mod app_error;
mod channels;
mod users;
mod cli;
use cli::Opts;
use app_error::AppError;
use users::{fetch_users, filter_users, print_users, UserFilterConfig, PrintUsersConfig };
use channels::{ invite_user_to_channel, get_channel };
use structopt::StructOpt;

fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();

    let opts = Opts::from_args();

    let token = opts.token;
    // let verbose = opts.verbose;
    // let dry_run = opts.dry_run;

    match opts.command {
        cli::Command::ListMembers { email_filter, username_filter, sort_by } => {
            list_members(
                token,
                email_filter.unwrap_or(vec![]),
                username_filter.unwrap_or(vec![]),
                sort_by
            )
        },
        cli::Command::InviteMembers { channel_name, email_filter, username_filter, sort_by } => {
            invite_members_to_channel(
                token,
                channel_name,
                email_filter.unwrap_or(vec![]),
                username_filter.unwrap_or(vec![]),
                sort_by
            )
        },
        cli::Command::UpdateUsergroupMembers { email_filter, username_filter, sort_by } => {
            update_usergroup_members(
                token,
                email_filter.unwrap_or(vec![]),
                username_filter.unwrap_or(vec![]),
                sort_by
            )
        },
    }
}

fn list_members(
    token: String,
    email_filter: Vec<Regex>,
    username_filter: Vec<Regex>,
    sort_by: Option<users::SortUsersBy>,
) -> Result<(), AppError> {
    let client = reqwest::Client::new();

    let members = fetch_users(&client, token.as_ref(), sort_by)?;
    let filter_config = UserFilterConfig{
        username_filter: username_filter,
        email_filter: email_filter,
        ..Default::default()
    };

    let member_groups = filter_users(members, &filter_config);
    let print_config = PrintUsersConfig{ ..Default::default() };
    for (title, members) in member_groups {
        print_users(title, members, &print_config);
    }
    return Ok(());
}

fn invite_members_to_channel(
    token: String,
    channel_name: String,
    email_filter: Vec<Regex>,
    username_filter: Vec<Regex>,
    sort_by: Option<users::SortUsersBy>,
) -> Result<(), AppError> {
    let client = reqwest::Client::new();

    let members = fetch_users(&client, token.as_ref(), sort_by)?;

    let filter_config = UserFilterConfig{
        username_filter: username_filter,
        email_filter: email_filter,
        ..Default::default()
    };
    let member_groups = filter_users(members, &filter_config);
    for (title, members) in member_groups {
        println!("{}", title);
        for (i, member) in members.iter().enumerate() {
            let name = member.name.clone().unwrap_or("--".to_owned());
            let email = member.profile.clone()
                .map(|p| p.email.unwrap_or("--".to_owned()))
                .unwrap_or("--".to_owned());
            let channel_id = get_channel(&client, token.as_ref(), channel_name.as_ref())?;

            match invite_user_to_channel(&client, token.as_ref(), member, channel_id.as_ref()) {
                Ok(_) => println!("{}. Inviting {} ({}) to #{}", i + 1, name, email, channel_name),
                Err(err) => println!("{}. {} ({}), {}", i + 1, name, email, err),
            };
        }
    }
    return Ok(());
}

fn update_usergroup_members(
    token: String,
    email_filter: Vec<Regex>,
    username_filter: Vec<Regex>,
    sort_by: Option<users::SortUsersBy>,
) -> Result<(), AppError> {
    let client = reqwest::Client::new();

    let members = fetch_users(&client, token.as_ref(), sort_by)?;
    let filter_config = UserFilterConfig{
        username_filter: username_filter,
        email_filter: email_filter,
        ..Default::default()
    };

    let member_groups = filter_users(members, &filter_config);
    let print_config = PrintUsersConfig{ ..Default::default() };
    for (title, members) in member_groups {
        print_users(title, members, &print_config);
    }
    return Ok(());
}
