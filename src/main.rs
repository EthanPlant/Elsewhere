use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::{Cli, Commands, RenderTarget};
use crate::config::{Config, LoadedConfig, SourceKind, load_config_for_post};
use crate::error::ElsewhereError;
use crate::post::CanonicalPost;
use crate::sources::Source;
use crate::sources::generic::GenericSource;
use crate::sources::zola::ZolaSource;

mod cli;
mod config;
mod error;
mod frontmatter;
mod post;
mod renderers;
mod sources;
mod templates;

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

    let loaded_config = load_config_for_post(&post)?;

    let post = read_post_with_canonical_url(&loaded_config, &loaded_config.root_dir, &post)?;

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

    if post.draft {
        println!("Warning: this post is marked as draft.");
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

    let loaded_config = load_config_for_post(&post)?;

    let post = read_post_with_canonical_url(&loaded_config, &loaded_config.root_dir, &post)?;

    let rendered = renderers::render(&target, &post, &loaded_config.config)?;

    match rendered.max_chars {
        Some(max_chars) => {
            eprintln!(
                "{} render: {} / {max_chars} characters",
                rendered.target, rendered.char_count
            );
        }
        None => eprintln!(
            "{} render {} characters",
            rendered.target, rendered.char_count
        ),
    }

    for warning in rendered.warnings {
        eprintln!("{warning}");
    }

    println!("{}", rendered.body);

    Ok(())
}

fn read_post_with_canonical_url(
    loaded_config: &LoadedConfig,
    site_root: &Path,
    post_path: &Path,
) -> Result<CanonicalPost> {
    let site_url = effective_site_url(loaded_config)?;
    let resolved_post_path = resolve_post_path(&loaded_config.root_dir, post_path);
    let mut post = match loaded_config.config.source {
        SourceKind::Generic => {
            let source = GenericSource;
            source.read_post(&resolved_post_path)?
        }
        SourceKind::Zola => {
            let source = ZolaSource::new(loaded_config.root_dir.clone());
            source.read_post(&resolved_post_path)?
        }
    };

    if post.canonical_url.is_none() {
        post.canonical_url = loaded_config.config.derive_canonical_url(
            &site_url,
            site_root,
            &resolved_post_path,
            &post,
        );
    }

    Ok(post)
}

fn effective_site_url(loaded_config: &LoadedConfig) -> Result<String> {
    if let Some(site_url) = loaded_config.config.site_url.clone() {
        return Ok(site_url);
    }

    match loaded_config.config.source {
        SourceKind::Zola => {
            let source = ZolaSource::new(loaded_config.root_dir.clone());
            let zola_config = source.read_config()?;
            Ok(zola_config.base_url)
        }
        SourceKind::Generic => Err(ElsewhereError::SiteUrlNotConfigured.into()),
    }
}

fn resolve_post_path(site_root: &Path, post_path: &Path) -> PathBuf {
    if post_path.is_absolute() {
        post_path.to_path_buf()
    } else {
        site_root.join(post_path)
    }
}

fn ensure_post_exists(path: &Path) -> Result<()> {
    if !path.is_file() {
        return Err(ElsewhereError::PostNotFound(path.to_path_buf()).into());
    }

    Ok(())
}
