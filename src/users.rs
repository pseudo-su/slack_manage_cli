use clap::{ arg_enum };
use regex::Regex;

use prettytable::{Attr, Cell, Row, Table};
use prettytable::format::Alignment;

use crate::AppError;
use std::fmt::{Formatter, Display, Error};
use std::ops::Deref;

use crate::api_client::apis::users_api;
use crate::api_client::apis::configuration::Configuration;
use crate::api_client::models::{UsersListResponseBody, UsersListMember};

arg_enum!{
    #[derive(Debug)]
    pub enum SortUsersBy { NoSort, EmailDomain, Username }
}


impl From<crate::api_client::apis::Error<crate::api_client::apis::users_api::UsersListError>> for AppError {
    fn from(e: crate::api_client::apis::Error<crate::api_client::apis::users_api::UsersListError>) -> Self {
        println!("{}", e);
        return AppError {
            message: "Error fetching user list".to_owned(),
        };
    }
}

pub async fn fetch_users(client_config: &Configuration, token: &str, sort_by: Option<SortUsersBy>) -> Result<Vec<UsersListMember>, AppError> {
    let sort_by = sort_by.unwrap_or(SortUsersBy::NoSort);

    let list_resp = users_api::users_list(client_config, Some(token), Some(500), None, Some(true)).await?;

    let all_members = match list_resp {
        UsersListResponseBody {
            members,
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
    pub skip_full_members: bool,
    pub filters: Vec<UserFilter>,
}

impl UserFilterConfig {
    pub fn print_included(&self) -> String {
        format!("Included: bots={} restricted={} ultra_restricted={}, full_members={}",
            !self.skip_bots,
            !self.skip_restricted,
            !self.skip_ultra_restricted,
            !self.skip_full_members,
        )
    }
}

impl Display for UserFilterConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.filters.len() == 0 {
            write!(f, "No matchers")?;
        } else {
            write!(f, "Matchers:")?;
        }
        for filter in &self.filters {
            write!(f, " `{}`", filter)?;
        }
        writeln!(f, "")?;
        writeln!(f, "{}", self.print_included())?;
        Ok(())
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
    pub members: Vec<UsersListMember>,
}

impl Display for FilterMembersResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "Matched {} users of the {} searched (skipped {}).", self.meta.matched_count, self.meta.searched_count, self.meta.skipped_count)?;
        // TODO: make print_users use formatter instead of `println!().`
        print_users(self.meta.display_name.to_owned(), self.members.clone(), &PrintUsersConfig::default());
        Ok(())
    }
}

pub fn filter_members(members: Vec<UsersListMember>, filter_config: &UserFilterConfig) -> FilterMembersResult {
    let undeleted_members: Vec<UsersListMember> = members
        .into_iter()
        // Skip deleted users
        .filter(|m| match m {
            UsersListMember {
                deleted: false,
                ..
            } => true,
            _ => false,
        })
        .collect();
   let undeleted_members_count = undeleted_members.len(); 
   let valid_members: Vec<UsersListMember> = undeleted_members
        .into_iter()
        // Skip bots if flag is set
        .filter(|m| {
            if filter_config.skip_bots {
                match m {
                    UsersListMember {
                        is_bot: true, ..
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
                    UsersListMember {
                        is_restricted: Some(true),
                        is_ultra_restricted: Some(false),
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
                    UsersListMember {
                        is_ultra_restricted: Some(true),
                        ..
                    } => false,
                    _ => true,
                }
            } else {
                true
            }
        })
        // skip full members
        .filter(|m| {
            if filter_config.skip_full_members {
                match m {
                    UsersListMember {
                        is_bot: false,
                        is_restricted: Some(false),
                        is_ultra_restricted: Some(false),
                        ..
                    } => false,
                    _ => true,
                }
            } else {
                true
            }
        })
        .collect();

    let display_name = filter_config.to_string();

    let filtered_members: Vec<UsersListMember> = valid_members
        .iter()
        .filter(| member | {

            // make sure the user passes all filters
            filter_config.filters.iter().all(|filter| {
                match filter {
                    UserFilter{
                        filter_on: UserFilterOn::Email,
                        ..
                    } => {
                        // let member = member.to_owned();
                        let email = member.profile.email.clone();
                        if let Some(member_email) = email {
                            filter.should_match == filter.regex.is_match(member_email.as_str())
                        } else {
                            false
                        }
                    },
                    UserFilter{
                        filter_on: UserFilterOn::Username,
                        ..
                    } => {
                        if filter.should_match == filter.regex.is_match(member.name.clone().as_str()) {
                            true
                        } else {
                            false
                        }
                    },
                }
            })
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

fn user_email_domain(user: &UsersListMember) -> Option<String> {
    let rgx = Regex::new("@(.*)$").unwrap();
    // TODO: super hack because I don't understand lifetimes
    let mut superhack = String::from("");
    let caps = match user {
        UsersListMember{
            profile,
            ..
        } => {
            let p = profile.deref().clone();
            if let Some(email) = p.email.clone() {
                superhack = email.clone();
                let captures = rgx.captures(&superhack);

                captures
            } else {
                None
            }
        },
        _ => None,
    };
    if let Some(caps) = caps {
        return caps.get(1).map(|m| m.as_str().to_owned());
    }
    return None;
}

fn sort_users(members: Vec<UsersListMember>, sort_by: SortUsersBy) -> Vec<UsersListMember> {
    match sort_by {
        SortUsersBy::NoSort => members,
        SortUsersBy::Username => {
            let mut sorted = members.clone();
            sorted.sort_by(|a, b| {
                // let a_username = a.name.clone().unwrap_or("".to_owned());
                // let b_username = b.name.clone().unwrap_or("".to_owned());
                let a_username = a.name.clone();
                let b_username = b.name.clone();

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

fn prepare_output(members: Vec<UsersListMember>, print_config: &PrintUsersConfig) -> (Option<Vec<String>>, Vec<Vec<Option<String>>>) {
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
                UsersListMember { id, .. } => Some(id.to_owned()),
                _ => None,
            };
            row.push(id_str);
        }
        if print_config.user_name {
            let name_str = match member {
                UsersListMember {
                    name, ..
                } => Some(name.to_owned()),
                _ => None,
            };
            row.push(name_str);
        }
        if print_config.email {
            let email_str = match member {
                UsersListMember {
                    profile,
                    ..
                } => {
                    if let Some(email) = profile.email.clone() {
                        Some(email.to_owned())
                    } else {
                        None
                    }
                },
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

pub fn print_users(title: String, members: Vec<UsersListMember>, print_config: &PrintUsersConfig) {
    let title = if print_config.title {Some(title)} else {None};
    let (header_row, rows) = prepare_output(members, print_config);
    if print_config.csv {
        return print_users_csv(title, header_row, rows);
    } else {
        return print_users_table(title, header_row, rows);
    }
}
