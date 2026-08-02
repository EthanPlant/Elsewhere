# Elsewhere

Elsewhere is a small POSSE CLI for static-site writers.

It reads posts from your website, turns them into platform-specific publishing drafts, and lets you review and edit those drafts before posting them elsewhere.

It is not a social media dashboard. It is not an automatic publishing service. It's a little dispatch desk for your website.

```text
write -> plan -> review -> render -> edit -> publish manually
```

Your website is the home. Platforms are edges.

## Why?

Your website should be the source of truth.

Publishing on the web often means copying the same post into several different places. Mastodon wants one shape. Bluesky wants another. Reddit has communities, titles, link posts, self posts, and suggested first comments. Long-form publishing tools may want a clean Markdown draft without your static site's front matter and local metadata.

Elsewhere turns a post from your static site into publishing drafts shaped for those destinations.

The original remains on your website. Everything else is derived.

## Status

Elsewhere is under active development.

The current implementation can:

- read posts from generic Markdown and Zola sites
- derive canonical URLs from source metadata and content paths
- apply site-level templates and per-post editorial overrides
- preview every configured target before rendering
- emit machine-readable plans as JSON
- render Mastodon and Bluesky drafts
- prepare structured Reddit link or self-post drafts
- export long-form Markdown drafts
- warn about draft posts and configured character limits

Elsewhere does not publish directly to any platform.

## Quick Start

Install Elsewhere from crates.io:

```sh
cargo install elsewhere
```

From the root of a Zola site, create an `elsewhere.toml` configuration file:

```sh
elsewhere init --source zola
```

For a generic Markdown site, use:

```sh
elsewhere init --source generic
```

Review how Elsewhere understands a post and what it would produce:

```sh
elsewhere plan content/writing/my-post.md
```

Render a Mastodon draft:

```sh
elsewhere render mastodon content/writing/my-post.md
```

Prepare a Reddit draft:

```sh
elsewhere render reddit content/writing/my-post.md
```

Render every supported target:

```sh
elsewhere render all content/writing/my-post.md
```

Export a long-form Markdown draft:

```sh
elsewhere render markdown content/writing/my-post.md > my-post.md
```

Elsewhere writes rendered drafts to standard output. Review and edit them before publishing.

## Documentation

* [Getting Started](docs/getting-started.md)
* [Configuration](docs/configuration.md)
* [Sources](docs/sources.md)
* [Planning and Review](docs/planning.md)
* [Renderers](docs/renderers.md)
* [Using Elsewhere with Zola](docs/zola.md)
* [Using Generic Markdown](docs/generic-markdown.md)
* [JSON Schemas](docs/schemas.md)
* [Security Model](docs/security.md)
* [Roadmap](docs/roadmap.md)

A complete runnable Zola project is available in [`examples/zola`](examples/zola).

## Security

Elsewhere is a local command-line tool. It reads local configuration and Markdown files, renders publishing drafts, and writes those drafts to standard output.

It does not authenticate with platforms, store platform credentials, or publish anything over the network.

Rendered drafts may contain unpublished writing. Be careful when redirecting output, recording terminal sessions, or running Elsewhere in CI.

Read the [security model](docs/security.md) for the project's trust boundaries. Vulnerabilities should be reported according to [SECURITY.md](SECURITY.md).

## License

Elsewhere is licensed under the GNU General Public License, version 3 or any later version.

See [LICENSE](LICENSE).
