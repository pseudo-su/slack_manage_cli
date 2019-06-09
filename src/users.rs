use clap::{ arg_enum };
use regex::Regex;
use slack_api::User;
use slack_api::UserProfile;
// use slack_api::usergroups_users;
use slack_api::users;

use prettytable::{Attr, Cell, Row, Table};
use prettytable::format::Alignment;

use crate::AppError;

arg_enum!{
    #[derive(Debug)]
    pub enum SortUsersBy { None, EmailDomain, Username }
}

pub fn fetch_users(client: &reqwest::Client, token: &str, sort_by: Option<SortUsersBy>) -> Result<Vec<User>, AppError> {
    let sort_by = sort_by.unwrap_or(SortUsersBy::None);
    let request = users::ListRequest { presence: None };
    let list_resp = users::list(client, token, &request)?;
    let all_members = match list_resp {
        users::ListResponse {
            members: Some(members),
            ..
        } => Ok(members),
        _ => Err(AppError {
            message: "no members found".to_owned(),
        }),
    }?;

    let members = sort_users(all_members, sort_by);
    Ok(members)
}

pub struct UserFilterConfig {
    pub skip_bots: bool,
    pub skip_restricted: bool,
    pub skip_ultra_restricted: bool,
    pub username_filter: Vec<Regex>,
    pub email_filter: Vec<Regex>,
}

impl Default for UserFilterConfig {
    fn default() -> Self {
        Self {
            skip_bots: true,
            skip_restricted: true,
            skip_ultra_restricted: true,
            username_filter: vec![],
            email_filter: vec![],
        }
    }
}

impl From<users::ListError<reqwest::Error>> for AppError {
    fn from(_: users::ListError<reqwest::Error>) -> Self {
        return AppError {
            message: "Error fetching user list".to_owned(),
        };
    }
}


pub fn filter_users(members: Vec<User>, filter_config: &UserFilterConfig) -> Vec<(String, Vec<User>)> {
    let valid_members: Vec<User> = members
        .into_iter()
        // Skip deleted users
        .filter(|m| match m {
            User {
                deleted: Some(false),
                ..
            } => true,
            _ => false,
        })
        // Skip bots if flag is set
        .filter(|m| {
            if filter_config.skip_bots {
                match m {
                    User {
                        is_bot: Some(true), ..
                    } => false,
                    _ => true,
                }
            } else {
                true
            }
        })
        // Skip guests
        .filter(|m| {
            if filter_config.skip_restricted {
                match m {
                    User {
                        is_restricted: Some(true),
                        ..
                    } => false,
                    _ => true,
                }
            } else {
                true
            }
        })
        // skip single channel guests
        .filter(|m| {
            if filter_config.skip_ultra_restricted {
                match m {
                    User {
                        is_ultra_restricted: Some(true),
                        ..
                    } => false,
                    _ => true,
                }
            } else {
                true
            }
        })
        .filter(|m| match m {
            User {
                id: Some(_),
                name: Some(_),
                profile: Some(UserProfile { email: Some(_), .. }),
                ..
            } => true,
            _ => false,
        })
        .collect();

    let mut filtered_groups: Vec<(String, Vec<User>)> = vec![];
    if filter_config.email_filter.len() < 1 && filter_config.username_filter.len() < 1 {
        let group = ("NO FILTER".to_owned(), valid_members.clone());
        filtered_groups.push(group);
    }

    for regex in &filter_config.email_filter {
        let members = valid_members.clone().into_iter()
        .filter(|m| {
            match m {
                User {
                    profile:
                        Some(UserProfile {
                            email: Some(email), ..
                        }),
                    ..
                } if regex.find(email).is_some() => true,
                _ => false,
            }
        })
        .collect();
        let group = (format!("EMAIL REGEX: \"{}\"", regex).to_owned(), members);
        filtered_groups.push(group);
    }

    for regex in &filter_config.username_filter {
        let members = valid_members.clone().into_iter()
        .filter(|m| {
            match m {
                User {
                    name: Some(name),
                    ..
                } if regex.find(name).is_some() => true,
                _ => false,
            }
        })
        .collect();
        let group = (format!("USERNAME REGEX: \"{}\"", regex).to_owned(), members);
        filtered_groups.push(group);
    }

    filtered_groups
}

pub struct PrintUsersConfig {
    pub csv: bool,
    pub title: bool,
    pub header: bool,
    pub count: bool,
    pub user_id: bool,
    pub user_name: bool,
    pub email: bool,
}

impl Default for PrintUsersConfig {
    fn default() -> Self {
        Self {
            csv: false,
            title: true,
            header: true,
            count: true,
            user_id: true,
            user_name: true,
            email: true,
        }
    }
}

fn user_email_domain(user: &User) -> Option<String> {
    let rgx = Regex::new("@(.*)$").unwrap();
    let caps = match user {
        User{ profile: Some(UserProfile{email: Some(email), ..}), ..} => rgx.captures(email),
        _ => None,
    };
    if let Some(caps) = caps {
        return caps.get(1).map(|m| m.as_str().to_owned());
    }
    return None;
}

fn sort_users(members: Vec<User>, sort_by: SortUsersBy) -> Vec<User> {
    match sort_by {
        SortUsersBy::None => members,
        SortUsersBy::Username => {
            let mut sorted = members.clone();
            sorted.sort_by(|a, b| {
                let a_username = a.name.clone().unwrap_or("".to_owned());
                let b_username = b.name.clone().unwrap_or("".to_owned());

                a_username.cmp(&b_username)
            });
            sorted
        },
        SortUsersBy::EmailDomain => {
            let mut sorted = members.clone();
            sorted.sort_by(|a, b| {
                let a_email = user_email_domain(a).unwrap_or("".to_owned());
                let b_email = user_email_domain(b).unwrap_or("".to_owned());

                a_email.cmp(&b_email)
            });
            sorted
        },
    }
}

fn prepare_output(members: Vec<User>, print_config: &PrintUsersConfig) -> (Option<Vec<String>>, Vec<Vec<Option<String>>>) {
    // Assemble the header
    let mut header_row = None;
    if print_config.header {
        let mut header = vec![];
        if print_config.count {
            header.push("Count".to_owned());
        }
        if print_config.user_id {
            header.push("User ID".to_owned());
        }
        if print_config.user_name {
            header.push("User name".to_owned());
        }
        if print_config.email {
            header.push("Email".to_owned());
        }
        header_row = Some(header);
    }

    // Assemble the rows
    let mut rows = vec![];
    for (i, member) in members.iter().enumerate() {
        let mut row = vec![];

        if print_config.count {
            row.push(Some(format!("{}", i + 1)));
        }
        if print_config.user_id {
            let id_str = match member {
                User { id: Some(id), .. } => Some(id.to_owned()),
                _ => None,
            };
            row.push(id_str);
        }
        if print_config.user_name {
            let name_str = match member {
                User {
                    name: Some(name), ..
                } => Some(name.to_owned()),
                _ => None,
            };
            row.push(name_str);
        }
        if print_config.email {
            let email_str = match member {
                User {
                    profile:
                        Some(UserProfile {
                            email: Some(email), ..
                        }),
                    ..
                } => Some(email.to_owned()),
                _ => None,
            };
            row.push(email_str);
        }
        rows.push(row);
    }
    return (header_row, rows);
}

fn print_users_table(title: Option<String>, header_row: Option<Vec<String>>, rows: Vec<Vec<Option<String>>>) {
    let mut table = Table::new();
    let col_num = rows.first().map(|r| r.len()).unwrap_or(0);

    table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    if let Some(title) = title {
        table.add_row(Row::new(vec![
            Cell::new_align(title.as_ref(), Alignment::CENTER)
                .with_style(Attr::Bold)
                .with_hspan(col_num)
        ]));
    }

    if let Some(header_row) = header_row {
        let header_cells = header_row.iter().map(|s| Cell::new(s).with_style(Attr::Bold)).collect();
        table.add_row(Row::new(header_cells));
    }

    for row in rows {
        let cells = row.iter().map(|s| {
            let val: String = s.clone().unwrap_or("-".to_owned());
            Cell::new(val.as_str())
        }).collect();
        table.add_row(cells);
    }

    // Print the table to stdout
    table.printstd();
    println!("");
}

fn print_users_csv(_: Option<String>, header_row: Option<Vec<String>>, rows: Vec<Vec<Option<String>>>) {
    if let Some(header_row) = header_row {
        println!("{}", header_row.join(","));
    }
    for row in rows {
        let cells: Vec<String> = row.iter().map(|s| s.clone().unwrap_or("".to_owned())).collect();
        println!("{}", cells.join(","));
    }
}

pub fn print_users(title: String, members: Vec<User>, print_config: &PrintUsersConfig) {
    let title = if print_config.title {Some(title)} else {None};
    let (header_row, rows) = prepare_output(members, print_config);
    if print_config.csv {
        return print_users_csv(title, header_row, rows);
    } else {
        return print_users_table(title, header_row, rows);
    }
}
