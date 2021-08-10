use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Name(String);

impl From<String> for Name {
    fn from(name: String) -> Self {
        Self(name)
    }
}
