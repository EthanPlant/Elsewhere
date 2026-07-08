use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{
    config::{LoadedConfig, SourceKind, load_config_for_post},
    error::ElsewhereError,
    post::CanonicalPost,
    sources::{Source, generic::GenericSource, zola::ZolaSource},
};

pub struct LoadedPost {
    pub config: LoadedConfig,
    pub post: CanonicalPost,
}

pub fn load_post(post_path: &Path) -> Result<LoadedPost> {
    ensure_post_exists(post_path)?;

    let loaded_config = load_config_for_post(post_path)?;
    let resolved_post_path = resolve_post_path(&loaded_config.root_dir, post_path);
    let site_url = effective_site_url(&loaded_config)?;

    let mut post = read_post(&loaded_config, &resolved_post_path)?;

    if post.canonical_url.is_none() {
        post.canonical_url = loaded_config.config.derive_canonical_url(
            &site_url,
            &loaded_config.root_dir,
            &resolved_post_path,
            &post,
        );
    }

    Ok(LoadedPost {
        config: loaded_config,
        post,
    })
}

fn read_post(loaded_config: &LoadedConfig, post_path: &Path) -> Result<CanonicalPost> {
    match loaded_config.config.source {
        SourceKind::Generic => {
            let source = GenericSource;
            source.read_post(post_path)
        }
        SourceKind::Zola => {
            let source = ZolaSource::new(loaded_config.root_dir.clone());
            source.read_post(post_path)
        }
    }
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
