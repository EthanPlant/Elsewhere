use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::{Cli, Commands, RenderTarget};
use crate::config::{Config, load_config};
use crate::error::ElsewhereError;
use crate::post::CanonicalPost;
use crate::sources::Source;
use crate::sources::generic_markdown::GenericMarkdownSource;

mod cli;
mod config;
mod error;
mod frontmatter;
mod post;
mod sources;

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
        toml::to_string_pretty(&config).context("failed to serialize default configuration")?;

    fs::write(&path, serialized).with_context(|| format!("failed to write {}", path.display()))?;

    println!("Created {}", path.display());
    println!("Edit this file to describe your static site.");

    Ok(())
}

fn plan(post: PathBuf) -> Result<()> {
    ensure_post_exists(&post)?;

    let loaded_config = load_config()?;

    let post = read_post_with_canonical_url(&loaded_config.config, &loaded_config.root_dir, &post)?;

    println!("Post");
    println!("Title: {}", post.title);
    println!(
        "Description: {}",
        post.description.as_deref().unwrap_or("not set")
    );
    println!(
        "Canonical URL: {}",
        post.canonical_url.as_deref().unwrap_or("not set")
    );

    if post.tags.is_empty() {
        println!("Tags: none");
    } else {
        println!("Tags: {}", post.tags.join(", "));
    }

    println!();
    println!("Available renders:");
    println!("- mastodon");
    println!("- bluesky");
    println!("- substack");

    Ok(())
}

fn render(target: RenderTarget, post: PathBuf) -> Result<()> {
    ensure_post_exists(&post)?;

    let loaded_config = load_config()?;

    let _post =
        read_post_with_canonical_url(&loaded_config.config, &loaded_config.root_dir, &post)?;

    anyhow::bail!(ElsewhereError::RendererNotImplemented {
        target: target.to_string(),
    });
}

fn read_post_with_canonical_url(
    config: &Config,
    site_root: &Path,
    post_path: &Path,
) -> Result<CanonicalPost> {
    let source = GenericMarkdownSource;
    let mut post = source.read_post(post_path)?;

    if post.canonical_url.is_none() {
        post.canonical_url = config.derive_canonical_url(site_root, post_path, &post);
    }

    Ok(post)
}

fn ensure_post_exists(path: &Path) -> Result<()> {
    if !path.is_file() {
        return Err(ElsewhereError::PostNotFound(path.to_path_buf()).into());
    }

    Ok(())
}
