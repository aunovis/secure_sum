use std::fmt::Display;

use crate::error::Error;

pub(crate) enum Input {
    Url(String),
}

impl Input {
    pub(crate) fn parse(path: String) -> Result<Self, Error> {
        if is_url(&path) {
            return Ok(Self::Url(path));
        }
        let message = format!("Unable to understand {path}");
        Err(Error::Input(message))
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Url(url) => write!(f, "URL: {url}"),
        }
    }
}

fn is_url(str: &str) -> bool {
    log::debug!("Testing if {str} is a URL");
    str.starts_with("https://") || str.starts_with("http://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocols_mark_urls() {
        assert!(is_url("https://quettapano"));
        assert!(is_url("http://andolama/mirquet"));
        assert!(!is_url("cimrinora/arquenie"));
    }
}
