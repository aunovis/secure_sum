mod depfile;
mod rust;

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
