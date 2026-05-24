# Elsewhere

Elsewhere treats your static site as the canonical source and renders platform-specific copies for other places.

It is a local POSSE (Post on your own site, syndicate elsewhere) CLI for static-site writers.

Your website is the home. Platforms are edges.

## Status

Elsewhere is currently in early development.

The CLI exists, the command structure is in place, and the source/renderer layout has been sketched out. The commands are placeholders until Markdown parsing and real rendering land in Phase 1.

## Commands

```sh
elsewhere --help
elsewhere init
elsewhere plan <post>
elsewhere render <target> <post>
```

Supported render targets:

mastodon
bluesky
substack

## Philosophy
Elsewhere is deliberately not a social media management app. It is not a content management system.

It does not start with platform accounts, OAuth tokens, dashboards, analytics, scheduling, or growth tools.

It starts with a simpler assumption:

Your static site is the canonical home of your writing.

Elsewhere reads a post from your site and prepares copies, excerpts, or platform-specific versions for other places.

## Example
Create a config file:

```bash
elsewhere init
```
Plan syndication for a post:
```bash
elsewhere plan content/writing/example.md
```
Render a placeholder Mastodon version:
```bash
elsewhere render mastodon content/writing/example.md
```

## Roadmap
Phase 1: parse Markdown and front matter
Phase 2: config and canonical URLs
Phase 3: real Mastodon, Bluesky, and Substack renderers
Phase 4: Zola support
Phase 5: editorial overrides
