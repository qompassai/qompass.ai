//! Four Cardinal Services on the home page, as a bearing compass.
//!
//! One bearing (degrees clockwise from north) drives everything: the
//! card turns under a fixed lubber line, and the readout blends the
//! solutions of the two cardinals nearest the bearing, by proximity
//! weight, into one list. The cardinal cards on /solutions list every
//! solution in plain prerendered HTML — that list is the guarantee,
//! complete and readable with JavaScript off; the compass is
//! presentation over the same table.
//!
//! The instrument mirrors the /about identity card (src/pages/identity.rs):
//! inline SVG, fixed lubber, the card angle written to a CSS custom
//! property and never read back, roles switched on only after
//! hydration, the drift paused on hover or focus and stopped by the
//! first interaction, nothing moving under reduced motion. It mirrors
//! rather than shares that code: the identity card turns between
//! discrete ticks, this one turns through a continuous bearing.

use crate::project_data::Benchmark;
use leptos::prelude::*;

/// One of the four services, centered on its cardinal bearing.
pub struct Cardinal {
    pub mark: &'static str,
    pub name: &'static str,
    pub brief: &'static str,
    pub center: u16,
}

pub const CARDINALS: [Cardinal; 4] = [
    Cardinal {
        mark: "N — NORTH",
        name: "Security",
        brief: "NIST-compliant security solutions — post-quantum cryptography, \
            red-team tooling, and privacy infrastructure, built in the open.",
        center: 0,
    },
    Cardinal {
        mark: "E — EAST",
        name: "Education",
        brief: "Empowering education solutions — learning tools that live in the \
            editor, and PathFinders, our internship program for new builders.",
        center: 90,
    },
    Cardinal {
        mark: "S — SOUTH",
        name: "Deployment",
        brief: "Quality deployment solutions — dotfiles, configuration, and \
            release tooling that carry a project from first checkout to \
            production.",
        center: 180,
    },
    Cardinal {
        mark: "W — WEST",
        name: "Infrastructure",
        brief: "Efficient infrastructure solutions — Nix packaging, a signed binary \
            cache, and the network plumbing the rest of the fleet runs on.",
        center: 270,
    },
];

const N: usize = 0;
const E: usize = 1;
const S: usize = 2;
const W: usize = 3;

const GITHUB: &str = "https://github.com/qompassai/";

/// The three classes the fleet divides into, for the nav dropdown and
/// the class index pages: applications, MCP servers, and packages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    App,
    Mcp,
    Package,
}

impl Class {
    /// The class label, as the nav and the index pages name it.
    pub const fn label(self) -> &'static str {
        match self {
            Self::App => "Apps",
            Self::Mcp => "MCP Servers",
            Self::Package => "Packages",
        }
    }

    /// The class index page's route.
    pub const fn route(self) -> &'static str {
        match self {
            Self::App => "/solutions/apps",
            Self::Mcp => "/solutions/mcp-servers",
            Self::Package => "/solutions/packages",
        }
    }
}

/// One published solution under one cardinal. `slug` is its page,
/// `/solutions/<slug>`; `href` is where it lives (GitHub, `LuaRocks`, or
/// that page itself for a solution that lives on this site).
pub struct Solution {
    pub name: &'static str,
    pub slug: &'static str,
    pub href: &'static str,
    pub cardinal: usize,
    pub class: Class,
    pub version: Option<&'static str>,
    pub blurb: &'static str,
}

impl Solution {
    /// Where the version points: the GitHub release tagged `v<version>`
    /// (any `#fragment` on the href dropped), or the `LuaRocks` module page.
    pub fn version_href(&self) -> Option<String> {
        let version = self.version?;
        Some(self.href.strip_prefix(GITHUB).map_or_else(
            || self.href.to_owned(),
            |rest| {
                let repo = rest.split('#').next().unwrap_or(rest);
                format!("{GITHUB}{repo}/releases/tag/v{version}")
            },
        ))
    }

    /// The project mark. Every solution carries one of the square
    /// emblems in `public/img/projects/`, named for its slug — except
    /// the phlow MCP server, which shares phlow's emblem.
    pub const fn mark(&self) -> Option<&'static str> {
        slug_mark(self.slug)
    }
}

/// The emblem path for a slug: `/img/projects/<slug>.webp`, spelled out
/// per solution so the mapping stays a compile-time constant table.
const fn slug_mark(slug: &str) -> Option<&'static str> {
    Some(match slug.as_bytes() {
        b"phlow-mcp-server" => "/img/projects/phlow.webp",
        b"qompass-hold" => "/img/projects/qompass-hold.webp",
        b"qssl" => "/img/projects/qssl.webp",
        b"radar" => "/img/projects/radar.webp",
        b"redteam" => "/img/projects/redteam.webp",
        b"tor" => "/img/projects/tor.webp",
        b"vongola" => "/img/projects/vongola.webp",
        b"fathom" => "/img/projects/fathom.webp",
        b"diver-security" => "/img/projects/diver-security.webp",
        b"blaze-ts-nvim" => "/img/projects/blaze-ts-nvim.webp",
        b"blaze-nvim" => "/img/projects/blaze-nvim.webp",
        b"diver" => "/img/projects/diver.webp",
        b"pathfinders" => "/img/projects/pathfinders.webp",
        b"qluv" => "/img/projects/qluv.webp",
        b"rose-nvim" => "/img/projects/rose-nvim.webp",
        b"tuxtype" => "/img/projects/tuxtype.webp",
        b"unreal" => "/img/projects/unreal.webp",
        b"eigenstyle" => "/img/projects/eigenstyle.webp",
        b"blaze-nvim-luarocks" => "/img/projects/blaze-nvim-luarocks.webp",
        b"diver-research" => "/img/projects/diver-research.webp",
        b"telco-admin-quizzes" => "/img/projects/telco-admin-quizzes.webp",
        b"azimuth" => "/img/projects/azimuth.webp",
        b"current" => "/img/projects/current.webp",
        b"dotfiles" => "/img/projects/dotfiles.webp",
        b"qonfig" => "/img/projects/qonfig.webp",
        b"qudaz" => "/img/projects/qudaz.webp",
        b"qxet" => "/img/projects/qxet.webp",
        b"rose" => "/img/projects/rose.webp",
        b"lumen" => "/img/projects/lumen.webp",
        b"diver-ai" => "/img/projects/diver-ai.webp",
        b"diver-comms" => "/img/projects/diver-comms.webp",
        b"diver-config" => "/img/projects/diver-config.webp",
        b"diver-dap" => "/img/projects/diver-dap.webp",
        b"diver-dev" => "/img/projects/diver-dev.webp",
        b"diver-games" => "/img/projects/diver-games.webp",
        b"diver-lintfmt" => "/img/projects/diver-lintfmt.webp",
        b"diver-utils" => "/img/projects/diver-utils.webp",
        b"hyprlua" => "/img/projects/hyprlua.webp",
        b"beacon" => "/img/projects/beacon.webp",
        b"bunker" => "/img/projects/bunker.webp",
        b"debian" => "/img/projects/debian.webp",
        b"network" => "/img/projects/network.webp",
        b"nur" => "/img/projects/nur.webp",
        b"qjp" => "/img/projects/qjp.webp",
        b"moonwalk" => "/img/projects/moonwalk.webp",
        _ => return None,
    })
}

const fn sol(
    name: &'static str,
    slug: &'static str,
    href: &'static str,
    cardinal: usize,
    class: Class,
    version: Option<&'static str>,
    blurb: &'static str,
) -> Solution {
    Solution { name, slug, href, cardinal, class, version, blurb }
}

/// The `v1.00` release, published 2026-10-15 on every GitHub repository.
const V1: Option<&str> = Some("1.00");
/// The phaedrusflow `LuaRocks` releases, verified 2026-10-15.
const ROCK: Option<&str> = Some("0.1.0-1");

/// Every solution on the compass, with its page slug and version.
///
/// Per cardinal: the org profile's set first, in its order, then the
/// published additions — MCP servers and the phaedrusflow `LuaRocks`
/// modules (crates.io: none published). rose.nvim has no version: the
/// repository is archived (GitHub refuses it a release) and rose now
/// lives inside Diver. The telco-admin practice quizzes live on this
/// site — their `href` is their own page — and carry no version.
pub const SOLUTIONS: [Solution; 45] = [
    sol("Qompass-Hold", "qompass-hold", "https://github.com/qompassai/qompass-hold", N, Class::App, V1, "Qompass Secret-Service."),
    sol("Qssl", "qssl", "https://github.com/qompassai/qssl", N, Class::App, V1, "Qompass AI SSL solution."),
    sol("Radar", "radar", "https://github.com/qompassai/radar", N, Class::App, V1, "Risk Analysis Diagnostic Assessment Resolver (R.A.D.A.R)."),
    sol("Redteam", "redteam", "https://github.com/qompassai/redteam", N, Class::App, V1, "Qompass AI on red teaming."),
    sol("Tor", "tor", "https://github.com/qompassai/tor", N, Class::App, V1, "Qompass AI Tor setup."),
    sol("Vongola", "vongola", "https://github.com/qompassai/vongola", N, Class::App, V1, "A secure, performant reverse-proxy server for your deep tech needs."),
    sol("Fathom", "fathom", "https://github.com/qompassai/fathom", N, Class::Mcp, V1, "Spec-led Rust port of REA — a reverse-engineering tool provider for agents (MCP/A2A), with evidence-carrying findings and a fail-closed policy gate."),
    sol("Diver-Security", "diver-security", "https://luarocks.org/modules/phaedrusflow/diver-security", N, Class::Package, ROCK, "Diver security toolkit: security.* RCE/MiTM/zombie/supply-chain guards, red/blue suites, pass integration, Ghidra bridge, PQC tokens."),
    sol("Blaze-TS.nvim", "blaze-ts-nvim", "https://github.com/qompassai/blaze-ts.nvim", E, Class::App, V1, "A tree-sitter grammar for the Mojo programming language, packaged as a Neovim plugin."),
    sol("Blaze.nvim", "blaze-nvim", "https://github.com/qompassai/blaze.nvim", E, Class::App, V1, "🔥 for Neovim."),
    sol("Diver", "diver", "https://github.com/qompassai/Diver", E, Class::App, V1, "Your blazingly fast everything editor."),
    sol("PathFinders", "pathfinders", "https://github.com/qompassai/PathFinders", E, Class::App, V1, "The Qompass intern program."),
    sol("Qluv", "qluv", "https://github.com/qompassai/qluv", E, Class::App, V1, "Qompass Lua version manager."),
    sol("Rose.nvim", "rose-nvim", "https://github.com/qompassai/rose.nvim", E, Class::App, None, "The archived Neovim front end for Rose; Rose now lives inside Diver."),
    sol("Tuxtype", "tuxtype", "https://github.com/qompassai/tuxtype", E, Class::App, V1, "Qompass AI on TuxType."),
    sol("Unreal", "unreal", "https://github.com/qompassai/unreal", E, Class::App, V1, "Qompass AI Unreal tooling."),
    sol("Eigenstyle", "eigenstyle", "https://github.com/qompassai/eigenstyle", E, Class::Mcp, V1, "Your voice profile as a tool — a Rust workspace (MCP server, TUI, and GPUI GUI) over a measured writing fingerprint."),
    sol("Blaze.nvim (LuaRocks)", "blaze-nvim-luarocks", "https://luarocks.org/modules/phaedrusflow/blaze.nvim", E, Class::Package, Some("1.0-1"), "🔥 for AI — the blaze.nvim plugin as a LuaRocks module."),
    sol("Diver-Research", "diver-research", "https://luarocks.org/modules/phaedrusflow/diver-research", E, Class::Package, ROCK, "Diver research workflow: research.* journal registry, ORCID, OpenAlex, Web of Science, Zenodo, deadlines, watchlist, templates, Zotero."),
    sol("Telco-Admin Practice Quizzes", "telco-admin-quizzes", "/solutions/telco-admin-quizzes", E, Class::App, None, "Telco-admin practice quizzes with encrypted answer keys, hosted on this site."),
    sol("Moonwalk", "moonwalk", "https://github.com/qompassai/moonwalk", E, Class::App, V1, "A cross-platform Lua debugger and Debug Adapter Protocol server — a native, plugin-free Neovim debugging workflow, compatible with VS Code and other DAP clients."),
    sol("Azimuth", "azimuth", "https://github.com/qompassai/Azimuth", S, Class::App, V1, "Your helpful AI navigator."),
    sol("Current", "current", "https://github.com/qompassai/current", S, Class::App, V1, "Qompass AI Current: your self-hosted AI runner."),
    sol("Dotfiles", "dotfiles", "https://github.com/qompassai/dotfiles", S, Class::App, V1, "XDG dotfiles for the Qompass AI development environment — a Qonfig link map paired with a Nix flake user environment."),
    sol("Qonfig", "qonfig", "https://github.com/qompassai/qonfig", S, Class::App, V1, "A quality dotfile manager."),
    sol("Qudaz", "qudaz", "https://github.com/qompassai/qudaz", S, Class::App, V1, "Qompass AI CUDA library for Zig."),
    sol("Qxet", "qxet", "https://github.com/qompassai/qxet", S, Class::App, V1, "QXet: Quality AI eXchange — content-addressed storage for large binary artifacts."),
    sol("Rose", "rose", "https://github.com/qompassai/rose", S, Class::App, V1, "Qompass AI's fork of Ollama, the local model server — upstream releases with the fork delta re-applied, serving local specialists on local hardware."),
    sol("Lumen", "lumen", "https://github.com/qompassai/lumen", S, Class::Mcp, V1, "Lux in motu — an MCP server for Aseprite sprite artistry with Bevy live-world interop."),
    sol("Phlow MCP Server", "phlow-mcp-server", "https://github.com/qompassai/phlow#mcp-wire-contract", S, Class::Mcp, V1, "Phlow's MCP server surface: one safe runtime shared with the CLI and TUI, over the MCP wire contract."),
    sol("Diver-AI", "diver-ai", "https://luarocks.org/modules/phaedrusflow/diver-ai", S, Class::Package, ROCK, "Diver AI suite: ai.* harness (registry/supervisor/policy), MCP, ACP, A2A, herd, rose integration, media, recon, plus salesforce.mcp."),
    sol("Diver-Comms", "diver-comms", "https://luarocks.org/modules/phaedrusflow/diver-comms", S, Class::Package, ROCK, "Diver mail tooling: comms.* clients (aerc, neomutt), mail and MIME helpers."),
    sol("Diver-Config", "diver-config", "https://luarocks.org/modules/phaedrusflow/diver-config", S, Class::Package, ROCK, "Diver config library modules: config.ui.* (UI library: icons, image rendering, themes, virtual text) and config.data.* (database adapters)."),
    sol("Diver-DAP", "diver-dap", "https://luarocks.org/modules/phaedrusflow/diver-dap", S, Class::Package, ROCK, "Diver debug-adapter suite: dap.* session engine, adapters, store, lifecycle and integrations (Moonwalk, phlow)."),
    sol("Diver-Dev", "diver-dev", "https://luarocks.org/modules/phaedrusflow/diver-dev", S, Class::Package, ROCK, "Diver developer tooling: dev.* BSP servers, SCIP indexers, browser (BiDi/CDP), apps, Salesforce suite, git/jj/cargo runners."),
    sol("Diver-Games", "diver-games", "https://luarocks.org/modules/phaedrusflow/diver-games", S, Class::Package, ROCK, "Diver game-engine tooling: games.* for Aseprite, Blender, Godot, LÖVE, Redot, TIC-80, Unity, Unreal and Voidsprite, plus shared character kit."),
    sol("Diver-Lintfmt", "diver-lintfmt", "https://luarocks.org/modules/phaedrusflow/diver-lintfmt", S, Class::Package, ROCK, "Diver tool catalogs: formatters.* (138 formatter adapters + engine) and linters.* (185 linter adapters + engine). One adapter per file."),
    sol("Diver-Utils", "diver-utils", "https://luarocks.org/modules/phaedrusflow/diver-utils", S, Class::Package, ROCK, "Diver foundations: shared utility library (utils.*), tools.*, plus the shared primitives other diver rocks build on."),
    sol("Hyprlua", "hyprlua", "https://luarocks.org/modules/phaedrusflow/hyprlua", S, Class::Package, ROCK, "LuaCATS annotations for the Hyprland 0.55+ Lua config API."),
    sol("Beacon", "beacon", "https://github.com/qompassai/beacon", W, Class::App, V1, "Beacon: a quality e-mail solution."),
    sol("Bunker", "bunker", "https://github.com/qompassai/bunker", W, Class::App, V1, "An Attic-based Nix binary cache with hybrid Ed25519 + ML-DSA-65 narinfo signing."),
    sol("Debian", "debian", "https://github.com/qompassai/debian", W, Class::App, V1, "Debian Linux for stable servers — the Qompass AI base configuration."),
    sol("Network", "network", "https://github.com/qompassai/network", W, Class::App, V1, "Qompass AI network tooling."),
    sol("NUR", "nur", "https://github.com/qompassai/NUR", W, Class::App, V1, "Qompass AI's Nix user repository — the qompassai package set, built from source."),
    sol("Qjp", "qjp", "https://github.com/qompassai/qjp", W, Class::App, V1, "NixOS module for NVIDIA Jetson devices."),
];

/// The solution whose page is `/solutions/<slug>`, if any.
pub fn solution_page(slug: &str) -> Option<&'static Solution> {
    SOLUTIONS.iter().find(|s| s.slug == slug)
}

/// Primary references for a generated solution page (the MCP servers
///
/// and the `LuaRocks` packages carry no guide TOML, so their citations
/// live here): `(title, url, note)` triples the page renders as its
/// REFERENCES section. Every URL is liveness-checked when the list
///
/// changes; a slug with no entry renders no section.
pub fn solution_references(slug: &str) -> &'static [(&'static str, &'static str, &'static str)] {
    match slug {
        "blaze-nvim-luarocks" => &[
            (
                "LuaRocks — the blaze.nvim rock",
                "https://luarocks.org/modules/phaedrusflow/blaze.nvim",
                "The Mojo language support plugin for Neovim, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "Blaze ships inside the Diver distribution as its Mojo lane.",
            ),
        ],
        "diver-ai" => &[
            (
                "LuaRocks — the diver-ai rock",
                "https://luarocks.org/modules/phaedrusflow/diver-ai",
                "AI tooling lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "diver-comms" => &[
            (
                "LuaRocks — the diver-comms rock",
                "https://luarocks.org/modules/phaedrusflow/diver-comms",
                "Communication tooling lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "diver-config" => &[
            (
                "LuaRocks — the diver-config rock",
                "https://luarocks.org/modules/phaedrusflow/diver-config",
                "Configuration lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "diver-dap" => &[
            (
                "LuaRocks — the diver-dap rock",
                "https://luarocks.org/modules/phaedrusflow/diver-dap",
                "Debug-adapter lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "diver-dev" => &[
            (
                "LuaRocks — the diver-dev rock",
                "https://luarocks.org/modules/phaedrusflow/diver-dev",
                "Development tooling lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "diver-games" => &[
            (
                "LuaRocks — the diver-games rock",
                "https://luarocks.org/modules/phaedrusflow/diver-games",
                "Games lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "diver-lintfmt" => &[
            (
                "LuaRocks — the diver-lintfmt rock",
                "https://luarocks.org/modules/phaedrusflow/diver-lintfmt",
                "Lint and format lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "diver-research" => &[
            (
                "LuaRocks — the diver-research rock",
                "https://luarocks.org/modules/phaedrusflow/diver-research",
                "Research and journal lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "diver-security" => &[
            (
                "LuaRocks — the diver-security rock",
                "https://luarocks.org/modules/phaedrusflow/diver-security",
                "Security tooling lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "diver-utils" => &[
            (
                "LuaRocks — the diver-utils rock",
                "https://luarocks.org/modules/phaedrusflow/diver-utils",
                "Utility lane of the Diver Neovim distribution, published as a LuaRock.",
            ),
            (
                "Diver — the Neovim distribution this rock belongs to",
                "https://github.com/qompassai/diver",
                "The source repository the rock is published from.",
            ),
        ],
        "eigenstyle" => &[
            (
                "Model Context Protocol — specification",
                "https://modelcontextprotocol.io/specification/2025-06-18",
                "The MCP tool surface Eigenstyle serves: tools/list and tools/call over JSON-RPC.",
            ),
            (
                "JSON-RPC 2.0 — specification",
                "https://www.jsonrpc.org/specification",
                "The wire format under every MCP exchange.",
            ),
        ],
        "fathom" => &[
            (
                "Model Context Protocol — specification",
                "https://modelcontextprotocol.io/specification/2025-06-18",
                "Fathom serves its providers over MCP (stdio and HTTP) — the tool contract.",
            ),
            (
                "A2A Protocol — Agent2Agent specification",
                "https://a2a-protocol.org/latest/",
                "Fathom also answers over the A2A agent protocol.",
            ),
            (
                "Ghidra — NSA software reverse engineering framework",
                "https://github.com/NationalSecurityAgency/ghidra",
                "The upstream analysis engine behind Fathom's native binary provider.",
            ),
        ],
        "hyprlua" => &[
            (
                "LuaRocks — the hyprlua rock",
                "https://luarocks.org/modules/phaedrusflow/hyprlua",
                "Lua bindings and helpers for the Hyprland compositor, published as a LuaRock.",
            ),
            (
                "Hyprland wiki",
                "https://wiki.hypr.land/",
                "The compositor hyprlua scripts against: its IPC and configuration model.",
            ),
        ],
        "lumen" => &[
            (
                "Model Context Protocol — specification",
                "https://modelcontextprotocol.io/specification/2025-06-18",
                "Lumen's style and VRM operations are served as MCP tools (stdio).",
            ),
            (
                "A2A Protocol — Agent2Agent specification",
                "https://a2a-protocol.org/latest/",
                "Lumen also serves the same operations over the A2A agent protocol.",
            ),
            (
                "VRM specification — VRM consortium",
                "https://github.com/vrm-c/vrm-specification",
                "The avatar format Lumen inspects and grades: a glTF 2.0 extension for humanoid avatars.",
            ),
            (
                "glTF 2.0 — Khronos specification",
                "https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html",
                "The base 3D format under every VRM file: the GLB container Lumen parses.",
            ),
        ],
        "phlow-mcp-server" => &[
            (
                "Model Context Protocol — specification",
                "https://modelcontextprotocol.io/specification/2025-06-18",
                "The server surface this package exposes for the Phlow runtime.",
            ),
            (
                "JSON-RPC 2.0 — specification",
                "https://www.jsonrpc.org/specification",
                "The wire format under every MCP exchange.",
            ),
            (
                "A2A Protocol — Agent2Agent specification",
                "https://a2a-protocol.org/latest/",
                "The agent protocol Phlow's runtime also speaks.",
            ),
        ],
        _ => &[],
    }
}

/// Measured benchmark tables for a solution page.
///
/// In the same `Benchmark` model the project study pages render.
/// Every figure was measured on primo for this site (2026-10-18) —
/// never taken from a project's own marketing — and each table's
/// note states the method, the machine, and the raw record the
/// values can be re-checked against. A slug with no entry renders
/// no BENCHMARKS section.
pub fn solution_benchmarks(slug: &str) -> &'static [Benchmark] {
    match slug {
        "moonwalk" => &[
            Benchmark {
                title: "Moonwalk — DAP session performance (measured 2026-10-18)",
                columns: &["Measurement", "Median", "Runs"],
                rows: &[
                    &["Launch → first breakpoint stopped", "104.7 ms", "10"],
                    &["Stepping (next × 200 per run)", "24.7 steps/s", "5"],
                    &["loop.lua plain (bundled Lua 5.4, spawn → exit)", "2.24 ms", "10"],
                    &["loop.lua under Moonwalk, no breakpoints (launch → terminated)", "130.1 ms", "10"],
                ],
                note: "Measured on primo (13th Gen Intel Core i7-13620H) with a release build of Moonwalk (qompassai/moonwalk @ d6a0b0a), driven over stdio by Moonwalk's own black-box DAP test client (test/dap/client.py) and fixtures (breakme.lua, loop.lua); one warm-up run discarded per measurement. Launch → stopped is timed from sending the launch request — the adapter is already spawned and initialized — to the stopped event at a line breakpoint. Stepping issues 200 next requests back-to-back, each awaiting its stopped event: DAP round-trip throughput, not raw Lua speed. The no-breakpoint figure is a whole debug session (launch → terminated) around a fixture that runs in 2.24 ms plain, so the 58× ratio of medians is the fixed cost of starting a session, not per-line overhead. Raw record: bench_moonwalk.py and moonwalk-results.json in the site-bench-20261018 work record.",
            },
        ],
        "lumen" => &[
            Benchmark {
                title: "Lumen — style grade throughput (measured 2026-10-18)",
                columns: &["Input", "Preset", "Median per grade", "Throughput"],
                rows: &[
                    &["448×448 portrait crop", "demo_toon_portrait", "7.77 ms", "25.8 Mpix/s"],
                    &["448×448 portrait crop", "ukiyo_e", "8.23 ms", "24.4 Mpix/s"],
                    &["1024×1024 (Lanczos3 upscale)", "demo_toon_portrait", "41.0 ms", "25.6 Mpix/s"],
                    &["1024×1024 (Lanczos3 upscale)", "ukiyo_e", "42.6 ms", "24.6 Mpix/s"],
                ],
                note: "Measured on primo (13th Gen Intel Core i7-13620H) with Lumen's release build, timing lumen::style::grade_image — the pixel path behind the style_apply MCP tool — at strength 1.0: 20 timed runs per row after 3 warm-ups, the median reported; PNG decode/encode and the per-run input clone sit outside the timed region. The subject is the 448×448 Séraphine portrait crop from the 2026-10-18 Lumen study, and that crop upscaled to 1024×1024. Throughput counts input megapixels per second. Raw record: run_lumen_bench.py and lumen-results.json in the site-bench-20261018 work record.",
            },
        ],
        "hyprlua" => &[
            Benchmark {
                title: "Hyprlua — typed coverage (counted 2026-10-18; coverage, not speed)",
                columns: &["Surface", "Counted", "Of"],
                rows: &[
                    &["Annotation modules present in library/", "2", "17 planned in library/map.md"],
                    &["Config keys named (HL.ConfigKey)", "341", "—"],
                    &["Config keys with a declared type (HL.ConfigValueTypes)", "337", "341 (98.8%)"],
                    &["Active ---@field annotations", "344", "—"],
                    &["---@class definitions", "4", "—"],
                    &["---@alias definitions", "3", "—"],
                ],
                note: "Counted from the addon's library/ tree (qompassai/hyprlua @ e103ee8) by line pattern: active annotations start with exactly ---@, and four commented-out ----@field lines are excluded. An annotations package has no runtime, so its honest value metric is typed coverage, not speed. The config surface is nearly fully typed — the four keys without a declared type are general.float_gaps, general.gaps_in, general.gaps_out and layout.single_window_aspect_ratio — while the module surface is early: only hl.config and hl.input of the 17 planned modules are present, and the top-level hl.lua is not yet among them. The repo does not state a total for the full Hyprland ≥ 0.55 Lua API surface, so no percentage of that surface is claimed. Raw record: count_hyprlua.py and hyprlua-results.json in the site-bench-20261018 work record.",
            },
        ],
        _ => &[],
    }
}

/// Real media artifacts shown on a generated solution page: (src, alt,
/// caption). Only artifacts the project itself produced — outputs of the
/// shipped engine, never illustrations. Empty for most slugs.
pub fn solution_media(slug: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    match slug {
        "lumen" => vec![
            ("/img/lumen/lumen_grade_compare.webp", "Before and after panels of Lumen style grades applied to a single portrait crop", "Lumen's shipped grade presets applied to one portrait crop by the engine itself — the before/after board from the 2026-10-18 lightshow video study, produced with the engine's own style operations."),
            ("/img/lumen/demo_toon_pixel_check.png", "The demo_toon grade checked on a pixel-art sprite", "The demo_toon preset — landed as a shipped style on 2026-10-18 — checked on a pixel sprite from the same study: the posterize step that speckles painted portraits stays clean on pixel art."),
        ],
        _ => Vec::new(),
    }
}

/// A cardinal stays in the readout up to this far off the bearing
/// (raw weight cos 80° ≈ 0.17); past it the share is too small to read.
const READOUT_MAX_OFF: u16 = 80;

/// The idle sweep: one degree per step, six steps a second.
#[cfg(feature = "hydrate")]
const SWEEP_STEP: std::time::Duration = std::time::Duration::from_millis(167);

/// A press has to travel this far (CSS px) before it turns the card,
/// so a tap on a tick stays a tap.
#[cfg(feature = "hydrate")]
const DRAG_SLOP: f64 = 4.0;

const POINTS: [&str; 16] =
    ["N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW"];

/// The eight winds the card carries as ticks, with their bearings.
const WINDS: [(u16, &str); 8] =
    [(0, "N"), (45, "NE"), (90, "E"), (135, "SE"), (180, "S"), (225, "SW"), (270, "W"), (315, "NW")];

/// Any whole-degree angle onto `0..360`.
pub fn normalize(deg: i32) -> u16 {
    u16::try_from(deg.rem_euclid(360)).unwrap_or(0)
}

/// Unsigned angular distance between two bearings, `0..=180`.
pub fn off(a: u16, b: u16) -> u16 {
    let d = (a % 360).abs_diff(b % 360);
    d.min(360 - d)
}

/// Signed whole degrees from `from` to `to` the short way round, in
/// `-179..=180` (a half-turn goes clockwise).
pub fn shortest_turn(from: u16, to: u16) -> i32 {
    let ahead = (i32::from(to % 360) - i32::from(from % 360)).rem_euclid(360);
    if ahead > 180 {
        ahead - 360
    } else {
        ahead
    }
}

/// Raw proximity weight `max(0, cos δ)`, exactly zero at and past 90°.
fn weight(off: u16) -> f64 {
    if off >= 90 {
        0.0
    } else {
        f64::from(off).to_radians().cos()
    }
}

/// One cardinal in the readout and its display share of the blend.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Share {
    pub cardinal: usize,
    pub share: f64,
}

/// The cardinals in the readout at `bearing`, primary first.
///
/// Share is weight over the summed weight of every cardinal with a
/// positive weight; a cardinal is listed only within `READOUT_MAX_OFF`.
/// Ties go clockwise from north. At most two are ever listed, since no
/// bearing lies within 80° of three cardinals.
pub fn blend(bearing: u16) -> Vec<Share> {
    let total: f64 = CARDINALS.iter().map(|c| weight(off(bearing, c.center))).sum();
    let mut shares: Vec<Share> = CARDINALS
        .iter()
        .enumerate()
        .filter(|(_, c)| off(bearing, c.center) <= READOUT_MAX_OFF)
        .map(|(i, c)| Share { cardinal: i, share: weight(off(bearing, c.center)) / total })
        .collect();
    shares.sort_by(|a, b| b.share.total_cmp(&a.share).then(a.cardinal.cmp(&b.cardinal)));
    shares
}

/// The 16-wind point nearest `bearing` (22.5° sectors, a boundary
/// rounds clockwise).
pub fn point_name(bearing: u16) -> &'static str {
    let sector = (u32::from(bearing % 360) * 100 + 1125) / 2250 % 16;
    usize::try_from(sector).ok().and_then(|i| POINTS.get(i)).copied().unwrap_or("N")
}

/// The bearing line: `045° — NE`.
pub fn bearing_text(bearing: u16) -> String {
    let b = bearing % 360;
    format!("{b:03}° — {}", point_name(b))
}

/// The slider's spoken value: `045 degrees — NE — Security and Education`.
pub fn value_text(bearing: u16) -> String {
    let b = bearing % 360;
    let names: Vec<&str> = blend(b).iter().filter_map(|s| CARDINALS.get(s.cardinal)).map(|c| c.name).collect();
    format!("{b:03} degrees — {} — {}", point_name(b), names.join(" and "))
}

/// Screen angle of a point `(dx, dy)` from the dial center, degrees
/// clockwise from straight up (screen y grows downward).
#[cfg(any(test, feature = "hydrate"))]
fn pointer_angle(dx: f64, dy: f64) -> f64 {
    dx.atan2(-dy).to_degrees()
}

/// The bearing after the card is dragged from screen angle `from` to
/// `to`, having started at `start`. The card turns with the pointer,
/// so the bearing under the lubber moves the other way.
#[cfg(any(test, feature = "hydrate"))]
#[allow(
    clippy::cast_possible_truncation,
    reason = "both angles come from atan2, so |start - (to - from)| < 1080; NaN saturates to 0"
)]
fn drag_bearing(start: u16, from: f64, to: f64) -> u16 {
    normalize((f64::from(start) - (to - from)).round() as i32)
}

fn solutions_of(cardinal: usize) -> impl Iterator<Item = &'static Solution> {
    SOLUTIONS.iter().filter(move |s| s.cardinal == cardinal)
}

/// Every solution of one class, alphabetical by display name — the
/// order of the class index pages.
pub fn solutions_of_class(class: Class) -> Vec<&'static Solution> {
    let mut out: Vec<&'static Solution> = SOLUTIONS.iter().filter(|s| s.class == class).collect();
    out.sort_by(|a, b| a.name.cmp(b.name));
    out
}

/// The version of one solution, linked out to its release; nothing for
/// a solution without one.
pub fn version_link(s: &'static Solution) -> impl IntoView {
    s.version.zip(s.version_href()).map(|(v, href)| {
        view! {
            <a class="cs-version" href=href aria-label=format!("{} version {v}", s.name)>
                {v}
            </a>
        }
    })
}

/// One solution: its page on this site, then its version.
fn solution_item(s: &'static Solution) -> impl IntoView {
    view! {
        <li>
            <a class="cs-solution" href=format!("/solutions/{}", s.slug)>
                {s.name}
            </a>
            {version_link(s)}
        </li>
    }
}

/// The links of one cardinal.
fn solution_links(cardinal: usize) -> impl IntoView {
    view! { <ul class="service-links">{solutions_of(cardinal).map(solution_item).collect_view()}</ul> }
}

/// The four cardinal cards, every solution listed under its cardinal —
/// shown on /solutions under the fleet.
#[component]
pub fn CardinalCards() -> impl IntoView {
    view! {
        <div class="cs-directory">
            {CARDINALS
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    view! {
                        <article class="cs-group">
                            <p class="pillar-mark">{c.mark}</p>
                            <h2>{c.name}</h2>
                            <p>{c.brief}</p>
                            {solution_links(i)}
                        </article>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// The section: compass and readout, and the binding line.
///
/// Every piece of state is a signal named here:
/// - `bearing` — the readout's bearing, always in `0..360`, because
///   every change goes through `steer`.
/// - `turn` — the card's unwrapped angle in degrees. `steer` moves it the
///   short way so the card never spins the long way across north;
///   nothing reads it back.
/// - `held` — the visitor has steered (drag, tick, or key): the sweep
///   stops for good. `hovering` pauses it only while the pointer or
///   focus is on the instrument.
/// - `hydrated`, `sweeping`, `dragging` — false in the prerender.
#[component]
pub fn CompassServices() -> impl IntoView {
    let bearing = RwSignal::new(0_u16);
    let turn = RwSignal::new(0_i32);
    let held = RwSignal::new(false);
    let hovering = RwSignal::new(false);
    let hydrated = RwSignal::new(false);
    let sweeping = RwSignal::new(false);
    let dragging = RwSignal::new(false);
    let shares = Memo::new(move |_| blend(bearing.get()));

    let steer = move |to: u16| {
        let to = to % 360;
        // Saturating: at six degrees a second the counter needs over
        // eleven years of sweep to get near i32::MAX.
        turn.update(|t| *t = t.saturating_add(shortest_turn(bearing.get_untracked(), to)));
        bearing.set(to);
    };
    let take_helm = move || {
        held.set(true);
        sweeping.set(false);
    };

    Effect::new(move |_| {
        hydrated.set(true);
        #[cfg(feature = "hydrate")]
        start_sweep(sweeping, held, hovering, bearing, steer);
    });

    // Drag: (pointer id, screen angle at press, bearing at press, press x, y).
    let press = StoredValue::new(None::<(i32, f64, u16, f64, f64)>);
    let instrument = NodeRef::<leptos::html::Div>::new();

    let on_down = move |ev: leptos::ev::PointerEvent| {
        #[cfg(feature = "hydrate")]
        {
            let Some(el) = instrument.get_untracked() else {
                return;
            };
            let rect = el.get_bounding_client_rect();
            let (x, y) = (f64::from(ev.client_x()), f64::from(ev.client_y()));
            let angle = pointer_angle(x - rect.left() - rect.width() / 2.0, y - rect.top() - rect.height() / 2.0);
            press.set_value(Some((ev.pointer_id(), angle, bearing.get_untracked(), x, y)));
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = (ev, instrument, press);
    };
    let on_move = move |ev: leptos::ev::PointerEvent| {
        #[cfg(feature = "hydrate")]
        {
            let (Some((id, from, start, x0, y0)), Some(el)) = (press.get_value(), instrument.get_untracked()) else {
                return;
            };
            if id != ev.pointer_id() {
                return;
            }
            let (x, y) = (f64::from(ev.client_x()), f64::from(ev.client_y()));
            if !dragging.get_untracked() {
                if (x - x0).hypot(y - y0) < DRAG_SLOP {
                    return;
                }
                let _ = el.set_pointer_capture(id);
                dragging.set(true);
                take_helm();
            }
            let rect = el.get_bounding_client_rect();
            let to = pointer_angle(x - rect.left() - rect.width() / 2.0, y - rect.top() - rect.height() / 2.0);
            steer(drag_bearing(start, from, to));
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = ev;
    };
    let on_up = move |_| {
        press.set_value(None);
        dragging.set(false);
    };

    let on_key = move |ev: leptos::ev::KeyboardEvent| {
        let step = if ev.shift_key() { 10 } else { 1 };
        let delta = match ev.key().as_str() {
            "ArrowRight" | "ArrowUp" => step,
            "ArrowLeft" | "ArrowDown" => -step,
            _ => return,
        };
        ev.prevent_default();
        take_helm();
        steer(normalize(i32::from(bearing.get_untracked()) + delta));
    };

    let card_style = move || format!("--cs-turn: {}deg", -turn.get());

    let rose = (0..72_u16)
        .map(|i| {
            let at = i * 5;
            let (y2, class) = if at % 45 == 0 {
                (44, "cs-major")
            } else if at % 15 == 0 {
                (32, "cs-minor")
            } else {
                (24, "cs-fine")
            };
            view! { <line class=class x1="300" y1="12" x2="300" y2=y2 transform=format!("rotate({at} 300 300)") /> }
        })
        .collect_view();

    let names = CARDINALS
        .iter()
        .map(|c| {
            view! {
                <text class="cs-service-name" x="300" y="112" transform=format!("rotate({} 300 300)", c.center)>
                    {c.name.to_uppercase()}
                </text>
            }
        })
        .collect_view();

    let ticks = WINDS
        .iter()
        .map(|&(at, wind)| {
            let label = format!("Set bearing to {at:03} degrees — {wind}");
            let cardinal = at % 90 == 0;
            view! {
                <g
                    class=move || if bearing.get() == at { "cs-tick is-heading" } else { "cs-tick" }
                    transform=format!("rotate({at} 300 300)")
                    role=move || hydrated.get().then_some("button")
                    tabindex=move || hydrated.get().then_some("0")
                    aria-label=label
                    on:click=move |_| {
                        take_helm();
                        steer(at);
                    }
                    on:keydown=move |ev| {
                        let key = ev.key();
                        if key == "Enter" || key == " " {
                            ev.prevent_default();
                            take_helm();
                            steer(at);
                        }
                    }
                >
                    <rect class="cs-hit" x="272" y="4" width="56" height="84" rx="6" />
                    <text class=if cardinal { "cs-wind is-cardinal" } else { "cs-wind" } x="300" y="72">
                        {wind}
                    </text>
                </g>
            }
        })
        .collect_view();

    // One blended list: the primary cardinal's solutions first.
    let readout_links =
        move || shares.get().into_iter().flat_map(|s| solutions_of(s.cardinal)).map(solution_item).collect_view();

    view! {
        <section class="services-band" aria-labelledby="services-head">
            <p class="band-label" id="services-head">"FOUR CARDINAL SERVICES"</p>
            <div class="cs-layout">
                <div class="cs-housing">
                    <div
                        class=move || {
                            if dragging.get() {
                                "cs-instrument is-dragging"
                            } else if sweeping.get() && !hovering.get() {
                                "cs-instrument is-sweeping"
                            } else {
                                "cs-instrument"
                            }
                        }
                        node_ref=instrument
                        on:pointerdown=on_down
                        on:pointermove=on_move
                        on:pointerup=on_up
                        on:pointercancel=on_up
                        on:mouseenter=move |_| hovering.set(true)
                        on:mouseleave=move |_| hovering.set(false)
                        on:focusin=move |_| hovering.set(true)
                        on:focusout=move |_| hovering.set(false)
                    >
                        <svg class="cs-dial" viewBox="0 0 600 600" aria-hidden="true">
                            <g class="cs-card" style=card_style>
                                <circle class="cs-rim" cx="300" cy="300" r="292" />
                                {rose}
                                {names}
                                <circle class="cs-inner" cx="300" cy="300" r="150" />
                            </g>
                        </svg>
                        <svg class="cs-dial cs-ticks" viewBox="0 0 600 600" role="group" aria-label="Bearing presets">
                            <g class="cs-card" style=card_style>
                                {ticks}
                            </g>
                            <polygon class="cs-lubber" points="288,0 312,0 300,18" />
                            <line class="cs-lubber" x1="300" y1="18" x2="300" y2="46" />
                        </svg>
                        <div
                            class="cs-hub"
                            role=move || hydrated.get().then_some("slider")
                            tabindex=move || hydrated.get().then_some("0")
                            aria-label=move || hydrated.get().then_some("Service bearing")
                            aria-valuemin=move || hydrated.get().then_some("0")
                            aria-valuemax=move || hydrated.get().then_some("359")
                            aria-valuenow=move || hydrated.get().then(|| bearing.get().to_string())
                            aria-valuetext=move || hydrated.get().then(|| value_text(bearing.get()))
                            on:keydown=on_key
                        >
                            <span class="cs-hub-label">"BEARING"</span>
                            <span class="cs-hub-value">{move || bearing_text(bearing.get())}</span>
                        </div>
                    </div>
                    <p class="cs-plate">"FIXED POINT · SPOKANE — 47.66° N, 117.43° W"</p>
                </div>
                <div class="compass-services-readout" aria-live=move || if sweeping.get() { "off" } else { "polite" }>
                    <p class="cs-r-bearing">{move || bearing_text(bearing.get())}</p>
                    <ul class="service-links">{readout_links}</ul>
                </div>
            </div>
            <p class="cs-binding">
                "Every solution on this compass is built under " <a href="/standards">"Standards"</a>
                ", weighed in " <a href="/analysis">"Analysis"</a> ", and proven in "
                <a href="/field-notes">"Field Notes"</a> "."
            </p>
        </section>
    }
    // Type-erased at the boundary, like the identity section: the full
    // static type of this tree is deep enough to strain the query depth
    // once nested in the home page.
    .into_any()
}

/// Start the idle sweep: one degree clockwise per `SWEEP_STEP`.
///
/// A step is skipped while the pointer or focus rests on the
/// instrument, and the sweep ends for good once the visitor steers.
/// Without `matchMedia`, or for a visitor who asked for reduced
/// motion, it never starts and the card stays where the prerender drew it.
#[cfg(feature = "hydrate")]
fn start_sweep(
    sweeping: RwSignal<bool>,
    held: RwSignal<bool>,
    hovering: RwSignal<bool>,
    bearing: RwSignal<u16>,
    steer: impl Fn(u16) + 'static,
) {
    let reduced = web_sys::window()
        .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .is_none_or(|m| m.matches());
    if reduced {
        return;
    }
    sweeping.set(true);
    let step = move || {
        if held.get_untracked() || hovering.get_untracked() {
            return;
        }
        steer(bearing.get_untracked().saturating_add(1) % 360);
    };
    if let Ok(handle) = leptos::leptos_dom::helpers::set_interval_with_handle(step, SWEEP_STEP) {
        on_cleanup(move || handle.clear());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names_at(bearing: u16) -> Vec<&'static str> {
        blend(bearing).iter().filter_map(|s| CARDINALS.get(s.cardinal)).map(|c| c.name).collect()
    }

    fn near(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn an_exact_cardinal_is_that_service_alone_at_full_share() {
        for (bearing, name) in [(0, "Security"), (90, "Education"), (180, "Deployment"), (270, "Infrastructure")] {
            let b = blend(bearing);
            assert_eq!(names_at(bearing), vec![name], "{bearing}");
            assert!(b.first().is_some_and(|s| near(s.share, 1.0)), "{bearing}");
        }
    }

    #[test]
    fn an_exact_intercardinal_splits_fifty_fifty_clockwise_from_north() {
        for (bearing, pair) in [
            (45, ["Security", "Education"]),
            (135, ["Education", "Deployment"]),
            (225, ["Deployment", "Infrastructure"]),
            (315, ["Security", "Infrastructure"]),
        ] {
            assert_eq!(names_at(bearing), pair.to_vec(), "{bearing}");
            assert!(blend(bearing).iter().all(|s| near(s.share, 0.5)), "{bearing}");
        }
    }

    #[test]
    fn the_neighbor_appears_ten_degrees_off() {
        assert_eq!(names_at(9), vec!["Security"]);
        let b = blend(10);
        assert_eq!(names_at(10), vec!["Security", "Education"]);
        assert_eq!(b.iter().map(|s| (s.share * 100.0).round()).collect::<Vec<_>>(), vec![85.0, 15.0]);
        assert_eq!(names_at(80), vec!["Education", "Security"]);
        assert_eq!(names_at(81), vec!["Education"]);
    }

    #[test]
    fn the_blend_wraps_through_north() {
        assert_eq!(names_at(359), vec!["Security"]);
        assert_eq!(names_at(350), vec!["Security", "Infrastructure"]);
        for d in 1..=44 {
            let east = blend(d);
            let west = blend(360 - d);
            assert_eq!(east.len(), west.len(), "{d}");
            for (e, w) in east.iter().zip(&west) {
                assert!(near(e.share, w.share), "{d}");
            }
        }
    }

    #[test]
    fn sixteen_wind_names_turn_over_at_sector_boundaries() {
        assert_eq!(point_name(0), "N");
        assert_eq!(point_name(11), "N");
        assert_eq!(point_name(12), "NNE");
        assert_eq!(point_name(33), "NNE");
        assert_eq!(point_name(34), "NE");
        assert_eq!(point_name(45), "NE");
        assert_eq!(point_name(90), "E");
        assert_eq!(point_name(191), "S");
        assert_eq!(point_name(192), "SSW");
        assert_eq!(point_name(348), "NNW");
        assert_eq!(point_name(349), "N");
        assert_eq!(point_name(359), "N");
    }

    #[test]
    fn readout_and_spoken_values_are_zero_padded() {
        assert_eq!(bearing_text(0), "000° — N");
        assert_eq!(bearing_text(45), "045° — NE");
        assert_eq!(value_text(45), "045 degrees — NE — Security and Education");
        assert_eq!(value_text(270), "270 degrees — W — Infrastructure");
    }

    #[test]
    fn every_bearing_lists_one_or_two_services_primary_first() {
        for b in 0..360 {
            let shares = blend(b);
            assert!((1..=2).contains(&shares.len()), "{b}");
            assert!(shares.iter().all(|s| s.share > 0.0 && s.share <= 1.0), "{b}");
            assert!(shares.windows(2).all(|w| w.first().map(|s| s.share) >= w.get(1).map(|s| s.share)), "{b}");
            assert!(shares.iter().map(|s| s.share).sum::<f64>() <= 1.0 + 1e-9, "{b}");
        }
    }

    #[test]
    fn out_of_range_bearings_fold_onto_the_card() {
        assert_eq!(normalize(-1), 359);
        assert_eq!(normalize(360), 0);
        assert_eq!(normalize(i32::MIN), 232);
        assert_eq!(normalize(i32::MAX), 127);
        assert_eq!(names_at(360), names_at(0));
        assert_eq!(names_at(u16::MAX), names_at(u16::MAX % 360));
        assert_eq!(point_name(u16::MAX), point_name(u16::MAX % 360));
        assert_eq!(bearing_text(405), "045° — NE");
    }

    #[test]
    fn the_card_turns_the_short_way_round() {
        assert_eq!(shortest_turn(359, 0), 1);
        assert_eq!(shortest_turn(0, 359), -1);
        assert_eq!(shortest_turn(0, 180), 180);
        assert_eq!(shortest_turn(10, 10), 0);
        assert_eq!(off(350, 10), 20);
        assert_eq!(off(0, 180), 180);
    }

    #[test]
    fn dragging_turns_the_bearing_against_the_pointer() {
        assert!(near(pointer_angle(0.0, -1.0), 0.0));
        assert!(near(pointer_angle(1.0, 0.0), 90.0));
        assert!(near(pointer_angle(0.0, 1.0), 180.0));
        assert!(near(pointer_angle(-1.0, 0.0), -90.0));
        // Card dragged 30° clockwise: the bearing under the lubber backs off 30°.
        assert_eq!(drag_bearing(0, 0.0, 30.0), 330);
        assert_eq!(drag_bearing(45, 90.0, 45.0), 90);
        // Across the bottom seam (-170° to 170° is 20° anticlockwise).
        assert_eq!(drag_bearing(10, -170.0, 170.0), 30);
        assert_eq!(drag_bearing(10, f64::NAN, 0.0), 0);
    }

    #[test]
    fn the_org_profile_set_stands_exactly_in_order() {
        let expected = [
            (N, "https://github.com/qompassai/qompass-hold"),
            (N, "https://github.com/qompassai/qssl"),
            (N, "https://github.com/qompassai/radar"),
            (N, "https://github.com/qompassai/redteam"),
            (N, "https://github.com/qompassai/tor"),
            (N, "https://github.com/qompassai/vongola"),
            (E, "https://github.com/qompassai/blaze-ts.nvim"),
            (E, "https://github.com/qompassai/blaze.nvim"),
            (E, "https://github.com/qompassai/Diver"),
            (E, "https://github.com/qompassai/PathFinders"),
            (E, "https://github.com/qompassai/qluv"),
            (E, "https://github.com/qompassai/rose.nvim"),
            (E, "https://github.com/qompassai/tuxtype"),
            (E, "https://github.com/qompassai/unreal"),
            (S, "https://github.com/qompassai/Azimuth"),
            (S, "https://github.com/qompassai/current"),
            (S, "https://github.com/qompassai/dotfiles"),
            (S, "https://github.com/qompassai/qonfig"),
            (S, "https://github.com/qompassai/qudaz"),
            (S, "https://github.com/qompassai/qxet"),
            (S, "https://github.com/qompassai/rose"),
            (W, "https://github.com/qompassai/beacon"),
            (W, "https://github.com/qompassai/bunker"),
            (W, "https://github.com/qompassai/debian"),
            (W, "https://github.com/qompassai/network"),
            (W, "https://github.com/qompassai/NUR"),
            (W, "https://github.com/qompassai/qjp"),
        ];
        for (cardinal, count) in [(N, 6), (E, 8), (S, 7), (W, 6)] {
            let got: Vec<&str> = solutions_of(cardinal).take(count).map(|s| s.href).collect();
            let want: Vec<&str> = expected.iter().filter(|(c, _)| *c == cardinal).map(|(_, h)| *h).collect();
            assert_eq!(got, want, "{cardinal}");
        }
    }

    #[test]
    fn the_completed_membership_counts_and_links_are_sound() {
        let counts: Vec<usize> = (0..4).map(|c| solutions_of(c).count()).collect();
        assert_eq!(counts, vec![8, 13, 18, 6]);
        for (i, s) in SOLUTIONS.iter().enumerate() {
            assert!(s.cardinal < CARDINALS.len(), "{}", s.name);
            assert!(
                s.href.starts_with("https://github.com/qompassai/")
                    || s.href.starts_with("https://luarocks.org/modules/phaedrusflow/")
                    || s.href == format!("/solutions/{}", s.slug),
                "{}",
                s.name
            );
            assert!(SOLUTIONS.iter().skip(i + 1).all(|o| o.href != s.href && o.name != s.name), "{}", s.name);
        }
    }

    #[test]
    fn every_slug_is_unique_lowercase_hyphenated_and_resolves() {
        for (i, s) in SOLUTIONS.iter().enumerate() {
            assert!(!s.slug.is_empty() && !s.slug.starts_with('-') && !s.slug.ends_with('-'), "{}", s.name);
            assert!(s.slug.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'), "{}", s.slug);
            assert!(SOLUTIONS.iter().skip(i + 1).all(|o| o.slug != s.slug), "{}", s.slug);
            assert_eq!(solution_page(s.slug).map(|p| p.name), Some(s.name), "{}", s.slug);
        }
        for (name, slug) in [
            ("Blaze.nvim", "blaze-nvim"),
            ("Blaze.nvim (LuaRocks)", "blaze-nvim-luarocks"),
            ("Phlow MCP Server", "phlow-mcp-server"),
            ("Blaze-TS.nvim", "blaze-ts-nvim"),
            ("Rose.nvim", "rose-nvim"),
        ] {
            assert_eq!(SOLUTIONS.iter().find(|s| s.name == name).map(|s| s.slug), Some(slug), "{name}");
        }
    }

    #[test]
    fn a_slug_off_the_table_resolves_to_nothing() {
        for slug in ["", "Diver", "blaze.nvim", "../diver", "diver/", "diver ", "phlow", "rose%2Dnvim", "nur\0"] {
            assert!(solution_page(slug).is_none(), "{slug:?}");
        }
    }

    #[test]
    fn github_solutions_carry_v1_00_except_the_archived_rose_nvim() {
        for s in SOLUTIONS.iter().filter(|s| s.href.starts_with(GITHUB)) {
            if s.slug == "rose-nvim" {
                assert_eq!(s.version, None);
                assert_eq!(s.version_href(), None);
            } else {
                assert_eq!(s.version, Some("1.00"), "{}", s.name);
            }
        }
        let release = |slug| solution_page(slug).and_then(Solution::version_href);
        assert_eq!(release("qssl").as_deref(), Some("https://github.com/qompassai/qssl/releases/tag/v1.00"));
        assert_eq!(release("diver").as_deref(), Some("https://github.com/qompassai/Diver/releases/tag/v1.00"));
        // The fragment on the href is not part of the repository.
        assert_eq!(
            release("phlow-mcp-server").as_deref(),
            Some("https://github.com/qompassai/phlow/releases/tag/v1.00")
        );
    }

    #[test]
    fn luarocks_solutions_carry_their_published_versions_never_1_00() {
        let rocks: Vec<(&str, Option<&str>, Option<String>)> = SOLUTIONS
            .iter()
            .filter(|s| s.href.starts_with("https://luarocks.org/"))
            .map(|s| (s.slug, s.version, s.version_href()))
            .collect();
        assert_eq!(rocks.len(), 12);
        for (slug, version, href) in &rocks {
            let want = if *slug == "blaze-nvim-luarocks" { "1.0-1" } else { "0.1.0-1" };
            assert_eq!(*version, Some(want), "{slug}");
            assert_eq!(href.as_deref(), solution_page(slug).map(|s| s.href), "{slug}");
        }
    }

    #[test]
    fn every_solution_carries_a_mark_that_exists() {
        assert_eq!(SOLUTIONS.iter().filter(|s| s.mark().is_some()).count(), SOLUTIONS.len());
        for mark in SOLUTIONS.iter().filter_map(Solution::mark) {
            let path = format!("{}/public{mark}", env!("CARGO_MANIFEST_DIR"));
            assert!(std::path::Path::new(&path).is_file(), "{path}");
        }
    }

    #[test]
    fn the_classes_divide_the_fleet() {
        let count = |class| SOLUTIONS.iter().filter(|s| s.class == class).count();
        assert_eq!(count(Class::App), 29);
        assert_eq!(count(Class::Mcp), 4);
        assert_eq!(count(Class::Package), 12);
        // Every package is a LuaRocks module; every MCP server lives on GitHub.
        assert!(SOLUTIONS
            .iter()
            .filter(|s| s.class == Class::Package)
            .all(|s| s.href.starts_with("https://luarocks.org/")));
        assert!(SOLUTIONS
            .iter()
            .filter(|s| s.class == Class::Mcp)
            .all(|s| s.href.starts_with("https://github.com/qompassai/")));
    }

    #[test]
    fn the_west_service_is_infrastructure() {
        assert_eq!(
            CARDINALS.iter().map(|c| c.name).collect::<Vec<_>>(),
            ["Security", "Education", "Deployment", "Infrastructure"]
        );
        assert!(CARDINALS.iter().all(|c| c.name != "Servers"));
    }
}
