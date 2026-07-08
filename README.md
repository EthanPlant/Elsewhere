# Elsewhere

Elsewhere treats your static site as the canonical source and renders platform-specific copies for other places.

It is a local POSSE (Post on your own site, syndicate elsewhere) CLI for static-site writers.

Your website is the home. Platforms are edges.

## Status

Elsewhere is currently in early development, but it is already usable for a basic static-site publishing workflow.

It can currently read Markdown posts with TOML front matter, derive canonical URLs from site configuration, understand basic Zola site structure, and render output for Mastodon, Bluesky, and Substack.

It does not publish directly to any platform. That is a deliberate design choice. Elsewhere currently prepares the output; you decide where it goes.

## Philosophy
Elsewhere is deliberately not a social media management app. It is not a content management system.

It does not start with platform accounts, OAuth tokens, dashboards, analytics, scheduling, or growth tools.

It starts with a simpler assumption:

> Your static site is the canonical home of your writing.

Elsewhere reads a post from your site and prepares copies, excerpts, or platform-specific versions for other places. The goal isn't to spray the same content everywhere. The goal is to make it easier to keep your own site canonical while still participating elsewhere.

## Installation

Durin development, Elsewhere can be installed from a local checkout:
```sh
cargo install --path .`
```
This installs the `elsewhere` binary into Cargo's binary directory. Typically: `~/.cargo/bin/elsewhere`. Make sure `~/.cargo/bin` is in your `PATH`.

## Commands:
`elsewhere --help`
`elsewhere init`
`elsewhere plan <post>`
`elsewhere render <target> <post>`

Supported render targets:

`mastodon`
`bluesky`
`substack`

## Quick Start
Create an `elsewhere.toml` file in the root of your static site:
```sh
elsewhere init
```

Plan syndication for a post:
```sh
elsewhere plan content/writing/example.md
```

Render a Mastodon version:
```sh
elsewhere render mastodon content/writing/example.md
```

Render a Substack Markdown version and write it to a file:
```sh
elsewhere render substack content/writing/example.md > substack.md
```

## Example
Given a post at `content/writing/example.md`. Elsewhere can derive the canonical URL and render a MAstodon post:

```sh
elsewhere render mastodon content/writing/example.md`
```
```
mastodon render: 116/500 characters
This is an example post syndicated through Elsewhere.

New Essay: Elsewhere Example
https://example.com/writing/example
```

## Configuration
Elsewhere is configured with an `elsewhere.toml` file in the root of your site. A starter config can be created by running
```sh
elsewhere init
```
The minimal config describes three things:
```toml
content_dir = "content"
source = "zola"

[defaults]
canonical_phrase = "Originally published on my website:"
```

`content_dir` tells Elsewhere where your posts live.

`source` tells Elsewhere how to interpret your site structure. Currently supported values are:

- `generic`
- `zola`

`defaults` contains shared rendering settings used by multiple render targets.

Renderers can also be customized in `elsewhere.toml`:
```toml
[mastodon] 
max_chars = 500 
template = """
{first_paragraph} 

New post: {title} 
{url}"""
```

Supported template variables currently include title, description, first paragraph, canonical URL, tags, hashtags, body, date, and slug.

## What works today

Elsewhere currently supports:

- Markdown posts with TOML front matter
- basic generic Markdown sites
- basic Zola sites
- canonical URL derivation
- slug, path, and draft front matter
- Mastodon, Bluesky, and Substack renderers
- simple template variables
- character counts and over-limit warnings

Zola support is intentionally limited. Elsewhere supports the common page case, not the full Zola routing model.

## Roadmap
Near-term
- improve plan output
- add editorial overrides, such as explicit POSSE excerpts
- add plain-text variants for Markdown-derived fields
- add tests around config discovery, canonical URLs, and renderers

Later:

- support more static-site generators
- track where posts have been syndicated
- optionally write syndication URLs back into front matter
- support u-syndication links
- consider direct publishing only after the rendering workflow is solid

## License
Elsewhere is published under the GNU General Public License, version 3 or later.

See LICENSE for details.
