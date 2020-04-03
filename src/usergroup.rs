use regex::Regex;

pub enum Ident {
  Id(String),
  Handle(String),
  Name(String),
}

pub struct MembershipSelector {
  pub usergroup: Ident,
  pub selector: Regex,
}

impl std::str::FromStr for MembershipSelector {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                  .split(',')
                                  .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;
        let ident = Ident::Handle("str".to_owned());
        let selector = Regex::new("hello")?;
        Ok(MembershipSelector { usergroup: ident, selector })
    }
}
