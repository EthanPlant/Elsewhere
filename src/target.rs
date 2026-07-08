// src/target.rs

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderTarget {
    Mastodon,
    Bluesky,
    Substack,
}

impl RenderTarget {
    pub fn all() -> &'static [RenderTarget] {
        &[Self::Mastodon, Self::Bluesky, Self::Substack]
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Mastodon => "mastodon",
            Self::Bluesky => "bluesky",
            Self::Substack => "substack",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Mastodon => "Mastodon",
            Self::Bluesky => "Bluesky",
            Self::Substack => "Substack",
        }
    }

    pub fn is_long_form(self) -> bool {
        matches!(self, Self::Substack)
    }
}

impl fmt::Display for RenderTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
