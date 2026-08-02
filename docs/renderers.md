# Renderers

A renderer turns Elsewhere's canonical post into a draft for a particular publishing destination.

```
source post
    -> canonical post
    -> renderer
    -> publishing draft
```

Elsewhere currently includes four renderers:

- Mastodon
- Bluesky
- Reddit
- Markdown

Renderers prepare output. They do not authenticate with platforms, make network requests, create posts, or decide that a draft is ready to publish.

The intended workflow remains:

```
plan -> review -> render -> edit -> publish manually
```

See [Sources](sources.md) for the canonical post model and [Planning and Review](planning.md) for inspecting proposed output before rendering it.

## Render a target

Use `elsewhere render` with a target and source post:

```sh
elsewhere render mastodon content/writing/example-post.md
```

Supported target names are:

```
mastodon
bluesky
reddit
markdown
all
```

For example:

```sh
elsewhere render bluesky content/writing/example-post.md
```

```sh
elsewhere render reddit content/writing/example-post.md
```

```sh
elsewhere render markdown content/writing/example-post.md
```

To render every target:

```sh
elsewhere render all content/writing/example-post.md
```

Review the post first with:

```sh
elsewhere plan content/writing/example-post.md
```

Planning exposes target errors, warnings, character counts, and proposed fields without requiring you to render each target separately.

## Output and diagnostics

Elsewhere writes rendered drafts to standard output.

Warnings and other diagnostics are written separately so that a successful draft can be redirected into a file or another local command.

For example:

```sh
elsewhere render markdown content/writing/example-post.md > example-post.md
```

The redirected file contains the Markdown draft rather than Elsewhere's diagnostic messages.

Rendered output may contain unpublished titles, excerpts, URLs, or complete post bodies. Be careful when redirecting it, logging terminal sessions, or passing it into CI tools.

See [Security Model](security.md) for the local output boundary.

## The common rendering model

Mastodon, Bluesky, and Markdown are template-based renderers.

Each renderer:

1. selects its effective template
2. resolves variables from the canonical post
3. substitutes those variables into the template
4. calculates any applicable character counts
5. reports warnings
6. emits the rendered draft

Reddit uses the same template system for its individual fields, but produces a structured publishing proposal rather than one block of text.

## Template precedence

Elsewhere chooses a template in this order:

1. a renderer-specific override on the source post
2. the renderer's site-level configuration in `elsewhere.toml`
3. the renderer's built-in default

A post-level override only affects that post.

For example, a Zola post can replace its Mastodon template:

```toml
[extra.elsewhere.mastodon]
template = """
A small note about this one:

{excerpt}

{url}
"""
```

A generic Markdown post uses:

```toml
[elsewhere.mastodon]
template = """
A small note about this one:

{excerpt}

{url}
"""
```

The Bluesky and Markdown renderers use the same override shape:

```toml
[extra.elsewhere.bluesky]
template = "..."

[extra.elsewhere.markdown]
template = "..."
```

Reddit overrides are structured and are merged with the site-level Reddit configuration. See [Reddit overrides](#reddit-overrides).

## Template variables

Templates contain variable names enclosed in braces:

```toml
template = """
New post: {title}

{excerpt}

{url}
"""
```

The current template variables are:

| Variable             | Value                                      |
| -------------------- | ------------------------------------------ |
| `{title}`            | Post title                                 |
| `{description}`      | Post description                           |
| `{excerpt}`          | Editorial excerpt selected by Elsewhere    |
| `{first_paragraph}`  | First non-empty block of the Markdown body |
| `{url}`              | Canonical URL                              |
| `{canonical_url}`    | Alias for `{url}`                          |
| `{date}`             | Post date                                  |
| `{slug}`             | Post slug                                  |
| `{tags}`             | Tags joined with `, `                      |
| `{body}`             | Markdown body without front matter         |
| `{body_markdown}`    | Alias for `{body}`                         |
| `{canonical_phrase}` | Shared phrase from `[defaults]`            |

Whitespace inside the braces is ignored:

```
{ title }
```

is equivalent to:

```
{title}
```

Templates do not support:

- conditionals;
- loops;
- filters;
- functions;
- expressions; or
- arbitrary code execution.

An unknown variable is an error:

```
{author}
```

Elsewhere also reports an error when a template requests an optional value that the canonical post does not contain.

For example, this template requires a description:

```toml
template = """
{description}

{url}
"""
```

If the post has no `description`, the renderer cannot prepare the draft.

See [Configuration](configuration.md) for the complete variable reference.

## Editorial excerpts

The `{excerpt}` variable is intended for short editorial introductions.

Elsewhere selects the first available value in this order:

1. an explicit Elsewhere excerpt on the post
2. the post description
3. the first paragraph of the body
4. the post title

For a Zola post:

```toml
[extra.elsewhere]
excerpt = "A shorter introduction written for syndication."
```

For generic Markdown:

```toml
[elsewhere]
excerpt = "A shorter introduction written for syndication."
```

Because the title is the final fallback, `{excerpt}` always produces a value for a valid post.

Elsewhere does not automatically summarize, shorten, or rewrite the selected excerpt for different platforms. Use renderer-specific templates or post-level overrides when the same excerpt does not work everywhere.

## Character limits

Mastodon, Bluesky, and the individual Reddit fields can have configured character limits.

Elsewhere compares the rendered output with those limits and reports a warning when a draft is too long.

It does not:

- truncate the draft
- remove part of the canonical URL
- shorten the title
- generate a thread
- rewrite the excerpt
- refuse to show the output

Character limits are planning aids, not platform validation. Elsewhere does not contact a destination to confirm its current rules or account-specific behaviour.

A draft that fits the configured limit may still need editing because of destination rules, URL handling, formatting, or community expectations.

## Mastodon

The Mastodon renderer produces one plain-text publishing draft.

Render it with:

```sh
elsewhere render mastodon content/writing/example-post.md
```

The built-in Mastodon template is:

```
{excerpt}

New post: {title}

{url}
```

The built-in character limit is `500`.

A site-level configuration might use:

```toml
[mastodon]
max_chars = 500
template = """
{excerpt}

New post: {title}

{url}
"""
```

The renderer substitutes the canonical post values and emits the completed text.

Given:

```toml
title = "Your Website Should Be the Source of Truth"
description = "A note about writing locally and publishing elsewhere."
canonical_url = "https://example.com/writing/source-of-truth/"
```

the built-in template produces:

```
A note about writing locally and publishing elsewhere.

New post: Your Website Should Be the Source of Truth

https://example.com/writing/source-of-truth/
```

Elsewhere reports a warning when the completed draft exceeds `max_chars`.

The URL is part of the rendered template and therefore part of the local character count.

### Mastodon overrides

A Zola post can replace the template under `[extra.elsewhere.mastodon]`:

```toml
[extra.elsewhere.mastodon]
template = """
I wrote about why the original should live on your own site.

{url}
"""
```

Generic Markdown uses `[elsewhere.mastodon]`:

```toml
[elsewhere.mastodon]
template = """
I wrote about why the original should live on your own site.

{url}
"""
```

A post-level override replaces the complete template. It does not append to or partially modify the site-level template.

The site-level character limit still applies.

## Bluesky

The Bluesky renderer also produces one plain-text publishing draft.

Render it with:

```sh
elsewhere render bluesky content/writing/example-post.md
```

The built-in Bluesky template is:

```
New essay: {title}

{excerpt}

{url}
```

The built-in character limit is `300`.

A site-level configuration might use:

```toml
[bluesky]
max_chars = 300
template = """
New essay: {title}

{excerpt}

{url}
"""
```

Given the same canonical post, the built-in template produces:

```
New essay: Your Website Should Be the Source of Truth

A note about writing locally and publishing elsewhere.

https://example.com/writing/source-of-truth/
```

Elsewhere reports a warning when the completed draft exceeds `max_chars`.

Elsewhere does not apply Bluesky-specific text facets or construct an API request. The output is a draft for you to copy, review, edit, and publish manually.

### Bluesky overrides

A Zola post can replace the template with:

```toml
[extra.elsewhere.bluesky]
template = """
The original belongs on your website.

{title}
{url}
"""
```

Generic Markdown uses:

```toml
[elsewhere.bluesky]
template = """
The original belongs on your website.

{title}
{url}
"""
```

The override replaces the full Bluesky template for that post.

The site-level character limit continues to apply.

## Reddit

Reddit does not have one universal post shape.

A proposed Reddit submission may contain:

- a subreddit
- a submission kind
- a title
- a URL
- a self-post body
- a suggested first comment

The Reddit renderer therefore emits a structured publishing draft rather than one undifferentiated string.

Render it with:

```sh
elsewhere render reddit content/writing/example-post.md
```

Elsewhere currently supports two Reddit submission kinds:

```
link
selfpost
```

The default is `link`.

Elsewhere does not contact Reddit, validate that a subreddit exists, inspect its rules, or submit any of the proposed fields.

### Link submissions

A link submission uses the canonical URL as the Reddit submission URL.

A site-level configuration might use:

```toml
[reddit]
kind = "link"
subreddit = "indieweb"
title_template = "{title}"
comment_template = """
{excerpt}

Originally published here:
{url}
"""
title_max_chars = 300
body_max_chars = 40000
comment_max_chars = 10000
```

The resulting proposal contains:

```
subreddit
kind
title
url
suggested first comment
```

The suggested first comment is optional.

A link submission does not use `body_template`.

### Self posts

A self post uses a rendered body instead of the canonical URL as the submission target.

Configure one with:

```toml
[reddit]
kind = "selfpost"
subreddit = "indieweb"
title_template = "{title}"
body_template = """
{excerpt}

Read the original on my website:

{url}
"""
title_max_chars = 300
body_max_chars = 40000
comment_max_chars = 10000
```

The resulting proposal contains:

```
subreddit
kind
title
body
```

A self post does not use `comment_template`.

The canonical URL can still appear inside the rendered body through `{url}`.

### Subreddit names

The `subreddit` field may be written as:

```toml
subreddit = "indieweb"
```

```toml
subreddit = "r/indieweb"
```

or:

```toml
subreddit = "/r/indieweb"
```

Elsewhere normalizes these forms for human-readable output.

A missing subreddit produces a warning rather than preventing the rest of the Reddit draft from being prepared.

Elsewhere does not confirm that the named community exists.

### Reddit templates

Reddit uses separate templates for separate submission fields.

#### `title_template`

The proposed submission title:

```toml
title_template = "{title}"
```

The default is:

```
{title}
```

#### `body_template`

The proposed body for a self post:

```toml
body_template = """
{excerpt}

{url}
"""
```

The default is:

```
{excerpt}

{url}
```

This template is only used when `kind = "selfpost"`.

#### `comment_template`

An optional suggested first comment for a link submission:

```toml
comment_template = """
Some additional context:

{excerpt}

{url}
"""
```

Elsewhere includes the rendered comment in the proposal. It does not post the comment.

This template is only used when `kind = "link"`.

### Reddit character limits

Reddit has separate local limits for its structured fields:

```toml
title_max_chars = 300
body_max_chars = 40000
comment_max_chars = 10000
```

Elsewhere checks the fields applicable to the proposed submission.

For a link submission, that normally means:

- title
- suggested first comment, when configured

For a self post, that means:

- title
- body

Exceeding a limit produces a warning. Elsewhere preserves the complete proposed field.

### Reddit overrides

Reddit overrides are merged field by field with the site-level configuration.

A Zola post can override only the subreddit:

```toml
[extra.elsewhere.reddit]
subreddit = "selfhosted"
```

The site-level kind and templates continue to apply.

It can instead turn one post into a self post:

```toml
[extra.elsewhere.reddit]
kind = "selfpost"
subreddit = "indieweb"
body_template = """
{body}

---

Originally published at {url}
"""
```

Generic Markdown uses:

```toml
[elsewhere.reddit]
kind = "selfpost"
subreddit = "indieweb"
body_template = """
{body}

---

Originally published at {url}
"""
```

Supported post-level Reddit fields are:

- `kind`
- `subreddit`
- `title_template`
- `body_template`
- `comment_template`

Character limits remain site-level configuration.

Because Reddit overrides are merged rather than replacing the entire configuration, a post can change one editorial decision without copying every default.

## Markdown

The Markdown renderer produces a long-form Markdown draft.

Render it with:

```sh
elsewhere render markdown content/writing/example-post.md
```

Save it to a file with:

```sh
elsewhere render markdown content/writing/example-post.md > example-post.md
```

The built-in Markdown template is:

```
# {title}

_{description}_

{body}

{canonical_phrase}
{url}
```

A site-level configuration might use:

```toml
[markdown]
template = """
# {title}

_{description}_

{body}

{canonical_phrase}
{url}"""
```

The renderer does not copy the original source file directly.

It:

1. reads the canonical post
2. removes the source front matter
3. selects values from the canonical model
4. constructs a new document from the output template

This distinction allows Elsewhere to remove static-site-specific metadata while preserving the post body as Markdown.

### Markdown input and Markdown output

Elsewhere can read Markdown and render Markdown without the two files serving the same purpose.

The source file may contain:

- Zola front matter
- site-specific paths
- taxonomies
- draft status
- Elsewhere overrides
- other local publishing metadata

The rendered Markdown draft contains only what the Markdown template requests.

For example, the source may be:

```md
+++
title = "The Website Is the Source of Truth"
description = "A note about writing locally."
date = 2026-07-29
draft = false

[taxonomies]
tags = ["indieweb"]

[extra.elsewhere]
excerpt = "The original should live somewhere you control."
+++

The web is very good at publishing the same words in several places.
```

The built-in Markdown renderer produces:

```md
# The Website Is the Source of Truth

_A note about writing locally._

The web is very good at publishing the same words in several places.

Originally published on my website:
https://example.com/writing/the-website-is-the-source-of-truth/
```

The source file remains the version used by your static site. The rendered file is a derived publishing draft.

### Canonical phrase

The Markdown renderer commonly uses `{canonical_phrase}` before the original URL.

Configure it under `[defaults]`:

```toml
[defaults]
canonical_phrase = "Originally published on my website:"
```

Another site might prefer:

```toml
[defaults]
canonical_phrase = "Read the original at:"
```

The phrase is a normal template value. A Markdown template does not have to include it.

### Markdown overrides

A Zola post can replace the Markdown template:

```toml
[extra.elsewhere.markdown]
template = """
# {title}

{body}

---

Original: {url}
"""
```

Generic Markdown uses:

```toml
[elsewhere.markdown]
template = """
# {title}

{body}

---

Original: {url}
"""
```

The override replaces the complete Markdown output template for that post.

The Markdown renderer has no configured character limit.

## Rendering all targets

Use:

```sh
elsewhere render all content/writing/example-post.md
```

to prepare every current target from the same canonical post.

This is useful when one source article will be syndicated to several destinations.

The outputs remain independent:

- Mastodon receives its own template
- Bluesky receives its own template
- Reddit receives structured fields
- Markdown receives a long-form template

`all` does not force the destinations to use the same excerpt, introduction, or output shape. Post-level overrides are applied separately to each target.

Plan the post first:

```sh
elsewhere plan content/writing/example-post.md
```

A plan is easier to review than discovering a target-specific template error while copying several completed drafts.

## Errors and warnings

A renderer reports an error when it cannot produce its draft.

Examples include:

- an unknown template variable
- an unclosed template variable
- a required canonical value that is missing
- an invalid Reddit submission kind
- malformed renderer configuration

Warnings describe conditions that require review but do not prevent output.

Examples include:

- a configured character limit was exceeded
- a Reddit subreddit was not provided
- the source post is still marked as a draft

Elsewhere does not silently repair either condition.

Correct the post, template, or configuration and run:

```sh
elsewhere plan content/writing/example-post.md
```

again.

## Rendered drafts are not API payloads

Renderer output is intended for human review.

The Mastodon and Bluesky renderers produce copyable text. The Reddit renderer produces a readable structured proposal. The Markdown renderer produces a document.

These outputs are not stable platform API request formats.

In particular, Elsewhere does not currently generate:

- Mastodon API requests
- Bluesky records or facets
- Reddit API submissions
- authentication headers
- scheduled jobs
- platform credentials

For stable machine-readable planning data, use:

```sh
elsewhere plan --json content/writing/example-post.md
```

See [JSON Schemas](schemas.md) for that contract.

## Choosing the right renderer

Use Mastodon or Bluesky when you want a short draft that points readers back to the canonical post.

Use Reddit when the destination requires a proposed community, title, submission kind, and possibly a separate body or first comment.

Use Markdown when another long-form publisher accepts Markdown but should not receive your static site's original front matter.

These are starting points rather than mandatory editorial rules.

Elsewhere provides the mechanical transformation. You remain responsible for whether the transformed draft belongs on the destination.
