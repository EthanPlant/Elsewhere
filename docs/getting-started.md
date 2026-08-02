# Getting Started

This guide takes you from an existing static site to your first set of publishing drafts.

Elsewhere supports two source formats:

- [Zola](zola.md); and
- [generic Markdown](generic-markdown.md) with TOML front matter.

Both use the same basic workflow:

```
write -> plan -> review -> render -> edit -> publish manually
```

Elsewhere does not publish directly to any platform. It prepares drafts for you to review and publish yourself.

## Install Elsewhere

Install the latest release from crates.io:

```sh
cargo install elsewhere
```

Confirm that Elsewhere is available:

```sh
elsewhere --version
```

### Build from source

To build the current development version:

```sh
git clone https://github.com/EthanPlant/Elsewhere.git
cd Elsewhere
cargo build --release
```

The compiled binary will be available at:

```
target/release/elsewhere
```

You can run it directly:

```sh
./target/release/elsewhere --version
```

## Initialize your site

Run `elsewhere init` from the root of your static-site project.

For a Zola site:

```sh
elsewhere init --source zola
```

For a generic Markdown site:

```sh
elsewhere init --source generic
```

Elsewhere creates an `elsewhere.toml` file in the current directory.

The generated file contains a working starting configuration with default templates for the supported renderers. Review it before continuing.

Elsewhere will not replace an existing `elsewhere.toml` unless you pass `--force`:

```sh
elsewhere init --source zola --force
```

Using `--force` discards the existing file.

## Configure your source

The source configuration tells Elsewhere how to read a post and construct its canonical URL.

### Zola

For a Zola site, Elsewhere reads the site configuration and the post's TOML front matter.

Make sure your Zola configuration defines the correct `base_url`:

```toml
base_url = "https://example.com"
```

A minimal Zola post looks like this:

```md
+++
title = "Hello from my website"
description = "A small post about owning your work."
date = 2026-07-29
+++

This is the first paragraph of the post.

The rest of the post continues here.
```

Elsewhere uses the site configuration, the post's location under the content directory, and fields such as `slug`, `path`, or `canonical_url` to determine the public URL.

See [Using Elsewhere with Zola](zola.md) for the complete source model.

### Generic Markdown

For a generic Markdown site, edit the generated `elsewhere.toml` and provide the public site URL, content directory, and URL pattern:

```toml
site_url = "https://example.com"
content_dir = "content"
source = "generic"

[generic]
url_pattern = "/{path}/"
```

A generic Markdown post also uses TOML front matter:

```md
+++
title = "Hello from my website"
description = "A small post about owning your work."
date = 2026-07-29
+++

This is the first paragraph of the post.

The rest of the post continues here.
```

See [Using Generic Markdown](generic-markdown.md) for URL construction, supported fields, and per-post metadata.

## Plan a post

Before rendering anything, inspect the post with `elsewhere plan`:

```sh
elsewhere plan content/writing/hello-from-my-website.md
```

Elsewhere will:

- parse the source post
- construct its canonical URL
- show the metadata available to renderers
- prepare each supported target
- report character counts and configured limits
- show short previews where appropriate
- report warnings or renderer errors

Planning does not create files, publish content, or contact external services.

Review the canonical URL carefully. Every publishing draft should point back to the original post on your website.

### Machine-readable plans

Use `--json` to emit a structured plan:

```sh
elsewhere plan --json content/writing/hello-from-my-website.md
```

JSON plans are useful for local scripts and CI checks. They do not change Elsewhere's publishing model: a plan describes the proposed output but does not publish it.

See [Planning and Review](planning.md) for the complete plan model and [JSON Schemas](schemas.md) for the machine-readable format.

## Render a draft

Once the plan looks correct, render a draft for a specific target.

### Mastodon

```sh
elsewhere render mastodon content/writing/hello-from-my-website.md
```

### Bluesky

```sh
elsewhere render bluesky content/writing/hello-from-my-website.md
```

### Reddit

```sh
elsewhere render reddit content/writing/hello-from-my-website.md
```

Reddit output is structured as a proposed submission rather than one block of text. Depending on the post and its configuration, the draft may include a community, submission type, title, URL or body, and suggested first comment.

### Markdown

```sh
elsewhere render markdown content/writing/hello-from-my-website.md
```

The Markdown renderer produces a clean long-form draft derived from the source post.

To save it to a file:

```sh
elsewhere render markdown content/writing/hello-from-my-website.md > hello-from-my-website.md
```

### All targets

Render every supported target at once:

```sh
elsewhere render all content/writing/hello-from-my-website.md
```

Elsewhere writes rendered drafts to standard output. Diagnostics and warnings are written separately so output can be redirected into another file or command.

## Review the output

Rendered output is a draft.

Before publishing it:

1. confirm that the canonical URL is correct
2. read the rendered text
3. check any character-limit warnings
4. adjust wording for the destination
5. confirm Reddit submission fields where applicable
6. publish the result manually

Elsewhere deliberately leaves this review step to you. Different platforms have different audiences, conventions, and failure modes. Producing a mechanically valid draft is not the same thing as making an editorial decision.

## Customize a post

Site-level templates belong in `elsewhere.toml`.

Individual posts can also provide Elsewhere-specific metadata and renderer overrides in their front matter. These can be used to:

- supply a shorter editorial excerpt
- replace the default text for one platform
- choose Reddit submission details
- change a suggested first comment
- override other renderer-specific fields

Source formats represent these overrides differently. See:

* [Using Elsewhere with Zola](zola.md);
* [Using Generic Markdown](generic-markdown.md); and
* [Renderers](renderers.md).

## Configuration discovery

When processing a post, Elsewhere looks for `elsewhere.toml` in the post’s directory and then walks upward through its parent directories.

This means you can run Elsewhere from outside the project root or pass a deeply nested post path while keeping one configuration file at the root of the site.

For example:

```
my-site/
├── elsewhere.toml
└── content/
    └── writing/
        └── hello-from-my-website.md
```

The post under `content/writing/` will use the `elsewhere.toml` at the site root.

## Troubleshooting

### Elsewhere cannot find its configuration

Make sure an `elsewhere.toml` exists in the post's directory or one of its parent directories.

You can create one from the site root:

```sh
elsewhere init --source zola
```

### The canonical URL is wrong

For Zola, check the site's `base_url` and any `slug`, `path`, or `canonical_url` fields on the post.

For generic Markdown, check `site_url`, `content_dir`, and `generic.url_pattern` in `elsewhere.toml`.

Run `elsewhere plan` again after making changes.

### A post is marked as a draft

Elsewhere warns when the source post is marked as a draft. This prevents an unpublished post from quietly looking like ordinary publishing material.

Confirm that you intend to prepare the post before continuing.

### A renderer exceeds its character limit

Elsewhere reports character counts during planning. Shorten the renderer template, provide a more concise editorial excerpt, or add a per-post override.

Elsewhere does not silently rewrite your post to make it fit.

### A template variable is unavailable

Templates can only use metadata present in the canonical post. Check the source post and the template in `elsewhere.toml`.

See [Sources](sources.md) for the canonical post fields and [Renderers](renderers.md) for template behaviour.

## Next steps

Read [Configuration](configuration.md) to customize your site-wide defaults, then use [Sources](sources.md) and [Renderers](renderers.md) to control how posts are interpreted and transformed.
