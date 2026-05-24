use std::{fmt, path::PathBuf};

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "elsewhere",
    version,
    about = "Render static-site posts elsewhere.",
    long_about = "Elsewhere treats your static site as the canonical source and renders platform-specific copies for other places."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    /// Create an elsewhere.toml configuration file.
    Init {
        /// Overwrite an existing elsewhere.toml file.
        #[arg(short, long)]
        force: bool,
    },
    /// Show what Elsewhere would render for a post.
    Plan {
        /// path to a Markdown post.
        post: PathBuf,
    },
    ///Render a post for a specific target.
    Render {
        /// Target platform or format.
        target: RenderTarget,

        /// Path to a markdown post.
        post: PathBuf,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RenderTarget {
    Mastodon,
    Bluesky,
    Substack,
}

impl fmt::Display for RenderTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Mastodon => "mastodon",
            Self::Bluesky => "bluesky",
            Self::Substack => "substack",
        };

        write!(f, "{value}")
    }
}
