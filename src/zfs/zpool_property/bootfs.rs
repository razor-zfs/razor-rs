use regex::Regex;
use std::fmt;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bootfs(String);

impl fmt::Display for Bootfs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Bootfs {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"(^[a-zA-Z]+[\\/+])([a-zA-Z]*\\/?)*|^-$"#).unwrap();

        if re.is_match(s) {
            return Ok(Self(s.to_string()));
        }

        Err(super::InvalidProperty::invalid_value(s))
    }
}

impl ops::Deref for Bootfs {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
