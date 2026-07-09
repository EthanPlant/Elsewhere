use std::str::FromStr;

use anyhow::Result;

use crate::{
    config::RedditPostKind,
    error::ElsewhereError,
    post::{ElsewhereFrontMatter, ElsewhereTargetOverride, RedditTargetOverride},
};

#[derive(Debug, Clone)]
pub struct ParsedMarkdown {
    pub frontmatter: ParsedFrontMatter,
    pub body_markdown: String,
}

#[derive(Debug, Clone)]
pub struct ParsedFrontMatter {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub canonical_url: Option<String>,
    pub slug: Option<String>,
    pub path: Option<String>,
    pub draft: bool,
    pub elsewhere: Option<ElsewhereFrontMatter>,
}

pub fn parse_markdown(input: &str) -> Result<ParsedMarkdown> {
    let mut lines = input.lines();

    let Some(first_line) = lines.next() else {
        return Err(ElsewhereError::MissingFrontMatter.into());
    };

    let delimiter = match first_line.trim_end_matches('\r') {
        "+++" => "+++",
        "---" => return Err(ElsewhereError::UnsupportedYamlFrontMatter.into()),
        _ => return Err(ElsewhereError::MissingFrontMatter.into()),
    };

    let mut frontmatter_lines = Vec::new();
    let mut body_lines = Vec::new();
    let mut found_closing_delimiter = false;

    for line in lines {
        if !found_closing_delimiter && line.trim_end_matches('\r') == delimiter {
            found_closing_delimiter = true;
            continue;
        }

        if found_closing_delimiter {
            body_lines.push(line);
        } else {
            frontmatter_lines.push(line);
        }
    }

    if !found_closing_delimiter {
        return Err(ElsewhereError::UnclosedFrontMatter(delimiter).into());
    }

    let frontmatter_raw = frontmatter_lines.join("\n");
    let body_markdown = body_lines.join("\n").trim_start().to_string();

    let frontmatter = parse_toml_frontmatter(&frontmatter_raw)?;

    Ok(ParsedMarkdown {
        frontmatter,
        body_markdown,
    })
}

fn parse_toml_frontmatter(input: &str) -> Result<ParsedFrontMatter> {
    let value: toml::Value = toml::from_str(input)?;
    let Some(table) = value.as_table() else {
        return Err(ElsewhereError::InvalidFrontMatterRoot.into());
    };

    Ok(ParsedFrontMatter {
        title: required_string(table, "title")?,
        description: optional_string(table, "description")?,
        date: optional_string_or_datetime(table, "date")?,
        tags: optional_tags(table)?,
        canonical_url: optional_string(table, "canonical_url")?,
        slug: optional_string(table, "slug")?,
        path: optional_string(table, "path")?,
        draft: optional_bool(table, "draft")?.unwrap_or(false),
        elsewhere: optional_elsewhere_frontmatter(table)?,
    })
}

fn required_string(table: &toml::Table, field: &'static str) -> Result<String> {
    let Some(value) = table.get(field) else {
        return Err(ElsewhereError::MissingRequiredField(field).into());
    };

    let Some(value) = value.as_str() else {
        return Err(ElsewhereError::InvalidFrontMatterField {
            field,
            expected: "a string",
        }
        .into());
    };

    Ok(value.to_string())
}

fn optional_string(table: &toml::Table, field: &'static str) -> Result<Option<String>> {
    let Some(value) = table.get(field) else {
        return Ok(None);
    };

    let Some(value) = value.as_str() else {
        return Err(ElsewhereError::InvalidFrontMatterField {
            field,
            expected: "a string",
        }
        .into());
    };

    Ok(Some(value.to_string()))
}

fn optional_bool(table: &toml::Table, field: &'static str) -> Result<Option<bool>> {
    let Some(value) = table.get(field) else {
        return Ok(None);
    };

    let Some(value) = value.as_bool() else {
        return Err(ElsewhereError::InvalidFrontMatterField {
            field,
            expected: "a boolean",
        }
        .into());
    };

    Ok(Some(value))
}

fn optional_string_or_datetime(table: &toml::Table, field: &'static str) -> Result<Option<String>> {
    let Some(value) = table.get(field) else {
        return Ok(None);
    };

    if let Some(value) = value.as_str() {
        return Ok(Some(value.to_string()));
    }

    if let Some(value) = value.as_datetime() {
        return Ok(Some(value.to_string()));
    }

    Err(ElsewhereError::InvalidFrontMatterField {
        field,
        expected: "a string or TOML datetime",
    }
    .into())
}

fn optional_string_array(table: &toml::Table, field: &'static str) -> Result<Vec<String>> {
    let Some(value) = table.get(field) else {
        return Ok(Vec::new());
    };

    let Some(array) = value.as_array() else {
        return Err(ElsewhereError::InvalidFrontMatterField {
            field,
            expected: "an array of strings",
        }
        .into());
    };

    let mut values = Vec::new();

    for item in array {
        let Some(item) = item.as_str() else {
            return Err(ElsewhereError::InvalidFrontMatterField {
                field,
                expected: "an array of strings",
            }
            .into());
        };

        values.push(item.to_string());
    }

    Ok(values)
}

fn optional_tags(table: &toml::Table) -> Result<Vec<String>> {
    let taxonomy_tags = optional_taxonomy_tags(table)?;

    if !taxonomy_tags.is_empty() {
        return Ok(taxonomy_tags);
    }

    optional_string_array(table, "tags")
}

fn optional_taxonomy_tags(table: &toml::Table) -> Result<Vec<String>> {
    let Some(taxonomies) = table.get("taxonomies") else {
        return Ok(Vec::new());
    };

    let Some(taxonomies) = taxonomies.as_table() else {
        return Err(ElsewhereError::InvalidFrontMatterField {
            field: "taxonomies",
            expected: "a table",
        }
        .into());
    };

    optional_string_array(taxonomies, "tags")
}

fn optional_elsewhere_frontmatter(table: &toml::Table) -> Result<Option<ElsewhereFrontMatter>> {
    let direct = optional_elsewhere_table(table.get("elsewhere"), "elsewhere")?;
    let zola_extra = optional_zola_elsewhere_table(table)?;

    // Prefer Zola-native metadata if both are present.
    Ok(zola_extra.or(direct))
}

fn optional_zola_elsewhere_table(table: &toml::Table) -> Result<Option<ElsewhereFrontMatter>> {
    let Some(extra) = table.get("extra") else {
        return Ok(None);
    };

    let Some(extra_table) = extra.as_table() else {
        return Err(ElsewhereError::InvalidFrontMatterField {
            field: "extra",
            expected: "a table",
        }
        .into());
    };

    optional_elsewhere_table(extra_table.get("elsewhere"), "extra.elsewhere")
}

fn optional_elsewhere_table(
    value: Option<&toml::Value>,
    field: &'static str,
) -> Result<Option<ElsewhereFrontMatter>> {
    let Some(value) = value else {
        return Ok(None);
    };

    let Some(table) = value.as_table() else {
        return Err(ElsewhereError::InvalidFrontMatterField {
            field,
            expected: "a table",
        }
        .into());
    };

    Ok(Some(ElsewhereFrontMatter {
        excerpt: optional_string(table, "excerpt")?,
        mastodon: optional_elsewhere_target(table, "mastodon")?,
        bluesky: optional_elsewhere_target(table, "bluesky")?,
        markdown: optional_elsewhere_target(table, "markdown")?,
        reddit: optional_reddit_target(table, "reddit")?,
    }))
}

fn optional_elsewhere_target(
    table: &toml::Table,
    key: &'static str,
) -> Result<Option<ElsewhereTargetOverride>> {
    let Some(value) = table.get(key) else {
        return Ok(None);
    };

    let Some(target_table) = value.as_table() else {
        return Err(ElsewhereError::InvalidFrontMatterField {
            field: key,
            expected: "a table",
        }
        .into());
    };

    Ok(Some(ElsewhereTargetOverride {
        template: optional_string(target_table, "template")?,
    }))
}

fn optional_reddit_target(
    table: &toml::Table,
    key: &'static str,
) -> Result<Option<RedditTargetOverride>> {
    let Some(value) = table.get(key) else {
        return Ok(None);
    };

    let Some(target_table) = value.as_table() else {
        return Err(ElsewhereError::InvalidFrontMatterField {
            field: key,
            expected: "a table",
        }
        .into());
    };

    let kind = optional_string(target_table, "kind")?
        .map(|kind| RedditPostKind::from_str(kind.as_str()))
        .transpose()?;

    Ok(Some(RedditTargetOverride {
        subreddit: optional_string(target_table, "subreddit")?,
        kind,
        title_template: optional_string(target_table, "title")?,
        body_template: optional_string(target_table, "body")?,
        comment_template: optional_string(target_table, "comment")?,
    }))
}
