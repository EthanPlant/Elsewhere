mod cli;
mod config;
mod error;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::{Cli, Commands, RenderTarget};
use crate::config::Config;
use crate::error::ElsewhereError;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => init(force),
        Commands::Plan { post } => plan(post),
        Commands::Render { target, post } => render(target, post),
    }
}

fn init(force: bool) -> Result<()> {
    let path = PathBuf::from("elsewhere.toml");

    if path.exists() && !force {
        return Err(ElsewhereError::ConfigExists(path).into());
    }

    let config = Config::default();
    let serialized =
        toml::to_string_pretty(&config).context("failed to serialize starter configuration")?;

    fs::write(&path, serialized).with_context(|| format!("failed to write {}", path.display()))?;

    println!("Created {}", path.display());
    println!("Edit this file to describe your static site.");

    Ok(())
}

fn plan(post: PathBuf) -> Result<()> {
    ensure_post_exists(&post)?;

    println!("Elsewhere plan");
    println!();
    println!("Canonical");
    println!("  Post:   {}", post.display());
    println!("  Status: placeholder");
    println!();
    println!("Available renders:");
    println!("  - mastodon");
    println!("  - bluesky");
    println!("  - substack");
    println!();
    println!("Next:");
    println!("  elsewhere render mastodon {}", post.display());

    Ok(())
}

fn render(_target: RenderTarget, post: PathBuf) -> Result<()> {
    ensure_post_exists(&post)?;
    println!("Unimplemented");

    Ok(())
}

fn ensure_post_exists(path: &Path) -> Result<()> {
    if !path.is_file() {
        return Err(ElsewhereError::PostNotFound(path.to_path_buf()).into());
    }

    Ok(())
}
