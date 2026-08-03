# Changelog

## [0.1.1] - 2026-08-03

Elsewhere 0.1.1 improves project setup, Zola compatibility, Reddit rendering, documentation, testing, and the release process.

### Added

- Added source-aware configuration to `elsewhere init`.

  - `elsewhere init --source zola` creates a starter configuration for a Zola site.
  - `elsewhere init --source generic` creates a starter configuration for a generic Markdown site.
  - Generic Markdown remains the default source.
- Added support for both `zola.toml` and `config.toml` when reading Zola site configuration. If both are present, `zola.toml` takes precedence.
- Added test coverage for structured Reddit output, including Reddit artifacts in JSON plans.
- Added a CI coverage check with a minimum line coverage threshold of 70%.
- Added automated crates.io publishing for tagged releases, including release-tag and package-version validation.
- Added a security policy and expanded security documentation.

### Changed

- Reworked the project documentation around the actual Elsewhere workflow: write, plan, review, render, edit, and publish.
- Expanded documentation for configuration, sources, renderers, planning, JSON schema, generic Markdown, and Zola.
- Updated the README with crates.io installation instructions and source-specific quick-start examples.
- Improved Cargo package metadata for crates.io.

### Fixed

- Fixed Reddit subreddit normalization. Values may now be written as `example`, `r/example`, or `/r/example`.
- Fixed the Reddit configuration in the runnable Zola example.
- Improved handling of missing Zola configuration files.
- Updated `anyhow` to 1.0.104 to address [RUSTSEC-2026-0190](https://rustsec.org/advisories/RUSTSEC-2026-0190).
