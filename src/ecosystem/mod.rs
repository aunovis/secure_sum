mod depfile;
mod rust;

pub(crate) use depfile::*;

pub(crate) enum Ecosystem {
    Rust,
}

impl Ecosystem {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Ecosystem::Rust => "rust",
        }
    }
}
