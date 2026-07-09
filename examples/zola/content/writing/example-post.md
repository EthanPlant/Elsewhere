+++
title = "A Tiny Example Post"
description = "A short demonstration post for Elsewhere's example project."
date = "2026-01-15"

[taxonomies]
tags = ["example", "markdown", "posse"]

[extra.elsewhere]
excerpt = "This is a deliberately small example post used to test syndication drafts."

[extra.elsewhere.mastodon]
template = """
A tiny example appears.

{excerpt}

{url}
"""

[extra.elsewhere.reddit]
subreddit = "example"
kind = "link"
title_template = "A Tiny Example Post"
comment_template = """
This is the suggested first comment for the example Reddit draft.

{excerpt}

Source:
{url}
"""
+++

This is a tiny example post.

It exists so Elsewhere has something safe, boring, and copy-pastable to render during tests, demos, and documentation updates.

The post is intentionally simple. It has a title, description, tags, a custom excerpt, a Mastodon override, a Reddit override, and enough body text to demonstrate the Markdown renderer.
