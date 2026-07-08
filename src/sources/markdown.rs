use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::frontmatter::parse_markdown;
use crate::post::CanonicalPost;

pub fn read_markdown_post(path: &Path) -> Result<CanonicalPost> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read post file: {}", path.display()))?;

    let parsed = parse_markdown(&raw)?;
    let frontmatter = parsed.frontmatter;
    let body_markdown = parsed.body_markdown;

    let slug = frontmatter.slug.or_else(|| infer_slug_from_path(path));

    let first_paragraph = CanonicalPost::first_paragraph_from_markdown(&body_markdown);

    Ok(CanonicalPost {
        title: frontmatter.title,
        description: frontmatter.description,
        date: frontmatter.date,
        tags: frontmatter.tags,
        canonical_url: frontmatter.canonical_url,
        body_markdown,
        first_paragraph,
        slug,
        path: frontmatter.path,
        draft: frontmatter.draft,
        elsewhere: frontmatter.elsewhere,
    })
}

fn infer_slug_from_path(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(ToOwned::to_owned)
}
