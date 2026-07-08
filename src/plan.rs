use std::{fmt, path::Path};

use serde::Serialize;

use crate::{renderers, target::RenderTarget, workspace::LoadedPost};

#[derive(Debug, Serialize)]
pub struct PlanOutput {
    pub canonical: PlanCanonical,
    pub targets: Vec<PlanTarget>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PlanCanonical {
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,

    pub tags: Vec<String>,
    pub draft: bool,
}

#[derive(Debug, Serialize)]
pub struct PlanTarget {
    pub target: RenderTarget,
    pub status: PlanStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanStatus {
    Ready,
    Warning,
    Error,
}

impl fmt::Display for PlanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Ready => "ready",
            Self::Warning => "warning",
            Self::Error => "error",
        };

        f.write_str(value)
    }
}

pub fn build_plan(loaded: &LoadedPost, post_path: &Path) -> PlanOutput {
    let post = &loaded.post;

    let warnings = plan_warnings(loaded);

    let targets = RenderTarget::all()
        .iter()
        .copied()
        .map(|target| build_target_plan(loaded, post_path, target))
        .collect();

    PlanOutput {
        canonical: PlanCanonical {
            title: post.title.clone(),
            canonical_url: post.canonical_url.clone(),
            tags: post.tags.clone(),
            draft: post.draft,
        },
        targets,
        warnings,
    }
}

pub fn print_plan(plan: &PlanOutput) {
    println!("Elsewhere plan");
    println!();

    println!("Canonical");
    println!("  Title: {}", plan.canonical.title);
    println!(
        "  URL:   {}",
        plan.canonical.canonical_url.as_deref().unwrap_or("not set")
    );

    if plan.canonical.tags.is_empty() {
        println!("  Tags:  none");
    } else {
        println!("  Tags:  {}", plan.canonical.tags.join(", "));
    }

    if plan.canonical.draft {
        println!("  Draft: yes");
    }

    for warning in &plan.warnings {
        println!("  Warning: {warning}");
    }

    for target in &plan.targets {
        println!();
        print_target_plan(target);
    }
}

fn print_target_plan(target: &PlanTarget) {
    println!("{}", target.target.display_name());
    println!("  Status: {}", target.status);

    match (target.length, target.max_length) {
        (Some(length), Some(max_length)) => {
            println!("  Length: {length} / {max_length}");
        }
        (Some(length), None) => {
            println!("  Length: {length}");
        }
        _ => {}
    }

    if let Some(error) = &target.error {
        println!("  Error: {error}");
    }

    for warning in &target.warnings {
        println!("  Warning: {warning}");
    }

    if let Some(output) = &target.output {
        println!("  Output: {output}");
    }

    if let Some(preview) = &target.preview {
        println!();

        for line in preview.lines() {
            println!("  {line}");
        }
    }
}

fn build_target_plan(loaded: &LoadedPost, post_path: &Path, target: RenderTarget) -> PlanTarget {
    match renderers::render(target, &loaded.post, &loaded.config.config) {
        Ok(rendered) => {
            let warnings: Vec<String> = rendered
                .warnings
                .iter()
                .filter(|warning| warning.as_str() != "Warning: post is marked as draft.")
                .map(|warning| normalize_warning(warning))
                .collect();

            let status = if warnings.is_empty() {
                PlanStatus::Ready
            } else {
                PlanStatus::Warning
            };

            let preview = if target.is_long_form() {
                None
            } else {
                Some(rendered.body)
            };

            let output = if target.is_long_form() {
                Some(format!(
                    "use `elsewhere render {} {} > {}.md`",
                    target.name(),
                    post_path.display(),
                    target.name()
                ))
            } else {
                None
            };

            PlanTarget {
                target,
                status,
                length: Some(rendered.char_count),
                max_length: rendered.max_chars,
                preview,
                output,
                warnings,
                error: None,
            }
        }
        Err(error) => PlanTarget {
            target,
            status: PlanStatus::Error,
            length: None,
            max_length: None,
            preview: None,
            output: None,
            warnings: Vec::new(),
            error: Some(error.to_string()),
        },
    }
}

fn plan_warnings(loaded: &LoadedPost) -> Vec<String> {
    let mut warnings = Vec::new();

    if loaded.post.draft {
        warnings.push("post is marked as draft".to_string());
    }

    warnings
}

fn normalize_warning(warning: &str) -> String {
    warning
        .strip_prefix("Warning: ")
        .unwrap_or(warning)
        .to_string()
}
