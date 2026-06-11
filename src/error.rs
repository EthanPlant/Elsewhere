use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ElsewhereError {
    #[error("post file does not exist: {0}")]
    PostNotFound(PathBuf),

    #[error("configuration file already exists: {0} (use --force to overwrite)")]
    ConfigExists(PathBuf),

    #[error("configuration file not found: run `elsewhere init` from your site root")]
    ConfigNotFound,

    #[error("Markdown file is missing front matter; expected TOML front matter delimited by +++")]
    MissingFrontMatter,

    #[error("front matter is missing closing delimiter: {0}")]
    UnclosedFrontMatter(&'static str),

    #[error("YAML front matter is not supported yet; use TOML front matter delimited by +++")]
    UnsupportedYamlFrontMatter,

    #[error("failed to parse TOML front matter: {0}")]
    InvalidTomlFrontMatter(#[from] toml::de::Error),

    #[error("front matter must be a TOML table")]
    InvalidFrontMatterRoot,

    #[error("front matter is missing required field `{0}`")]
    MissingRequiredField(&'static str),

    #[error("front matter field `{field}` must be {expected}")]
    InvalidFrontMatterField {
        field: &'static str,
        expected: &'static str,
    },

    #[error("template variable {{{variable}}} is not available for this post")]
    MissingTemplateValue { variable: String },

    #[error("unknown template variable {{{variable}}}")]
    UnknownTemplateVariable { variable: String },

    #[error("template contains an unclosed variable; expected `}}`")]
    UnclosedTemplateVariable,
}
