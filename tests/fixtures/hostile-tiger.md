# Hostile fixture — sanitizer bait, never published

This file is pushed through the exact markdown pipeline the Tiger Style
guides use (`build.rs::render_markdown`) on every build. Everything
active in it must come out inert; everything legitimate must survive.

<script>alert("script block")</script>

<script src="https://evil.example/x.js"></script>

<img src="https://evil.example/x.png" onerror="alert('img handler')">

<div onclick="alert('div handler')">a raw div with a handler</div>

<iframe src="https://evil.example/frame"></iframe>

[link with a javascript scheme](javascript:alert('link scheme'))

[plain javascript link](javascript:alert)

[link with a data scheme](data:text/html;base64,PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==)

<details>

<summary><strong>Legitimate collapsible</strong></summary>

Body copy with **strong** text and a [safe link](https://example.com/docs)
that must both survive sanitization.

</details>

- [ ] a tasklist item whose checkbox must survive
