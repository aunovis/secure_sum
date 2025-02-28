use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub(crate) enum ProbeName {
    archived,
    blocksDeleteOnBranches,
    codeApproved,
}

impl Display for ProbeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = serde_json::to_string(self)
            .unwrap_or_else(|_| String::new())
            .trim_matches('"')
            .to_string();
        write!(f, "{name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_is_as_expected() {
        assert_eq!(ProbeName::archived.to_string(), "archived");
    }
}
