use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::{config::SourceKind, target::RenderTarget};

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
        /// Source format to configure (defaults to `generic`)
        #[arg(long, value_enum, default_value = "generic")]
        source: InitSourceArg,

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum InitSourceArg {
    Generic,
    Zola,
}

impl From<InitSourceArg> for SourceKind {
    fn from(value: InitSourceArg) -> Self {
        match value {
            InitSourceArg::Generic => Self::Generic,
            InitSourceArg::Zola => Self::Zola,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RenderTargetArg {
    Mastodon,
    Bluesky,
    Markdown,
    Reddit,
    All,
}

impl RenderTargetArg {
    pub fn as_single_target(&self) -> Option<RenderTarget> {
        match self {
            Self::Mastodon => Some(RenderTarget::Mastodon),
            Self::Bluesky => Some(RenderTarget::Bluesky),
            Self::Markdown => Some(RenderTarget::Markdown),
            Self::Reddit => Some(RenderTarget::Reddit),
            Self::All => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::error::ErrorKind;

    use super::*;

    #[test]
    fn init_defaults_to_generic_source() {
        let cli = Cli::try_parse_from(["elsewhere", "init"]).unwrap();

        assert!(matches!(
            cli.command,
            Commands::Init {
                source: InitSourceArg::Generic,
                force: false,
            }
        ));
    }

    #[test]
    fn init_accepts_explicit_generic_source() {
        let cli = Cli::try_parse_from(["elsewhere", "init", "--source", "generic"]).unwrap();

        assert!(matches!(
            cli.command,
            Commands::Init {
                source: InitSourceArg::Generic,
                force: false,
            }
        ));
    }

    #[test]
    fn init_accepts_zola_source() {
        let cli = Cli::try_parse_from(["elsewhere", "init", "--source", "zola"]).unwrap();

        assert!(matches!(
            cli.command,
            Commands::Init {
                source: InitSourceArg::Zola,
                force: false,
            }
        ));
    }

    #[test]
    fn init_rejects_unknown_source() {
        let error = Cli::try_parse_from(["elsewhere", "init", "--source", "hugo"]).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::InvalidValue);

        let message = error.to_string();
        assert!(message.contains("hugo"));
        assert!(message.contains("generic"));
        assert!(message.contains("zola"));
    }
}
