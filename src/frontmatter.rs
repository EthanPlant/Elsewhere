use anyhow::Result;

use crate::error::ElsewhereError;

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
        tags: optional_string_array(table, "tags")?,
        canonical_url: optional_string(table, "canonical_url")?,
        slug: optional_string(table, "slug")?,
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
