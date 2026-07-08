use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

use crate::{
    cli::{Cli, Commands, RenderTargetArg},
    config::Config,
    error::ElsewhereError,
    plan::{build_plan, print_plan},
    renderers::{self, RenderedPost},
    target::RenderTarget,
    workspace::{LoadedPost, load_post},
};

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { force } => init(force),
        Commands::Plan { json, post } => plan(post, json),
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

fn plan(post_path: PathBuf, json: bool) -> Result<()> {
    let loaded = load_post(&post_path)?;
    let plan = build_plan(&loaded, &post_path);

    if json {
        serde_json::to_writer_pretty(std::io::stdout(), &plan)?;
        println!();
    } else {
        print_plan(&plan);
    }

    Ok(())
}

fn render(target: RenderTargetArg, post_path: PathBuf) -> Result<()> {
    let loaded = load_post(&post_path)?;

    if let Some(target) = target.as_single_target() {
        render_one(&loaded, target)
    } else {
        render_all(&loaded)
    }
}

fn render_one(loaded: &LoadedPost, target: RenderTarget) -> Result<()> {
    let rendered = renderers::render(target, &loaded.post, &loaded.config.config)?;

    print_render_diagnostics(&rendered);
    println!("{}", rendered.body);

    Ok(())
}

fn render_all(loaded: &LoadedPost) -> Result<()> {
    for (index, target) in RenderTarget::all().iter().copied().enumerate() {
        let rendered = renderers::render(target, &loaded.post, &loaded.config.config)?;

        if index > 0 {
            println!();
        }

        println!("== {} ==", target.display_name());

        match rendered.max_chars {
            Some(max_chars) => {
                println!("Length: {} / {max_chars}", rendered.char_count);
            }
            None => {
                println!("Length: {}", rendered.char_count);
            }
        }

        for warning in &rendered.warnings {
            println!("{warning}");
        }

        println!();
        print!("{}", rendered.body);

        if !rendered.body.ends_with('\n') {
            println!();
        }
    }

    Ok(())
}

fn print_render_diagnostics(rendered: &RenderedPost) {
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

    for warning in &rendered.warnings {
        eprintln!("{warning}");
    }
}
