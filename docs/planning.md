# Planning and Review

`elsewhere plan` shows how Elsewhere understands a post before you render any publishing drafts.

Planning is the review boundary in Elsewhere's workflow:

```
write -> plan -> review -> render -> edit -> publish manually
```

A plan combines the canonical post with the proposed output for every supported renderer. It reports missing metadata, template errors, character-limit warnings, draft status, and other conditions that may require attention.

Planning does not modify the source post, create output files, contact external services, or publish anything.

## Plan a post

Pass the source post to `elsewhere plan`:

```sh
elsewhere plan content/writing/example-post.md
```

Elsewhere will:

1. locate the applicable `elsewhere.toml`
2. load the selected source adapter
3. parse the post
4. construct the canonical post
5. derive its canonical URL
6. prepare each renderer
7. count characters where limits apply
8. print a reviewable plan

Run the command again after changing the post, its front matter, or `elsewhere.toml`.

## What a plan contains

A plan has two main parts:

```
canonical post
target plans
```

The canonical section describes the original post as Elsewhere understands it.

The target sections describe the drafts Elsewhere could produce from that post.

This distinction is important. A correct renderer cannot compensate for an incorrect canonical URL, title, or source interpretation. Review the canonical post before reviewing individual targets.

## Canonical post

The canonical section identifies the source material shared by every renderer.

It includes information such as:

- the title
- the canonical URL
- the source path
- tags
- draft status
- other metadata relevant to planning

The canonical URL deserves particular attention. It should point to the public copy on your website, not a local file, staging environment, or social platform.

If the URL is wrong, correct the source metadata or site configuration before rendering anything.

See [Sources](sources.md) for the canonical post model and source-specific URL rules.

## Target plans

Elsewhere prepares a separate plan for each current target:

- Mastodon
- Bluesky
- Reddit
- Markdown

Each target is evaluated independently.

A target plan may contain:

- its status
- a rendered preview
- character counts
- configured limits
- warnings
- structured publishing fields
- an error explaining why that renderer could not prepare a draft

A failure in one renderer does not erase useful information from the others. For example, an invalid Reddit override can appear as a Reddit error while the Mastodon and Bluesky plans remain available for review.

## Status values

Each target plan has one of three statuses:

```
ready
warning
error
```

### `ready`

The renderer prepared its proposed output without finding a condition that requires attention.

`ready` does not mean that the draft is editorially complete or safe to publish without reading it. It only means that Elsewhere successfully prepared the configured output.

### `warning`

The renderer prepared output, but the result has one or more warnings.

Warnings may include:

- a character limit was exceeded
- a Reddit community was not configured
- the source post is still a draft
- another non-fatal condition needs review

A warning does not prevent rendering.

Elsewhere reports the condition and leaves the decision to you.

### `error`

The renderer could not prepare its proposed output.

Common causes include:

- an unknown template variable
- a required template value that is absent
- an invalid renderer-specific value
- incompatible renderer configuration

An error affects that target's plan. Source-loading and configuration errors that prevent construction of the canonical post occur before target planning and stop the command entirely.

Correct the error and run `elsewhere plan` again.

## Character counts

Elsewhere counts the characters in short-form and structured fields that have configured limits.

For Mastodon and Bluesky, the plan compares the complete rendered draft with the renderer's configured `max_chars`.

For Reddit, Elsewhere may count separate fields such as:

- the submission title
- the self-post body
- the suggested first comment

A plan reports both the rendered count and the applicable limit.

Exceeding a limit produces a warning. Elsewhere does not:

- truncate the text
- remove words
- shorten the title
- split the draft into a thread
- otherwise rewrite your work to make it fit

You can respond by changing the site-level template, adding a per-post override, or writing a shorter editorial excerpt.

See [Configuration](configuration.md) for limit settings and [Renderers](renderers.md) for target-specific behaviour.

## Character counting is not platform validation

The configured limits are local planning values.

Elsewhere does not contact a platform to determine its current rules, inspect account-specific settings, or verify whether a draft will be accepted.

A draft may still require changes because of:

- platform-specific URL handling
- community rules
- moderation policies
- link previews
- formatting differences
- limits that have changed since your configuration was written

Planning catches predictable local problems. It does not replace reviewing the destination.

## Previews

Where useful, the human-readable plan includes a compact preview of the proposed output.

Previews are intended to answer questions such as:

- Did Elsewhere choose the correct excerpt?
- Is the title repeated unnecessarily?
- Does the canonical URL appear where expected?
- Is the draft obviously too long?
- Did a per-post override take effect?

A preview is not a separate rendering mode. It is derived from the same post, templates, and renderer configuration used by `elsewhere render`.

After reviewing the plan, use `render` to emit the complete publishing draft.

## Short-form targets

Mastodon and Bluesky produce one block of rendered text.

Their plans can therefore show a direct preview along with:

- the total character count
- the configured limit
- any resulting warnings

For example:

```sh
elsewhere plan content/writing/example-post.md
```

lets you inspect both short-form drafts before choosing one to render:

```sh
elsewhere render mastodon content/writing/example-post.md
```

```sh
elsewhere render bluesky content/writing/example-post.md
```

Planning both targets is useful even when their templates look similar. Their limits and editorial conventions may differ.

## Reddit plans

Reddit output is structured rather than represented by one string.

A Reddit plan may describe:

- the proposed subreddit
- whether the submission is a link or self post
- the rendered title
- the canonical submission URL
- the rendered self-post body
- a suggested first comment
- character counts for the applicable fields
- Reddit-specific warnings

Fields depend on the configured submission kind.

### Link submissions

A link submission uses the canonical URL as the submission URL.

Its plan may include:

- subreddit
- title
- URL
- suggested first comment

### Self posts

A self post uses rendered Markdown as the submission body.

Its plan may include:

- subreddit
- title
- body

The plan is a proposed submission, not proof that the post complies with a subreddit's rules. Elsewhere does not contact Reddit or inspect community requirements.

Render the complete structured draft with:

```sh
elsewhere render reddit content/writing/example-post.md
```

## Markdown plans

Markdown is a long-form output.

The plan identifies whether the Markdown renderer can produce the draft without printing the complete article as part of every review. Use the plan to catch template or metadata errors, then render the full document separately:

```sh
elsewhere render markdown content/writing/example-post.md
```

To save the result:

```sh
elsewhere render markdown content/writing/example-post.md > example-post.md
```

The rendered file is a new publishing draft. It is not a copy of the original source file and does not include the source post's original front matter unless your output template explicitly constructs equivalent content.

## Draft warnings

When the canonical post has:

```toml
draft = true
```

Elsewhere reports a warning during planning.

Draft status does not prevent planning or rendering. This is deliberate: you may want to prepare syndication drafts before publishing the original post.

The warning exists because the canonical URL may not yet be publicly available. Publishing a derived draft first would reverse the intended ownership relationship:

```
website first
platforms second
```

Before publishing elsewhere, confirm that the original post is available at its canonical URL.

## Template errors

Planning evaluates renderer templates before you ask Elsewhere to emit their output.

For example, this template refers to an unsupported variable:

```toml
[mastodon]
max_chars = 500
template = """
Written by {author}:

{title}
{url}
"""
```

Because `{author}` is not part of the canonical post model, the Mastodon target reports an error.

A supported variable can also fail when its value is absent. For example:

```toml
template = """
{description}

{url}
"""
```

requires the source post to provide a description.

By contrast, `{excerpt}` always has a fallback for a valid post:

1. explicit Elsewhere excerpt
2. description
3. first paragraph
4. title

Use `plan` after changing templates so these errors are found before you begin copying drafts into publishing interfaces.

## Per-post overrides

Planning applies per-post overrides before producing target plans.

This makes `plan` the easiest way to confirm that an override is:

- placed in the correct front-matter table
- valid for the selected source
- using supported fields
- taking precedence over site defaults
- producing the intended draft

For example, after adding a Mastodon override to a Zola post:

```toml
[extra.elsewhere.mastodon]
template = """
A shorter introduction for Mastodon.

{excerpt}

{url}
"""
```

run:

```sh
elsewhere plan content/writing/example-post.md
```

The Mastodon plan should reflect the post-level template while the other targets continue using their normal configuration.

See [Renderers](renderers.md) for the complete override model.

## Machine-readable plans

Use `--json` to emit the plan as JSON:

```sh
elsewhere plan --json content/writing/example-post.md
```

The JSON representation contains the same planning model as the human-readable output:

- canonical post information
- target plans
- statuses
- counts and limits
- previews or structured artifacts
- warnings
- errors

The JSON output is intended for tools that need structured information rather than terminal-oriented formatting.

For example:

```sh
elsewhere plan --json content/writing/example-post.md > plan.json
```

You can then inspect it with another local tool:

```sh
jq . plan.json
```

See [JSON Schemas](schemas.md) for the exact fields and compatibility expectations.

## Using plans in scripts

Machine-readable plans can support local workflows such as:

- checking whether a canonical URL was derived
- detecting target errors
- displaying character counts
- producing an editorial review interface
- enforcing project-specific checks in CI

A script should distinguish between:

- a command that could not construct a plan; and
- a completed plan containing target warnings or errors.

Do not treat `ready` as permission to publish automatically. It means the renderer prepared output successfully, not that a human has approved the result.

Elsewhere deliberately does not include an automatic publishing command. A wrapper that publishes every `ready` target removes the review boundary provided by the normal workflow.

## Planning in CI

You may run Elsewhere in CI to check that posts can be parsed and rendered.

For example:

```sh
elsewhere plan --json content/writing/example-post.md > plan.json
```

This can catch:

- invalid front matter
- broken templates
- missing required values
- malformed renderer configuration
- character-limit warnings

Remember that a generated plan may contain unpublished titles, excerpts, URLs, or post content. Treat CI logs and uploaded artifacts accordingly.

Do not publish `plan.json` unintentionally.

See [Security Model](security.md) for the local data and output boundary.

## Review checklist

Before rendering, confirm:

1. the title is correct
2. the canonical URL points to the original post
3. the source is not unintentionally marked as a draft
4. the excerpt is appropriate for syndication
5. the expected per-post overrides were applied
6. short-form drafts fit their configured limits
7. Reddit fields describe the intended submission
8. no target reports an unexplained warning
9. no target reports an error

Then render the target you intend to use:

```sh
elsewhere render mastodon content/writing/example-post.md
```

or render every target:

```sh
elsewhere render all content/writing/example-post.md
```

Read and edit the rendered drafts before publishing them manually.

## Plan before publishing

Elsewhere's planning model is intentionally more cautious than:

```
source file -> API request
```

A static-site post and a platform post may share words without serving the same editorial purpose. The plan makes the transformation visible before anything leaves your terminal.

The original is the source of truth.

The plan shows the proposed derivations.

You decide what gets published.
