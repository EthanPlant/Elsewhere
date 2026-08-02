# Security Model

Elsewhere is a local command-line tool for turning static-site posts into publishing drafts.

Its security model is deliberately small.

Elsewhere reads files you provide, interprets local configuration, constructs a canonical post, and writes proposed publishing output to your terminal.

It does not operate a server, authenticate with social platforms, store platform credentials, or publish anything over the network.

This document describes the security properties and trust boundaries of Elsewhere itself.

For reporting security vulnerabilities in Elsewhere, see [`SECURITY.md`](../SECURITY.md).

## Security boundary

The normal Elsewhere workflow is:

```text
local configuration
        +
local source post
        |
        v
     Elsewhere
        |
        v
plan or publishing draft
        |
        v
     stdout
```

The important boundary is local.

Elsewhere expects the following inputs to be under your control:

- `elsewhere.toml`
- the static site configuration used by a source adapter
- source Markdown files
- any scripts or CI workflows invoking Elsewhere

Elsewhere treats those files as content and configuration. It does not treat them as executable code.

## What Elsewhere does

At this revision, Elsewhere can:

- read `elsewhere.toml`
- search parent directories for that configuration
- read Markdown source files
- read source-specific site configuration such as Zola's `zola.toml`;
- parse TOML front matter
- construct canonical URLs
- substitute canonical post values into templates
- calculate character counts
- produce human-readable plans
- produce JSON plans
- render Mastodon, Bluesky, Reddit, and Markdown drafts
- create an `elsewhere.toml` file through `elsewhere init`

These operations are local.

## What Elsewhere does not do

Elsewhere does not currently:

- authenticate with Mastodon
- authenticate with Bluesky
- authenticate with Reddit
- authenticate with a long-form publishing service
- store OAuth tokens or API keys
- make publishing API requests
- schedule posts
- run a background service
- listen on a network port
- receive inbound network traffic
- operate a hosted service
- execute source Markdown
- execute template contents
- execute arbitrary configuration values
- publish a post automatically

A rendered draft remains local until you deliberately send or copy it somewhere else.

## Network access

The `init`, `plan`, and `render` workflows do not require Elsewhere to contact the destination platforms.

For example:

```sh
elsewhere render mastodon content/writing/example-post.md
```

does not contact Mastodon.

It constructs the text locally and writes it to standard output.

Likewise:

```sh
elsewhere render reddit content/writing/example-post.md
```

does not contact Reddit, verify the configured subreddit, inspect its rules, or submit the proposed post.

This has an important consequence: Elsewhere does not need platform credentials.

It also means Elsewhere cannot tell you whether a destination will actually accept the draft. Platform limits, moderation rules, authentication, and final publication remain outside Elsewhere's security boundary.

## Credentials

Elsewhere does not currently need social-platform credentials.

Do not put API keys, OAuth tokens, passwords, session cookies, or other secrets in:

```
elsewhere.toml
```

or in Elsewhere-specific post metadata.

There is no supported configuration field that requires them.

A future feature that adds direct platform publishing would materially change Elsewhere's threat model and should be treated as a security-sensitive design change.

## Source files

Elsewhere reads user-authored Markdown from the local filesystem.

The source file can influence:

- the canonical title
- description
- date
- tags
- slug
- canonical URL
- Markdown body
- editorial excerpt
- renderer templates
- structured Reddit fields

This content may then appear in terminal output, JSON plans, redirected files, logs, or downstream tools.

Treat source files from an untrusted repository as untrusted data.

Elsewhere does not execute their Markdown, but a malicious source file can still produce malicious-looking or misleading output.

For example, a post could contain:

```
$(rm -rf "$HOME")
```

Elsewhere treats this as text.

Your shell may not if you later pass the rendered output through `eval`, command substitution, or another unsafe wrapper.

Do not execute Elsewhere output as shell code.

## Configuration files

`elsewhere.toml` is also trusted local input.

Elsewhere uses it to control:

- source selection
- site URL construction
- content paths
- renderer templates
- character limits
- other rendering behaviour

A malicious configuration cannot directly execute arbitrary code through Elsewhere's template language, but it can change the drafts Elsewhere produces.

For example, an untrusted configuration could replace:

```
{url}
```

with unrelated or deceptive text in a publishing template.

Review configuration from repositories you do not trust before publishing any output derived from it.

## Configuration discovery

Elsewhere searches for `elsewhere.toml` beginning from the source post's directory and walking upward through parent directories.

For example:

```text
site/
├── elsewhere.toml
└── content/
    └── writing/
        └── post.md
```

uses the configuration at:

```
site/elsewhere.toml
```

A more deeply nested configuration takes precedence because it is encountered first.

This makes configuration discovery convenient, but it also means the configuration used for a post is determined partly by its filesystem location.

When working in an unfamiliar checkout, use `elsewhere plan` and verify that the resulting canonical URL and renderer output are what you expect before publishing anything.

## Templates

Elsewhere's template language is substitution-based.

A template such as:

```
{excerpt}

{url}
```

replaces known variables with values from the canonical post.

Templates do not support:

- shell commands
- executable expressions
- arbitrary functions
- scripting
- loops
- conditionals
- filesystem access

An unknown variable is an error rather than executable input.

For example:

```
{system("rm -rf /")}
```

is not interpreted as a function call.

It is an unsupported template variable.

This keeps the template boundary deliberately small.

## Template output is not escaped

Elsewhere prepares text and Markdown drafts.

It does not attempt to sanitize canonical post values for every possible downstream context.

For example, a title may contain:

```
<example>
```

or:

```
$(command)
```

and those characters may appear unchanged in rendered output.

That is normally correct for a draft intended for human review.

It also means Elsewhere output should not be assumed to be safe for:

- shell evaluation
- HTML insertion
- SQL queries
- configuration-file generation
- source-code generation
- another interpreter

without context-appropriate escaping by the consuming tool.

The fact that data passed through Elsewhere does not make it trusted.

## Standard output

Rendered drafts are written to standard output.

For example:

```sh
elsewhere render markdown content/writing/example-post.md
```

prints the Markdown draft.

You can redirect it yourself:

```sh
elsewhere render markdown content/writing/example-post.md > post.md
```

The file creation in this example is performed by the shell redirection, not by Elsewhere's Markdown renderer.

Be careful when redirecting output to an existing path. Your shell may replace that file.

Elsewhere cannot protect a file from a redirection you explicitly requested outside the program.

## Diagnostics

Elsewhere keeps publishing output separate from diagnostics so rendered output can be redirected cleanly.

This is useful for workflows such as:

```sh
elsewhere render markdown post.md > exported.md
```

but does not make the output non-sensitive.

A renderer may emit:

- unpublished titles
- editorial excerpts
- canonical URLs
- full article bodies
- proposed Reddit posts
- suggested Reddit comments

Treat redirected output according to the sensitivity of the source material.

## Planning output

`elsewhere plan` is intended for review, but a plan can contain substantial information about an unpublished post.

Human-readable plans may expose:

- titles
- canonical URLs
- excerpts
- rendered social drafts
- Reddit destination information
- warnings about unpublished state

JSON plans expose the same model in machine-readable form:

```sh
elsewhere plan --json content/writing/example-post.md
```

A JSON plan is not merely build metadata.

Treat it as derived content.

## CI

Elsewhere can be used in CI to confirm that posts can be parsed and rendered:

```sh
elsewhere plan --json content/writing/example-post.md > plan.json
```

This can catch:

- invalid front matter
- broken templates
- missing required values
- renderer errors
- configured character-limit warnings

CI introduces a different exposure boundary.

A plan generated from an unpublished post may contain information you did not intend to make public.

Be careful with:

- public workflow logs
- uploaded artifacts
- build summaries
- debugging output
- cache contents
- third-party CI integrations

Do not upload `plan.json` as a public artifact unless you intend its contents to be public.

For a public repository, assume that ordinary CI logs and artifacts may be visible to other people.

## Untrusted pull requests

Running Elsewhere against content introduced by an untrusted pull request means processing attacker-controlled Markdown and potentially attacker-controlled `elsewhere.toml`.

Elsewhere itself does not execute those files as code.

That does not make a surrounding CI job automatically safe.

The repository's tests, build scripts, shell commands, actions, and other tooling may have broader capabilities than Elsewhere.

If Elsewhere is used as part of a pull-request workflow:

- use the minimum required token permissions;
- do not expose repository secrets to untrusted code;
- do not execute rendered output;
- avoid publishing generated plans automatically; and
- review the security properties of the complete workflow rather than only Elsewhere.

Elsewhere's local-first design limits its own authority. It does not limit the authority of the environment that invokes it.

## `elsewhere init`

`elsewhere init` is the main Elsewhere operation that deliberately creates a project file.

For example:

```sh
elsewhere init --source zola
```

creates:

```
elsewhere.toml
```

in the current directory.

Elsewhere refuses to replace an existing configuration unless you explicitly pass:

```sh
--force
```

For example:

```sh
elsewhere init --source zola --force
```

replaces the existing `elsewhere.toml`.

Treat `--force` as a destructive operation.

Review or commit an existing configuration before replacing it if you may need to recover it.

## `plan` and `render`

`plan` and `render` are read-oriented with respect to the static-site project.

They read configuration and source material and emit results.

They do not modify the source post as part of the normal workflow.

For example:

```sh
elsewhere render mastodon post.md
```

does not edit:

```
post.md
```

and:

```sh
elsewhere plan post.md
```

does not record approval state inside the source file.

Any output file created through shell redirection remains the responsibility of the calling shell or script.

## Canonical URLs

Elsewhere derives canonical URLs from local configuration and source metadata.

It does not request the URL to confirm that:

- the page exists
- TLS is valid
- the domain belongs to you
- the page contains the expected post
- the post is already public

A misconfigured or malicious `site_url`, `path`, `slug`, `url_pattern`, or `canonical_url` can therefore cause a draft to point somewhere unintended.

Always review the canonical URL in:

```sh
elsewhere plan path/to/post.md
```

before publishing a derived post.

For a draft source, verify that the canonical page is actually public before posting the derivative elsewhere.

## Draft posts

Elsewhere allows posts marked:

```toml
draft = true
```

to be planned and rendered.

It warns instead of refusing.

This is an editorial feature: you may reasonably want to prepare social drafts before the canonical article is published.

It is also a potential disclosure risk.

Rendering or logging a draft can expose material before your static site publishes it.

A draft flag is not an access-control mechanism.

Do not rely on it to keep content secret.

## Markdown output

The Markdown renderer preserves the canonical Markdown body.

Elsewhere does not execute or sanitize generator-specific markup inside it.

A source body may therefore contain:

- HTML
- shortcodes
- internal links
- template syntax
- custom Markdown extensions
- other destination-sensitive material

When the output is fed into another Markdown renderer or publishing system, that system determines what the markup means.

Review long-form exports before passing them to another renderer, especially when the original source came from somewhere you do not trust.

## Reddit artifacts

The Reddit renderer prepares structured fields such as:

- subreddit
- submission kind
- title
- URL
- self-post body
- suggested first comment

These are proposals.

Elsewhere does not validate subreddit rules or contact Reddit.

A configured subreddit should therefore not be treated as an authorization boundary or validation result.

Before publishing, confirm both the destination and the rendered content yourself.

## Character limits

Elsewhere reports configured character limits as warnings.

These checks are not security controls.

They do not verify the destination's current rules, prevent publication, sanitize content, or guarantee that an external platform will accept the draft.

Elsewhere deliberately leaves the final publishing decision outside the program.

## No automatic publishing

The human review step is part of Elsewhere's security model.

The intended flow is:

```
write -> plan -> review -> render -> edit -> publish manually
```

Elsewhere stops before publication.

A wrapper can remove that boundary by doing something like:

```
elsewhere render -> platform API
```

but that is no longer the security model documented here.

An automated wrapper must handle its own:

- credentials
- secret storage
- network failures
- retries
- destination validation
- rate limits
- duplicate publication
- authorization
- logging
- and rollback behaviour

Do not assume Elsewhere's local security properties automatically extend to an external publishing script.

## Local-first does not mean risk-free

Elsewhere has a small attack surface because it has little authority.

It does not have platform credentials or a network publishing capability that can be abused if a post contains surprising input.

Its main risks are therefore around local data handling:

- publishing the wrong derived text;
- leaking unpublished content through logs or artifacts;
- using the wrong canonical URL;
- trusting configuration from an unfamiliar repository;
- overwriting `elsewhere.toml` with `init --force`;
- feeding untrusted rendered text into a dangerous downstream interpreter; or
- adding automation that removes the human review boundary.

The safest workflow is also the ordinary one:

```sh
elsewhere plan content/writing/example-post.md
```

read the plan, then:

```sh
elsewhere render <target> content/writing/example-post.md
```

read the resulting draft, edit it if necessary, and publish it yourself.

## Reporting vulnerabilities

This document explains how Elsewhere is designed to behave.

If you discover a way for Elsewhere itself to cross these boundaries unexpectedly — for example, arbitrary code execution, unintended file modification, credential exposure, or another security vulnerability — do not report sensitive details in a public issue.

Follow the vulnerability-reporting instructions in [`SECURITY.md`](../SECURITY.md).
