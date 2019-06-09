use clap::{ Arg, ArgGroup, ArgMatches, App, AppSettings, SubCommand };
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

mod app_error;
mod channels;
mod users;
use app_error::AppError;
use users::{fetch_users, filter_users, print_users, UserFilterConfig, PrintUsersConfig };
use channels::{ invite_user_to_channel, get_channel };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), AppError> {
    let token_arg = Arg::with_name("token")
        .long("token")
        .value_name("oauth-access-token")
        .help("OAuth Access Token")
        .takes_value(true);

    let token_file_arg = Arg::with_name("token-filepath")
        .long("token-filepath")
        .value_name("oauth-access-token-filepath")
        .help("Path to file containing access token as text")
        .takes_value(true);

    let token_group = ArgGroup::with_name("baz")
        .args(&["token", "token-filepath"])
        .required(true);

    let email_filter_arg = Arg::with_name("email-filter")
        .takes_value(true)
        .multiple(true)
        .long("email-filter");

    let username_filter_arg = Arg::with_name("username-filter")
        .takes_value(true)
        .multiple(true)
        .long("username-filter");

    let sort_users_by_arg = Arg::with_name("sort-users-by")
        .takes_value(true)
        .long("sort");

    let matches = App::new("slack-manage")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(VERSION)
        .about("Various tools for managing a slack server")
        .subcommand(SubCommand::with_name("list-members")
            .about("controls testing features")
            // Auth
            .arg(token_arg.clone())
            .arg(token_file_arg.clone())
            .group(token_group.clone())
            // Filter
            .arg(email_filter_arg.clone())
            .arg(username_filter_arg.clone())
            // presentational
            .arg(sort_users_by_arg.clone())
        )
        .subcommand(SubCommand::with_name("invite-members")
            .about("controls testing features")
            .arg(Arg::with_name("channel-name")
                .required(true)
                .index(1))
            // Auth
            .arg(token_arg.clone())
            .arg(token_file_arg.clone())
            .group(token_group.clone())
            // Filter
            .arg(email_filter_arg.clone())
            .arg(username_filter_arg.clone())
            // presentational
            .arg(sort_users_by_arg.clone())
        )
        .subcommand(SubCommand::with_name("update-usergroup-members")
            .about("controls testing features")
            // TODO: ArgGroup one of (id|name|handle)
            .arg(Arg::with_name("usergroup")
                .required(true)
                .index(1))
            // Auth
            .arg(token_arg.clone())
            .arg(token_file_arg.clone())
            .group(token_group.clone())
            // Filter
            .arg(email_filter_arg.clone())
            .arg(username_filter_arg.clone())
            // presentational
            .arg(sort_users_by_arg.clone())
        )
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("list-members") {
        return list_members(matches);
    }

    if let Some(matches) = matches.subcommand_matches("invite-members") {
        return invite_members_to_channel(matches);
    }

    if let Some(matches) = matches.subcommand_matches("update-usergroup-members") {
        return update_usergroup_members(matches);
    }

    Ok(())
}

fn load_token(token: Option<&str>, token_filepath: Option<&str> ) -> Result<String, AppError> {
    if let Some(token) = token {
        return Ok(token.to_owned());
    }
    if let Some(filepath) = token_filepath {
        let mut file = File::open(filepath).map_err(|_| AppError{ message: "".to_owned() })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|_| AppError{ message: "".to_owned() })?;

        return Ok(contents.trim().to_owned());
    }
    return Err(AppError{ message: "Unable to load token".to_owned() });
}

// TODO: had to copy this here Because IntoIter didn't work (I had to `.clone()`)
macro_rules! value_t {
    ($m:ident.value_of($v:expr), $t:ty) => {
        match $m.value_of($v) {
            Some(v) => {
                match v.parse::<$t>() {
                    Ok(val) => Ok(val),
                    Err(_)  => Err(format!("'{}' isn't a valid value", ::clap::Format::Warning(v))),
                }
            },
            None => Err(format!("The argument '{}' not found", ::clap::Format::Warning($v)))
        }
    };
    ($m:ident.values_of($v:expr), $t:ty) => {
        match $m.values_of($v) {
            Some(ref v) => {
                let mut tmp = Vec::with_capacity(v.len());
                let mut err = None;
                for pv in v.clone() {
                    match pv.parse::<$t>() {
                        Ok(rv) => tmp.push(rv),
                        Err(e) => {
                            err = Some(format!("'{}' isn't a valid value\n\t{}", ::clap::Format::Warning(pv),e));
                            break
                        }
                    }
                }
                match err {
                    Some(e) => Err(e),
                    None => Ok(tmp)
                }
            },
            None => Err(format!("The argument '{}' was not found", ::clap::Format::Warning($v)))
        }
    };
}

fn list_members(matches: &ArgMatches) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let oauth_api_key = matches.value_of("token");
    let oauth_api_key_filepath = matches.value_of("token-filepath");
    let token = load_token(oauth_api_key, oauth_api_key_filepath)?;

    let email_filter = value_t!(matches.values_of("email-filter"), Regex).unwrap_or(vec![]);
    let username_filter = value_t!(matches.values_of("username-filter"), Regex).unwrap_or(vec![]);
    let sort_by = value_t!(matches.value_of("sort-users-by"), users::SortUsersBy).ok();

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

fn invite_members_to_channel(matches: &ArgMatches) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let oauth_api_key = matches.value_of("token");
    let oauth_api_key_filepath = matches.value_of("token-filepath");
    let token = load_token(oauth_api_key, oauth_api_key_filepath)?;

    let email_filter = value_t!(matches.values_of("email-filter"), Regex).unwrap_or(vec![]);
    let username_filter = value_t!(matches.values_of("username-filter"), Regex).unwrap_or(vec![]);
    let sort_by = value_t!(matches.value_of("sort-users-by"), users::SortUsersBy).ok();

    let channel_name = matches.value_of("channel-name").unwrap();

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
            let channel_id = get_channel(&client, token.as_ref(), channel_name)?;

            match invite_user_to_channel(&client, token.as_ref(), member, channel_id.as_ref()) {
                Ok(_) => println!("{}. Inviting {} ({}) to #{}", i + 1, name, email, channel_name),
                Err(err) => println!("{}. {} ({}), {}", i + 1, name, email, err),
            };
        }
    }
    return Ok(());
}

fn update_usergroup_members(matches: &ArgMatches) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let oauth_api_key = matches.value_of("token");
    let oauth_api_key_filepath = matches.value_of("token-filepath");
    let token = load_token(oauth_api_key, oauth_api_key_filepath)?;

    let email_filter = value_t!(matches.values_of("email-filter"), Regex).unwrap_or(vec![]);
    let username_filter = value_t!(matches.values_of("username-filter"), Regex).unwrap_or(vec![]);
    let sort_by = value_t!(matches.value_of("sort-users-by"), users::SortUsersBy).ok();

    let members = fetch_users(&client, token.as_ref(), sort_by)?;
    let filter_config = UserFilterConfig{
        username_filter: username_filter,
        email_filter: email_filter,
        ..Default::default()
    };

    let member_groups = filter_users(members, &filter_config);
    for (title, members) in member_groups {
        println!("{}", title);
        println!("{:#?}", members);
    }
    return Ok(());
}
