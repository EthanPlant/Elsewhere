use anyhow::Result;

use crate::{config::Config, error::ElsewhereError, post::CanonicalPost};

pub fn render_template(template: &str, post: &CanonicalPost, config: &Config) -> Result<String> {
    let mut output = String::new();
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '{' {
            output.push(c);
            continue;
        }

        let mut variable = String::new();
        let mut closed = false;

        for next in chars.by_ref() {
            if next == '}' {
                closed = true;
                break;
            }

            variable.push(next);
        }

        if !closed {
            return Err(ElsewhereError::UnclosedTemplateVariable.into());
        }

        let variable = variable.trim();
        let value = resolve_variable(variable, post, config)?;

        output.push_str(&value);
    }

    Ok(output)
}

fn resolve_variable(variable: &str, post: &CanonicalPost, config: &Config) -> Result<String> {
    match variable {
        "title" => Ok(post.title.clone()),
        "description" => post.description.clone().ok_or_else(|| {
            ElsewhereError::MissingTemplateValue {
                variable: variable.to_string(),
            }
            .into()
        }),
        "first_paragraph" => post.first_paragraph.clone().ok_or_else(|| {
            ElsewhereError::MissingTemplateValue {
                variable: variable.to_string(),
            }
            .into()
        }),
        "url" | "canonical_url" => post.canonical_url.clone().ok_or_else(|| {
            ElsewhereError::MissingTemplateValue {
                variable: variable.to_string(),
            }
            .into()
        }),
        "date" => post.date.clone().ok_or_else(|| {
            ElsewhereError::MissingTemplateValue {
                variable: variable.to_string(),
            }
            .into()
        }),
        "slug" => post.slug.clone().ok_or_else(|| {
            ElsewhereError::MissingTemplateValue {
                variable: variable.to_string(),
            }
            .into()
        }),
        "tags" => Ok(post.tags.join(", ")),
        "body" | "body_markdown" => Ok(post.body_markdown.clone()),
        "canonical_phrase" => Ok(config.defaults.canonical_phrase.clone()),

        _ => Err(ElsewhereError::UnknownTemplateVariable {
            variable: variable.to_string(),
        }
        .into()),
    }
}
