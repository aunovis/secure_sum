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
    todo!()
}
