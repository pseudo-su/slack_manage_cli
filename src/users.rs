use clap::{ arg_enum };
use regex::Regex;
use slack_api::User;
use slack_api::UserProfile;
// use slack_api::usergroups_users;
use slack_api::users;

use prettytable::{Attr, Cell, Row, Table};
use prettytable::format::Alignment;

use crate::AppError;
use std::fmt::{Formatter, Display, Error};

#[derive(Debug)]
pub struct RegexGroupFilter {
  pub group_name: Option<String>,
  pub regex: Regex,
}

arg_enum!{
    #[derive(Debug)]
    pub enum SortUsersBy { NoSort, EmailDomain, Username }
}

pub fn fetch_users(client: &reqwest::Client, token: &str, sort_by: Option<SortUsersBy>) -> Result<Vec<User>, AppError> {
    let sort_by = sort_by.unwrap_or(SortUsersBy::NoSort);
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
    pub username_filter: Vec<RegexGroupFilter>,
    pub email_filter: Vec<RegexGroupFilter>,
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
    fn from(e: users::ListError<reqwest::Error>) -> Self {
        println!("{}", e);
        return AppError {
            message: "Error fetching user list".to_owned(),
        };
    }
}

pub struct FilterUsersResultMeta {
    skipped_users_count: usize,
    searched_users_count: usize,
    matched_users_count: usize,
}

#[derive(Clone)]
pub struct FilteredGroup {
    pub group_name: Option<String>,
    // TODO: rename to display_name
    pub display_name: String,
    pub filter: Option<String>,
    pub members: Vec<User>
}

impl Display for FilteredGroup {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        // TODO: make print_users use formatter instead of `println!().`
        print_users(self.display_name.to_owned(), self.members.clone(), &PrintUsersConfig::default());
        Ok(())
    }
}

pub struct FilterUsersResult {
    pub meta: FilterUsersResultMeta,
    pub groups: Vec<FilteredGroup>,
}

impl Display for FilterUsersResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "Matched {} users of the {} searched (skipped {}).", self.meta.matched_users_count, self.meta.searched_users_count, self.meta.skipped_users_count)?;
        for group in self.groups.clone() {
            write!(f, "{}", group)?;
        }
        Ok(())
    }
}

pub fn filter_users(members: Vec<User>, filter_config: &UserFilterConfig) -> FilterUsersResult {
    let members_count = members.len();
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

    let mut filtered_groups: Vec<FilteredGroup> = vec![];
    if filter_config.email_filter.len() < 1 && filter_config.username_filter.len() < 1 {
        let group = FilteredGroup{
            group_name: None,
            display_name: "NO FILTER".to_owned(),
            filter: None,
            members: valid_members.clone(),
        };
        filtered_groups.push(group);
    }

    for filter in &filter_config.email_filter {
        let members = valid_members.clone().into_iter()
        .filter(|m| {
            match m {
                User {
                    profile:
                        Some(UserProfile {
                            email: Some(email), ..
                        }),
                    ..
                } if filter.regex.is_match(email) => true,
                _ => false,
            }
        })
        .collect();
        let regex_str = filter.regex.as_str();
        let group_name = filter.group_name.to_owned();
        let group_label = group_name.to_owned()
            .map_or("".to_owned(), |g| format!("Group name=\"{}\" ", g.as_str()));
        let display_name = format!("Filtered by email: {}Regex=\"{}\"", group_label, regex_str).to_owned();
        let group = FilteredGroup{
            group_name,
            display_name,
            filter: Some(format!("{}", filter.regex.as_str())),
            members,
        };
        filtered_groups.push(group);
    }

    for filter in &filter_config.username_filter {
        let members = valid_members.clone().into_iter()
        .filter(|m| {
            match m {
                User {
                    name: Some(name),
                    ..
                } => filter.regex.is_match(name),
                _ => false,
            }
        })
        .collect();
        let regex_str = filter.regex.as_str();
        let group_name = filter.group_name.to_owned();
        let group_label = group_name.to_owned()
            .map_or("".to_owned(), |g| format!("Group name=\"{}\" ", g.as_str()));
        let display_name = format!("Filtered by username: {}Regex=\"{}\"", group_label, regex_str).to_owned();
        let group = FilteredGroup{
            group_name,
            display_name,
            filter: Some(format!("{}", filter.regex.as_str())),
            members,
        };
        filtered_groups.push(group);
    }
    let matched_users_count = filtered_groups.iter().fold( 0, |prev, group| prev + group.members.len());
    let result = FilterUsersResult{
        meta: FilterUsersResultMeta{
            skipped_users_count: members_count - valid_members.len(),
            searched_users_count: valid_members.len(),
            matched_users_count,
        },
        
        groups: filtered_groups,
    };

    result
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
        SortUsersBy::NoSort => members,
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
        table.set_titles(Row::new(vec![
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
