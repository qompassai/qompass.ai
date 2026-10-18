//! Batch 2 concept diagrams — Ghost, Beacon, Bunker, Current,
//! Dotfiles, NUR. Owned ids are listed in `IDS`; the registry in
//!
//! `diagrams.rs` delegates here. Follow the authoring contract in
//! that module's header: `gfx-*` classes only, text at 10.5px or
//! larger at viewBox scale, motion only via the stylesheet's
//! reduced-motion-gated classes, and every figure must read
//! correctly as a static diagram.

use leptos::prelude::*;

/// The diagram ids this module owns.
pub const IDS: &[&str] =
    &["beacon-mail", "bunker-cache", "current-runner", "dotfiles-links", "ghost-mail", "nur-index"];

/// The bare `<svg>` for one of this module's ids, or `None`.
pub fn diagram(id: &'static str) -> Option<AnyView> {
    match id {
        "beacon-mail" => Some(beacon_mail()),
        "bunker-cache" => Some(bunker_cache()),
        "current-runner" => Some(current_runner()),
        "dotfiles-links" => Some(dotfiles_links()),
        "ghost-mail" => Some(ghost_mail()),
        "nur-index" => Some(nur_index()),
        _ => None,
    }
}

/// Ghost: one NixOS module brings up the whole mail stack —
///
/// Postfix receives SMTP, Rspamd runs the SPF/DKIM/DMARC
/// authentication gates, Dovecot delivers by LMTP into the
/// maildir, and outbound mail leaves DKIM-signed by Rspamd.
///
/// Component roles per `default.nix` and `docs/quickstart.rst`.
fn ghost_mail() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 400"
            role="img"
            aria-label="Diagram: Ghost's mail stack — inbound SMTP reaches Postfix, passes Rspamd's SPF, DKIM and DMARC authentication gates, and is delivered by Dovecot LMTP into the maildir; outbound mail is DKIM-signed by Rspamd before it leaves."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "ONE NIXOS MODULE — THE WHOLE MAIL STACK"
            </text>
            <rect class="gfx-chip" x="8" y="48" width="120" height="64" rx="6" />
            <text class="gfx-text" x="68" y="76" text-anchor="middle">
                "INTERNET"
            </text>
            <text class="gfx-note" x="68" y="94" text-anchor="middle">
                "inbound SMTP"
            </text>
            <line class="gfx-arrow" x1="132" y1="80" x2="148" y2="80" />
            <circle class="gfx-pulse" cx="136" cy="80" r="3.5" />
            <rect class="gfx-panel" x="152" y="48" width="128" height="64" rx="6" />
            <text class="gfx-text" x="216" y="76" text-anchor="middle">
                "POSTFIX"
            </text>
            <text class="gfx-note" x="216" y="94" text-anchor="middle">
                "receive + submit"
            </text>
            <line class="gfx-arrow" x1="284" y1="80" x2="300" y2="80" />
            <rect class="gfx-hot" x="304" y="48" width="140" height="64" rx="6" />
            <text class="gfx-text" x="374" y="76" text-anchor="middle">
                "RSPAMD"
            </text>
            <text class="gfx-note" x="374" y="94" text-anchor="middle">
                "filter + sign"
            </text>
            <line class="gfx-arrow" x1="448" y1="80" x2="464" y2="80" />
            <rect class="gfx-panel" x="468" y="48" width="128" height="64" rx="6" />
            <text class="gfx-text" x="532" y="76" text-anchor="middle">
                "DOVECOT"
            </text>
            <text class="gfx-note" x="532" y="94" text-anchor="middle">
                "LMTP delivery"
            </text>
            <line class="gfx-arrow" x1="600" y1="80" x2="616" y2="80" />
            <rect class="gfx-chip" x="620" y="48" width="124" height="64" rx="6" />
            <text class="gfx-text" x="682" y="76" text-anchor="middle">
                "MAILDIR"
            </text>
            <text class="gfx-note" x="682" y="94" text-anchor="middle">
                "per-user store"
            </text>
            <text class="gfx-head" x="380" y="142" text-anchor="middle">
                "AUTHENTICATION GATES — ANSWERED FROM DNS TXT RECORDS"
            </text>
            <line class="gfx-arrow" x1="374" y1="116" x2="374" y2="152" />
            <circle class="gfx-pulse-y" cx="374" cy="120" r="3.5" />
            <rect class="gfx-chip" x="168" y="156" width="112" height="36" rx="6" />
            <text class="gfx-text" x="224" y="179" text-anchor="middle">
                "SPF"
            </text>
            <rect class="gfx-chip" x="324" y="156" width="112" height="36" rx="6" />
            <text class="gfx-text" x="380" y="179" text-anchor="middle">
                "DKIM"
            </text>
            <rect class="gfx-chip" x="480" y="156" width="112" height="36" rx="6" />
            <text class="gfx-text" x="536" y="179" text-anchor="middle">
                "DMARC"
            </text>
            <text class="gfx-note" x="380" y="216" text-anchor="middle">
                "greylisting — and antivirus, if enabled — run in the same rspamd pass"
            </text>
            <rect class="gfx-chip" x="8" y="240" width="120" height="64" rx="6" />
            <text class="gfx-text" x="68" y="268" text-anchor="middle">
                "SUBMISSION"
            </text>
            <text class="gfx-note" x="68" y="286" text-anchor="middle">
                "authenticated user"
            </text>
            <line class="gfx-arrow" x1="132" y1="272" x2="148" y2="272" />
            <rect class="gfx-panel" x="152" y="240" width="128" height="64" rx="6" />
            <text class="gfx-text" x="216" y="268" text-anchor="middle">
                "POSTFIX"
            </text>
            <text class="gfx-note" x="216" y="286" text-anchor="middle">
                "sender-login map"
            </text>
            <line class="gfx-arrow" x1="284" y1="272" x2="300" y2="272" />
            <circle
                class="gfx-pulse"
                cx="288"
                cy="272"
                r="3.5"
                style="animation-delay:0.9s"
            />
            <rect class="gfx-hot" x="304" y="240" width="140" height="64" rx="6" />
            <text class="gfx-text" x="374" y="268" text-anchor="middle">
                "DKIM-SIGN"
            </text>
            <text class="gfx-note" x="374" y="286" text-anchor="middle">
                "selector + key from config"
            </text>
            <line class="gfx-arrow" x1="448" y1="272" x2="464" y2="272" />
            <rect class="gfx-chip" x="468" y="240" width="128" height="64" rx="6" />
            <text class="gfx-text" x="532" y="268" text-anchor="middle">
                "INTERNET"
            </text>
            <text class="gfx-note" x="532" y="286" text-anchor="middle">
                "public key in DNS"
            </text>
            <rect class="gfx-badge" x="100" y="328" width="560" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="353" text-anchor="middle">
                "CLIENTS READ VIA IMAP — DOVECOT SERVES THE SAME MAILDIR"
            </text>
            <text class="gfx-note" x="380" y="390" text-anchor="middle">
                "the same module also runs nginx (ACME + web) and kresd (local DNS resolver)"
            </text>
        </svg>
    }
    .into_any()
}

/// Beacon: a single Go binary serves SMTP, IMAP, HTTPS
/// (webmail/admin/account) and ACME TLS issuance; the queue and
/// the data directory are the only moving parts outside it.
///
/// Surface per the beacon (mox-derived) `doc.go` and quickstart.
fn beacon_mail() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 336"
            role="img"
            aria-label="Diagram: Beacon is one binary — mail clients and remote servers reach its SMTP and IMAP listeners, its HTTPS listeners serve webmail and admin interfaces, ACME issues its TLS certificates, and a queue plus one data directory hold everything else."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "ONE GO BINARY — SMTP · IMAP · HTTPS · ACME"
            </text>
            <rect class="gfx-chip" x="8" y="48" width="132" height="64" rx="6" />
            <text class="gfx-text" x="74" y="76" text-anchor="middle">
                "MAIL CLIENTS"
            </text>
            <text class="gfx-note" x="74" y="94" text-anchor="middle">
                "IMAP / submission"
            </text>
            <rect class="gfx-chip" x="8" y="140" width="132" height="64" rx="6" />
            <text class="gfx-text" x="74" y="168" text-anchor="middle">
                "REMOTE SERVERS"
            </text>
            <text class="gfx-note" x="74" y="186" text-anchor="middle">
                "SMTP delivery in"
            </text>
            <line class="gfx-arrow" x1="144" y1="80" x2="160" y2="80" />
            <circle class="gfx-pulse" cx="148" cy="80" r="3.5" />
            <line class="gfx-arrow" x1="144" y1="172" x2="160" y2="172" />
            <rect class="gfx-panel" x="164" y="48" width="300" height="180" rx="6" />
            <text class="gfx-head" x="314" y="72" text-anchor="middle">
                "BEACON — ONE BINARY"
            </text>
            <rect class="gfx-chip" x="180" y="86" width="128" height="36" rx="6" />
            <text class="gfx-text" x="244" y="109" text-anchor="middle">
                "SMTP"
            </text>
            <rect class="gfx-chip" x="320" y="86" width="128" height="36" rx="6" />
            <text class="gfx-text" x="384" y="109" text-anchor="middle">
                "IMAP"
            </text>
            <rect class="gfx-chip" x="180" y="132" width="128" height="36" rx="6" />
            <text class="gfx-text" x="244" y="155" text-anchor="middle">
                "HTTPS"
            </text>
            <rect class="gfx-hot" x="320" y="132" width="128" height="36" rx="6" />
            <text class="gfx-text" x="384" y="155" text-anchor="middle">
                "ACME / TLS"
            </text>
            <text class="gfx-note" x="314" y="192" text-anchor="middle">
                "webmail · admin · account interfaces over HTTPS"
            </text>
            <text class="gfx-note" x="314" y="212" text-anchor="middle">
                "certificates requested via ACME (Let's Encrypt)"
            </text>
            <line class="gfx-arrow" x1="468" y1="86" x2="492" y2="86" />
            <rect class="gfx-panel" x="496" y="48" width="256" height="76" rx="6" />
            <text class="gfx-text" x="624" y="80" text-anchor="middle">
                "QUEUE"
            </text>
            <text class="gfx-note" x="624" y="100" text-anchor="middle">
                "retries · delivery to remote SMTP"
            </text>
            <line class="gfx-arrow" x1="468" y1="172" x2="492" y2="172" />
            <circle
                class="gfx-pulse"
                cx="472"
                cy="172"
                r="3.5"
                style="animation-delay:0.8s"
            />
            <rect class="gfx-panel" x="496" y="140" width="256" height="76" rx="6" />
            <text class="gfx-text" x="624" y="172" text-anchor="middle">
                "DATA DIRECTORY"
            </text>
            <text class="gfx-note" x="624" y="192" text-anchor="middle">
                "accounts · messages · ACME keys"
            </text>
            <rect class="gfx-badge" x="80" y="252" width="600" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="277" text-anchor="middle">
                "QUICKSTART WRITES CONFIG + DNS RECORDS — SERVE RUNS EVERYTHING"
            </text>
            <text class="gfx-note" x="380" y="322" text-anchor="middle">
                "derived from the mox codebase — no separate MTA, IMAP server, or web stack to wire"
            </text>
        </svg>
    }
    .into_any()
}

/// Bunker: the signed binary-cache flow — the client uploads only
///
/// the paths the server lacks, the server chunks, compresses, and
/// deduplicates NARs, and a fetching client verifies the narinfo
/// Ed25519 signature before substituting anything (fail-closed).
///
/// Chunk parameters per `server/src/config-template.toml`.
fn bunker_cache() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 392"
            role="img"
            aria-label="Diagram: Bunker's cache flow — a client pushes only missing store paths, the server signs the narinfo with Ed25519 and stores chunked, compressed, deduplicated NARs; a fetching Nix client verifies the signature before substituting, and refuses an untrusted cache."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "SIGNED UPLOAD → CHUNKED STORE → VERIFIED FETCH"
            </text>
            <rect class="gfx-chip" x="8" y="48" width="132" height="64" rx="6" />
            <text class="gfx-text" x="74" y="76" text-anchor="middle">
                "LOCAL NIX STORE"
            </text>
            <text class="gfx-note" x="74" y="94" text-anchor="middle">
                "paths + NARs"
            </text>
            <line class="gfx-arrow" x1="144" y1="80" x2="160" y2="80" />
            <circle class="gfx-pulse" cx="148" cy="80" r="3.5" />
            <rect class="gfx-panel" x="164" y="48" width="160" height="64" rx="6" />
            <text class="gfx-text" x="244" y="76" text-anchor="middle">
                "bunker push"
            </text>
            <text class="gfx-note" x="244" y="94" text-anchor="middle">
                "missing paths only"
            </text>
            <line class="gfx-arrow" x1="328" y1="80" x2="344" y2="80" />
            <rect class="gfx-hot" x="348" y="48" width="180" height="64" rx="6" />
            <text class="gfx-text" x="438" y="76" text-anchor="middle">
                "SIGN NARINFO"
            </text>
            <text class="gfx-note" x="438" y="94" text-anchor="middle">
                "Ed25519 cache key"
            </text>
            <line class="gfx-arrow" x1="532" y1="80" x2="548" y2="80" />
            <rect class="gfx-panel" x="552" y="48" width="200" height="64" rx="6" />
            <text class="gfx-text" x="652" y="76" text-anchor="middle">
                "CACHE — bunkerd"
            </text>
            <text class="gfx-note" x="652" y="94" text-anchor="middle">
                "serves narinfo + NAR"
            </text>
            <text class="gfx-head" x="380" y="146" text-anchor="middle">
                "INSIDE THE SERVER"
            </text>
            <rect class="gfx-panel" x="164" y="160" width="160" height="64" rx="6" />
            <text class="gfx-text" x="244" y="184" text-anchor="middle">
                "CHUNKING"
            </text>
            <text class="gfx-note" x="244" y="202" text-anchor="middle">
                "content-defined · 16/64/256 KiB"
            </text>
            <line class="gfx-arrow" x1="328" y1="192" x2="344" y2="192" />
            <rect class="gfx-panel" x="348" y="160" width="180" height="64" rx="6" />
            <text class="gfx-text" x="438" y="184" text-anchor="middle">
                "COMPRESS + DEDUP"
            </text>
            <text class="gfx-note" x="438" y="202" text-anchor="middle">
                "zstd · identical chunks stored once"
            </text>
            <line class="gfx-arrow" x1="532" y1="192" x2="548" y2="192" />
            <circle
                class="gfx-pulse"
                cx="536"
                cy="192"
                r="3.5"
                style="animation-delay:0.7s"
            />
            <rect class="gfx-panel" x="552" y="160" width="200" height="64" rx="6" />
            <text class="gfx-text" x="652" y="184" text-anchor="middle">
                "STORAGE + INDEX"
            </text>
            <text class="gfx-note" x="652" y="202" text-anchor="middle">
                "local or S3 · database index"
            </text>
            <rect class="gfx-chip" x="8" y="248" width="132" height="64" rx="6" />
            <text class="gfx-text" x="74" y="276" text-anchor="middle">
                "NIX CLIENT"
            </text>
            <text class="gfx-note" x="74" y="294" text-anchor="middle">
                "fetching a path"
            </text>
            <line class="gfx-arrow" x1="144" y1="280" x2="160" y2="280" />
            <rect class="gfx-panel" x="164" y="248" width="160" height="64" rx="6" />
            <text class="gfx-text" x="244" y="276" text-anchor="middle">
                "FETCH"
            </text>
            <text class="gfx-note" x="244" y="294" text-anchor="middle">
                "narinfo + NAR data"
            </text>
            <line class="gfx-arrow" x1="328" y1="280" x2="344" y2="280" />
            <rect class="gfx-hot" x="348" y="248" width="180" height="64" rx="6" />
            <text class="gfx-text" x="438" y="276" text-anchor="middle">
                "VERIFY SIGNATURE"
            </text>
            <text class="gfx-note" x="438" y="294" text-anchor="middle">
                "against the trusted public key"
            </text>
            <line class="gfx-arrow" x1="532" y1="280" x2="548" y2="280" />
            <rect class="gfx-panel" x="552" y="248" width="200" height="64" rx="6" />
            <text class="gfx-text" x="652" y="276" text-anchor="middle">
                "SUBSTITUTE — OR REFUSE"
            </text>
            <text class="gfx-note" x="652" y="294" text-anchor="middle">
                "an untrusted cache is never used"
            </text>
            <rect class="gfx-badge" x="110" y="336" width="540" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="361" text-anchor="middle">
                "TRUST IS CHECKED ON EVERY FETCH — NOT ASSUMED FROM THE URL"
            </text>
        </svg>
    }
    .into_any()
}

/// Current: the local check runner's lifecycle — a repo declares
///
/// checks in `.current.toml`, the runner executes them one at a
/// time under hard bounds, and appends one JSON object per check
/// to a local results log. The project's own design, per
///
/// `runner/README.md` and the runner source; nothing is sent
/// anywhere.
fn current_runner() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 400"
            role="img"
            aria-label="Diagram: Current's check lifecycle — a .current.toml manifest declares checks, the runner resolves the repo and runs the checks one at a time with no shell, a hard timeout, and a 1 MiB per-stream log cap, appending one JSON object per check to a local JSONL ledger; exit code 0, 1, or 2 reports the outcome."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "DECLARED CHECKS — ONE AT A TIME — JSONL LEDGER"
            </text>
            <rect class="gfx-panel" x="8" y="48" width="168" height="64" rx="6" />
            <text class="gfx-text" x="92" y="76" text-anchor="middle">
                ".current.toml"
            </text>
            <text class="gfx-note" x="92" y="94" text-anchor="middle">
                "[[checks]] argv · workdir · timeout"
            </text>
            <line class="gfx-arrow" x1="180" y1="80" x2="196" y2="80" />
            <circle class="gfx-pulse" cx="184" cy="80" r="3.5" />
            <rect class="gfx-panel" x="200" y="48" width="168" height="64" rx="6" />
            <text class="gfx-text" x="284" y="76" text-anchor="middle">
                "RESOLVE REPO"
            </text>
            <text class="gfx-note" x="284" y="94" text-anchor="middle">
                "path, or name under repo roots"
            </text>
            <line class="gfx-arrow" x1="372" y1="80" x2="388" y2="80" />
            <rect class="gfx-hot" x="392" y="48" width="168" height="64" rx="6" />
            <text class="gfx-text" x="476" y="76" text-anchor="middle">
                "RUN — ONE AT A TIME"
            </text>
            <text class="gfx-note" x="476" y="94" text-anchor="middle">
                "manifest order · no shell"
            </text>
            <line class="gfx-arrow" x1="564" y1="80" x2="580" y2="80" />
            <rect class="gfx-panel" x="584" y="48" width="168" height="64" rx="6" />
            <text class="gfx-text" x="668" y="76" text-anchor="middle">
                "JSONL LEDGER"
            </text>
            <text class="gfx-note" x="668" y="94" text-anchor="middle">
                "one object per check"
            </text>
            <rect class="gfx-panel" x="104" y="140" width="552" height="110" rx="6" />
            <text class="gfx-head" x="380" y="164" text-anchor="middle">
                "EVERY CHECK RUNS BOUNDED"
            </text>
            <text class="gfx-text" x="124" y="190">
                "direct argv — no shell; workdir must resolve inside the repo root"
            </text>
            <text class="gfx-text" x="124" y="214">
                "hard timeout kills and reaps the child"
            </text>
            <text class="gfx-text" x="124" y="238">
                "stdout / stderr drained separately — 1 MiB kept per stream, truncation recorded"
            </text>
            <rect class="gfx-chip" x="104" y="278" width="168" height="36" rx="6" />
            <text class="gfx-text" x="188" y="301" text-anchor="middle">
                "exit 0 — all passed"
            </text>
            <rect class="gfx-chip" x="296" y="278" width="224" height="36" rx="6" />
            <text class="gfx-text" x="408" y="301" text-anchor="middle">
                "exit 1 — failed or timed out"
            </text>
            <rect class="gfx-chip" x="544" y="278" width="208" height="36" rx="6" />
            <text class="gfx-text" x="648" y="301" text-anchor="middle">
                "exit 2 — usage / manifest error"
            </text>
            <rect class="gfx-badge" x="110" y="338" width="540" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="363" text-anchor="middle">
                "RESULTS STAY LOCAL — NO TELEMETRY, NOTHING IS SENT ANYWHERE"
            </text>
        </svg>
    }
    .into_any()
}

/// Dotfiles: the deployment flow — repo files are mapped by the
///
/// `qonfig.yaml` link plan onto their `$HOME` targets under the
/// XDG convention, while the Nix flake's `userEnvironment`
/// builds the tool set. Counts and targets per `qonfig.yaml`
///
/// and `flake.nix`.
fn dotfiles_links() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 384"
            role="img"
            aria-label="Diagram: Dotfiles deployment — files in the repository pass through the qonfig.yaml link plan (link, clean, create, shell steps) into their $HOME targets under $XDG_CONFIG_HOME, while the Nix flake's userEnvironment package set is built alongside."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "REPO FILES → LINK PLAN → $HOME"
            </text>
            <rect class="gfx-panel" x="8" y="48" width="220" height="176" rx="6" />
            <text class="gfx-head" x="118" y="72" text-anchor="middle">
                "DOTFILES REPO"
            </text>
            <text class="gfx-text" x="24" y="100">
                ".config/ — 800+ app dirs"
            </text>
            <text class="gfx-text" x="24" y="126">
                "bashrc · zshrc"
            </text>
            <text class="gfx-text" x="24" y="152">
                "tmux · sesh · neovide"
            </text>
            <text class="gfx-text" x="24" y="178">
                "Diver · zed · kitty (force)"
            </text>
            <text class="gfx-note" x="24" y="204">
                "flake.nix alongside"
            </text>
            <line class="gfx-arrow" x1="232" y1="136" x2="252" y2="136" />
            <circle class="gfx-pulse" cx="236" cy="136" r="3.5" />
            <rect class="gfx-panel" x="256" y="48" width="240" height="176" rx="6" />
            <text class="gfx-head" x="376" y="72" text-anchor="middle">
                "QONFIG.YAML LINK PLAN"
            </text>
            <text class="gfx-text" x="272" y="100">
                "link — 25 targets"
            </text>
            <text class="gfx-text" x="272" y="126">
                "create parents · relink stale"
            </text>
            <text class="gfx-text" x="272" y="152">
                "clean — ~ and ~/.config"
            </text>
            <text class="gfx-text" x="272" y="178">
                "create — ~/Downloads"
            </text>
            <text class="gfx-text" x="272" y="204">
                "shell — submodule + ssh setup"
            </text>
            <line class="gfx-arrow" x1="500" y1="136" x2="520" y2="136" />
            <circle
                class="gfx-pulse"
                cx="504"
                cy="136"
                r="3.5"
                style="animation-delay:0.8s"
            />
            <rect class="gfx-panel" x="524" y="48" width="228" height="176" rx="6" />
            <text class="gfx-head" x="638" y="72" text-anchor="middle">
                "$HOME TARGETS"
            </text>
            <text class="gfx-text" x="540" y="100">
                "~/.config/tmux/tmux.conf"
            </text>
            <text class="gfx-text" x="540" y="126">
                "~/.config/diver"
            </text>
            <text class="gfx-text" x="540" y="152">
                "~/.wezterm.lua"
            </text>
            <text class="gfx-text" x="540" y="178">
                "~/.bashrc_user · ~/.zshrc_user"
            </text>
            <text class="gfx-note" x="540" y="204">
                "macOS-only links gated by if:"
            </text>
            <rect class="gfx-panel" x="8" y="248" width="744" height="64" rx="6" />
            <text class="gfx-head" x="380" y="274" text-anchor="middle">
                "NIX FLAKE — userEnvironment"
            </text>
            <text class="gfx-note" x="380" y="296" text-anchor="middle">
                "buildEnv package set per system (x86_64-linux full · aarch64-linux reduced) — quickstart: nix flake update, then nix develop"
            </text>
            <rect class="gfx-badge" x="100" y="328" width="560" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="353" text-anchor="middle">
                "TARGETS LIVE UNDER $XDG_CONFIG_HOME (~/.config) BY CONVENTION"
            </text>
        </svg>
    }
    .into_any()
}

/// NUR: the index flow — `repos.json` registers 581 community
///
/// repositories; evaluating `default.nix` resolves each into the
/// `nur.repos.<name>.<package>` namespace at pinned revisions,
/// and packages build from source, unreviewed by Nixpkgs.
fn nur_index() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 336"
            role="img"
            aria-label="Diagram: NUR's index flow — repos.json registers 581 community repositories including Qompass AI's own; evaluation resolves them into the nur.repos attribute namespace at locked revisions; consumers install packages that build from source and are not reviewed by Nixpkgs members."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "ONE INDEX — 581 COMMUNITY REPOSITORIES"
            </text>
            <rect class="gfx-panel" x="8" y="48" width="230" height="168" rx="6" />
            <text class="gfx-head" x="123" y="72" text-anchor="middle">
                "repos.json"
            </text>
            <text class="gfx-text" x="24" y="100">
                "each entry: a git url"
            </text>
            <text class="gfx-text" x="24" y="126">
                "file · branch · submodules"
            </text>
            <rect class="gfx-hot" x="24" y="140" width="198" height="40" rx="6" />
            <text class="gfx-text" x="123" y="164" text-anchor="middle">
                "qompassai → nur-packages"
            </text>
            <text class="gfx-note" x="24" y="202">
                "this fork's own entry"
            </text>
            <line class="gfx-arrow" x1="242" y1="132" x2="258" y2="132" />
            <circle class="gfx-pulse" cx="246" cy="132" r="3.5" />
            <rect class="gfx-panel" x="262" y="48" width="228" height="168" rx="6" />
            <text class="gfx-head" x="376" y="72" text-anchor="middle">
                "EVALUATION"
            </text>
            <text class="gfx-text" x="278" y="100">
                "default.nix imports each repo"
            </text>
            <text class="gfx-text" x="278" y="126">
                "repos // repoOverrides"
            </text>
            <text class="gfx-text" x="278" y="152">
                "nur.repos.<name>.<pkg>"
            </text>
            <text class="gfx-note" x="278" y="178">
                "repos.json.lock pins revisions"
            </text>
            <text class="gfx-note" x="278" y="202">
                "updated by bin/nur update"
            </text>
            <line class="gfx-arrow" x1="494" y1="132" x2="510" y2="132" />
            <circle
                class="gfx-pulse"
                cx="498"
                cy="132"
                r="3.5"
                style="animation-delay:0.8s"
            />
            <rect class="gfx-panel" x="514" y="48" width="238" height="168" rx="6" />
            <text class="gfx-head" x="633" y="72" text-anchor="middle">
                "YOUR PACKAGE SET"
            </text>
            <text class="gfx-text" x="530" y="100">
                "overlay / legacyPackages"
            </text>
            <text class="gfx-text" x="530" y="126">
                "NixOS + Home Manager modules"
            </text>
            <text class="gfx-text" x="530" y="152">
                "nix-shell -p nur.repos.…"
            </text>
            <text class="gfx-note" x="530" y="178">
                "find packages at"
            </text>
            <text class="gfx-note" x="530" y="202">
                "nur.nix-community.org"
            </text>
            <rect class="gfx-badge" x="100" y="240" width="560" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="265" text-anchor="middle">
                "PACKAGES BUILD FROM SOURCE — NOT REVIEWED BY NIXPKGS MEMBERS"
            </text>
            <text class="gfx-note" x="380" y="312" text-anchor="middle">
                "Qompass AI's fork of nix-community/NUR — the index is upstream's design, the entry above is Qompass's"
            </text>
        </svg>
    }
    .into_any()
}
