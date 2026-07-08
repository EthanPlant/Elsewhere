use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::target::RenderTarget;

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
        /// Path to a Markdown post.
        post: PathBuf,

        /// Emit machine-readable JSON
        #[arg(short, long)]
        json: bool,
    },

    /// Render a post for a specific target.
    Render {
        /// Target platform or format.
        target: RenderTargetArg,

        /// Path to a Markdown post.
        post: PathBuf,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RenderTargetArg {
    Mastodon,
    Bluesky,
    Markdown,
    All,
}

impl RenderTargetArg {
    pub fn as_single_target(&self) -> Option<RenderTarget> {
        match self {
            Self::Mastodon => Some(RenderTarget::Mastodon),
            Self::Bluesky => Some(RenderTarget::Bluesky),
            Self::Markdown => Some(RenderTarget::Markdown),
            Self::All => None,
        }
    }
}
