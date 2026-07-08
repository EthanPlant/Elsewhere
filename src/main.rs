use anyhow::Result;
use clap::Parser;

use crate::{app::run, cli::Cli};

mod app;
mod cli;
mod config;
mod error;
mod frontmatter;
mod post;
mod renderers;
mod sources;
mod target;
mod templates;
mod workspace;

fn main() -> Result<()> {
    run(Cli::parse())
}
