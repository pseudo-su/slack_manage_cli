use clap::{ arg_enum };
use regex::Regex;
use slack_api::sync::User;
use slack_api::sync::UserProfile;
// use slack_api::sync::usergroups_users;
use slack_api::sync::users;

use prettytable::{Attr, Cell, Row, Table};
use prettytable::format::Alignment;

use crate::AppError;
use std::fmt::{Formatter, Display, Error};

arg_enum!{
    #[derive(Debug)]
    pub enum SortUsersBy { NoSort, EmailDomain, Username }
}

pub fn fetch_users(client: &reqwest::blocking::Client, token: &str, sort_by: Option<SortUsersBy>) -> Result<Vec<User>, AppError> {
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

#[derive(Clone)]
pub enum UserFilterOn {
    Username,
    Email,
}

impl Display for UserFilterOn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {        
        match self {
            UserFilterOn::Username => write!(f, "Username")?,
            UserFilterOn::Email => write!(f, "Email")?,
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct UserFilter {
    pub filter_on: UserFilterOn,
    pub regex: Regex,
    pub should_match: bool,
}

impl Display for UserFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}={}", self.filter_on, self.regex.as_str())?;
        Ok(())
    }
}

pub struct UserFilterConfig {
    pub skip_bots: bool,
    pub skip_restricted: bool,
    pub skip_ultra_restricted: bool,
    pub filters: Vec<UserFilter>,
}

impl Default for UserFilterConfig {
    fn default() -> Self {
        Self {
            skip_bots: true,
            skip_restricted: true,
            skip_ultra_restricted: true,
            filters: vec![],
        }
    }
}

impl Display for UserFilterConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // write!(f, "Skip={}", );
        if self.filters.len() == 0 {
            write!(f, "No filters")?;
        } else {
            write!(f, "Filters:")?;
        }
        for filter in &self.filters {
            write!(f, " `{}`", filter)?;
        }
        Ok(())
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

pub struct FilterMembersResultMeta {
    pub skipped_count: usize,
    pub searched_count: usize,
    pub matched_count: usize,
    pub display_name: String,
}

pub struct FilterMembersResult {
    pub meta: FilterMembersResultMeta,
    pub members: Vec<User>,
}

impl Display for FilterMembersResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "Matched {} users of the {} searched (skipped {}).", self.meta.matched_count, self.meta.searched_count, self.meta.skipped_count)?;
        // TODO: make print_users use formatter instead of `println!().`
        print_users(self.meta.display_name.to_owned(), self.members.clone(), &PrintUsersConfig::default());
        Ok(())
    }
}

pub fn filter_members(members: Vec<User>, filter_config: &UserFilterConfig) -> FilterMembersResult {
    let undeleted_members: Vec<User> = members
        .into_iter()
        // Skip deleted users
        .filter(|m| match m {
            User {
                deleted: Some(false),
                ..
            } => true,
            _ => false,
        })
        .collect();
   let undeleted_members_count = undeleted_members.len(); 
   let valid_members: Vec<User> = undeleted_members
        .into_iter()
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
            _ => {
                false
            },
        })
        .collect();

    let display_name = filter_config.to_string();

    let filtered_members: Vec<User> = valid_members
        .iter()
        .filter(| &member | {
            if let User {
                name: Some(member_username),
                profile:
                    Some(UserProfile {
                        email: Some(member_email),
                        ..
                    }),
                ..
            } = member {
                // make sure the user passes all filters
                filter_config.filters.iter().all(|filter| {
                    match filter {
                        UserFilter{
                            filter_on: UserFilterOn::Email,
                            ..
                        } => filter.regex.is_match(member_email.as_str()) == filter.should_match,
                        UserFilter{
                            filter_on: UserFilterOn::Username,
                            ..
                        } => filter.regex.is_match(member_username.as_str()) == filter.should_match,
                    }
                })
            } else { false }
        })
        .map(|m| m.to_owned())
        .collect();

    let result = FilterMembersResult{
        meta: FilterMembersResultMeta{
            skipped_count: undeleted_members_count - valid_members.len(),
            
            searched_count: valid_members.len(),
            matched_count: filtered_members.len(),
            display_name,
        },
        
        members: filtered_members,
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
