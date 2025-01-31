use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Url(pub(crate) String);

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Url {
    fn from(value: String) -> Self {
        Url(value)
    }
}

impl From<&str> for Url {
    fn from(value: &str) -> Self {
        Url(value.to_owned())
    }
}

impl Url {
    pub(crate) fn str_without_protocol(&self) -> &str {
        self.0
            .trim_start_matches("http://")
            .trim_start_matches("https://")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_protocol_is_as_expected() {
        assert_eq!(
            Url("aunovis.de".to_string()).str_without_protocol(),
            "aunovis.de"
        );
        assert_eq!(
            Url("http://aunovis.de".to_string()).str_without_protocol(),
            "aunovis.de"
        );
        assert_eq!(
            Url("https://aunovis.de".to_string()).str_without_protocol(),
            "aunovis.de"
        );
    }
}
