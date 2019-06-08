use clap::{ Arg, ArgGroup, ArgMatches, App, AppSettings, SubCommand, value_t };
use regex::Regex;

mod app_error;
use app_error::AppError;
mod users;
use users::{fetch_users, filter_users, print_users, UserFilterConfig, PrintUsersConfig };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), AppError> {
    let token_arg = Arg::with_name("oauth-access-token")
        .short("token")
        .long("oauth-access-token")
        .value_name("token")
        .help("OAuth Access Token")
        .takes_value(true);

    let token_file_arg = Arg::with_name("oauth-access-token-file")
        .short("token-file")
        .long("oauth-access-token-file")
        .value_name("token-file")
        .help("Path to file containing access token")
        .takes_value(true);

    let token_group = ArgGroup::with_name("baz")
        .args(&["oauth-access-token", "oauth-access-token-file"])
        .required(true);

    let email_filter_arg = Arg::with_name("email-filter")
        .takes_value(true)
        .long("email-filter");

    let matches = App::new("slack-manage")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(VERSION)
        .about("Various tools for managing a slack server")
        .subcommand(SubCommand::with_name("list-members")
            .about("controls testing features")
            .arg(token_arg.clone())
            .arg(token_file_arg.clone())
            .group(token_group.clone())
            .arg(email_filter_arg.clone())
        )
        .subcommand(SubCommand::with_name("bulk-invite")
            .about("controls testing features")
            .arg(Arg::with_name("channel")
                .required(true)
                .index(1))
            .arg(token_arg.clone())
            .arg(token_file_arg.clone())
            .group(token_group.clone())
            .arg(email_filter_arg.clone())
        )
        .subcommand(SubCommand::with_name("usergroups-update")
            .about("controls testing features")
            // ArgGroup one of (id|name|handle)
            .arg(Arg::with_name("usergroup")
                .required(true)
                .index(1))
            .arg(token_arg.clone())
            .arg(token_file_arg.clone())
            .group(token_group.clone())
            .arg(email_filter_arg.clone())
        )
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("list-members") {
        return list_members(matches);
    }

    if let Some(matches) = matches.subcommand_matches("bulk-invite") {
        return invite_members_to_channel(matches);
    }

    if let Some(matches) = matches.subcommand_matches("usergroups-update") {
        return update_usergroup_members(matches);
    }

    Ok(())
}

fn list_members(matches: &ArgMatches) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let oauth_api_key = matches.value_of("oauth-access-token").unwrap();
    let email_filter: Option<Regex> = value_t!(matches.value_of("email-filter"), Regex).ok();

    let members = fetch_users(&client, oauth_api_key)?;
    let filter_config = UserFilterConfig{
        // filter_email_domain: matches.value_of("email-match").unwrap_or(vec![]),
        email_filter: email_filter,
        ..Default::default()
    };

    let members = filter_users(members, &filter_config);
    let print_config = PrintUsersConfig{ ..Default::default() };
    print_users(members, &print_config);
    return Ok(());
}

fn invite_members_to_channel(matches: &ArgMatches) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let oauth_api_key = matches.value_of("oauth-access-token").unwrap();
    // let email_filter: Option<Regex> = value_t!(matches.value_of("email-filter"), Regex).ok();
    let members = fetch_users(&client, oauth_api_key)?;
    println!("{:#?}", members);
    return Ok(());
}

fn update_usergroup_members(matches: &ArgMatches) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let oauth_api_key = matches.value_of("oauth-access-token").unwrap();
    // let email_filter: Option<Regex> = value_t!(matches.value_of("email-filter"), Regex).ok();
    let members = fetch_users(&client, oauth_api_key)?;
    println!("{:#?}", members);
    return Ok(());
}
