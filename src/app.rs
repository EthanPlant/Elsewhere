use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

use crate::{
    cli::{Cli, Commands, RenderTargetArg},
    config::Config,
    error::ElsewhereError,
    renderers,
    workspace::load_post,
};

pub fn run(cli: Cli) -> Result<()> {
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

    let config = Config::starter();
    let serialized =
        toml::to_string_pretty(&config).context("failed to serialize starter configuration")?;

    fs::write(&path, serialized).with_context(|| format!("failed to write {}", path.display()))?;

    println!("Created {}", path.display());
    println!("Edit this file to describe your static site.");

    Ok(())
}

fn plan(post_path: PathBuf) -> Result<()> {
    let loaded = load_post(&post_path)?;
    let post = loaded.post;

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

fn render(target: RenderTargetArg, post_path: PathBuf) -> Result<()> {
    let loaded = load_post(&post_path)?;
    let target = target
        .as_single_target()
        .expect("render all is currently unsupported");
    let rendered = renderers::render(target, &loaded.post, &loaded.config.config)?;

    match rendered.max_chars {
        Some(max_chars) => {
            eprintln!(
                "{} render: {} / {max_chars} characters",
                rendered.target, rendered.char_count
            );
        }
        None => {
            eprintln!(
                "{} render: {} characters",
                rendered.target, rendered.char_count
            );
        }
    }

    for warning in rendered.warnings {
        eprintln!("{warning}");
    }

    println!("{}", rendered.body);

    Ok(())
}
