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

/// One published solution under one cardinal. `slug` is its page,
/// `/solutions/<slug>`; `href` is where it lives (GitHub or `LuaRocks`).
pub struct Solution {
    pub name: &'static str,
    pub slug: &'static str,
    pub href: &'static str,
    pub cardinal: usize,
    pub version: Option<&'static str>,
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

    /// The project mark, for the solutions that have one of the twelve.
    pub const fn mark(&self) -> Option<&'static str> {
        match self.slug.as_bytes() {
            b"bunker" => Some("/img/projects/bunker.webp"),
            b"diver" => Some("/img/projects/diver.webp"),
            b"fathom" => Some("/img/projects/fathom.webp"),
            b"phlow-mcp-server" => Some("/img/projects/phlow.webp"),
            b"vongola" => Some("/img/projects/vongola.webp"),
            _ => None,
        }
    }
}

const fn sol(
    name: &'static str,
    slug: &'static str,
    href: &'static str,
    cardinal: usize,
    version: Option<&'static str>,
) -> Solution {
    Solution { name, slug, href, cardinal, version }
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
/// lives inside Diver.
pub const SOLUTIONS: [Solution; 43] = [
    sol("Qompass-Hold", "qompass-hold", "https://github.com/qompassai/qompass-hold", N, V1),
    sol("Qssl", "qssl", "https://github.com/qompassai/qssl", N, V1),
    sol("Radar", "radar", "https://github.com/qompassai/radar", N, V1),
    sol("Redteam", "redteam", "https://github.com/qompassai/redteam", N, V1),
    sol("Tor", "tor", "https://github.com/qompassai/tor", N, V1),
    sol("Vongola", "vongola", "https://github.com/qompassai/vongola", N, V1),
    sol("fathom", "fathom", "https://github.com/qompassai/fathom", N, V1),
    sol("diver-security", "diver-security", "https://luarocks.org/modules/phaedrusflow/diver-security", N, ROCK),
    sol("blaze-ts.nvim", "blaze-ts-nvim", "https://github.com/qompassai/blaze-ts.nvim", E, V1),
    sol("blaze.nvim", "blaze-nvim", "https://github.com/qompassai/blaze.nvim", E, V1),
    sol("Diver", "diver", "https://github.com/qompassai/Diver", E, V1),
    sol("PathFinders", "pathfinders", "https://github.com/qompassai/PathFinders", E, V1),
    sol("Qluv", "qluv", "https://github.com/qompassai/qluv", E, V1),
    sol("Rose.nvim", "rose-nvim", "https://github.com/qompassai/rose.nvim", E, None),
    sol("Tuxtype", "tuxtype", "https://github.com/qompassai/tuxtype", E, V1),
    sol("Unreal", "unreal", "https://github.com/qompassai/unreal", E, V1),
    sol("eigenstyle", "eigenstyle", "https://github.com/qompassai/eigenstyle", E, V1),
    sol(
        "blaze.nvim (LuaRocks)",
        "blaze-nvim-luarocks",
        "https://luarocks.org/modules/phaedrusflow/blaze.nvim",
        E,
        Some("1.0-1"),
    ),
    sol("diver-research", "diver-research", "https://luarocks.org/modules/phaedrusflow/diver-research", E, ROCK),
    sol("Azimuth", "azimuth", "https://github.com/qompassai/Azimuth", S, V1),
    sol("Current", "current", "https://github.com/qompassai/current", S, V1),
    sol("Dotfiles", "dotfiles", "https://github.com/qompassai/dotfiles", S, V1),
    sol("Qonfig", "qonfig", "https://github.com/qompassai/qonfig", S, V1),
    sol("Qudaz", "qudaz", "https://github.com/qompassai/qudaz", S, V1),
    sol("Qxet", "qxet", "https://github.com/qompassai/qxet", S, V1),
    sol("Rose", "rose", "https://github.com/qompassai/rose", S, V1),
    sol("lumen", "lumen", "https://github.com/qompassai/lumen", S, V1),
    sol("phlow MCP server", "phlow-mcp-server", "https://github.com/qompassai/phlow#mcp-wire-contract", S, V1),
    sol("diver-ai", "diver-ai", "https://luarocks.org/modules/phaedrusflow/diver-ai", S, ROCK),
    sol("diver-comms", "diver-comms", "https://luarocks.org/modules/phaedrusflow/diver-comms", S, ROCK),
    sol("diver-config", "diver-config", "https://luarocks.org/modules/phaedrusflow/diver-config", S, ROCK),
    sol("diver-dap", "diver-dap", "https://luarocks.org/modules/phaedrusflow/diver-dap", S, ROCK),
    sol("diver-dev", "diver-dev", "https://luarocks.org/modules/phaedrusflow/diver-dev", S, ROCK),
    sol("diver-games", "diver-games", "https://luarocks.org/modules/phaedrusflow/diver-games", S, ROCK),
    sol("diver-lintfmt", "diver-lintfmt", "https://luarocks.org/modules/phaedrusflow/diver-lintfmt", S, ROCK),
    sol("diver-utils", "diver-utils", "https://luarocks.org/modules/phaedrusflow/diver-utils", S, ROCK),
    sol("hyprlua", "hyprlua", "https://luarocks.org/modules/phaedrusflow/hyprlua", S, ROCK),
    sol("beacon", "beacon", "https://github.com/qompassai/beacon", W, V1),
    sol("bunker", "bunker", "https://github.com/qompassai/bunker", W, V1),
    sol("debian", "debian", "https://github.com/qompassai/debian", W, V1),
    sol("network", "network", "https://github.com/qompassai/network", W, V1),
    sol("NUR", "nur", "https://github.com/qompassai/NUR", W, V1),
    sol("qjp", "qjp", "https://github.com/qompassai/qjp", W, V1),
];

/// The solution whose page is `/solutions/<slug>`, if any.
pub fn solution_page(slug: &str) -> Option<&'static Solution> {
    SOLUTIONS.iter().find(|s| s.slug == slug)
}

/// One MCP server in the estate inventory: what it is, where it
/// stands, and the evidence line. Statuses use the vocabulary the
/// tests pin: Commissioned, Design stage, In development, Legacy,
/// Live, Published.
pub struct McpServer {
    pub name: &'static str,
    pub role: &'static str,
    pub status: &'static str,
    pub detail: &'static str,
    pub href: Option<&'static str>,
    pub registry: Option<&'static str>,
}

const fn mcp(
    name: &'static str,
    role: &'static str,
    status: &'static str,
    detail: &'static str,
    href: Option<&'static str>,
    registry: Option<&'static str>,
) -> McpServer {
    McpServer { name, role, status, detail, href, registry }
}

/// The display form of a roster name, for titles and links: each
/// word capitalized ("ghidra bridge" -> "Ghidra Bridge"). The
/// roster `name` itself stays the lowercase key — slugs and client
/// config keys derive from it; this is presentation only.
pub fn mcp_display_name(name: &str) -> String {
    name.split(' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Every MCP server in the estate roster, alphabetical — the
/// public mirror of the internal census, verified 2026-10-20, plus
/// the day's commissions (atelier). The Ghidra bridge is upstream,
/// not a Qompass product, and is listed as External because the
/// estate declares and uses it; rea, the upstream server fathom
/// superseded, is not listed. Vongola was restored 2026-10-20: the
/// census omitted it then (corrected the same day), so its entry
/// and page are read from the server's own source
/// (crates/vongola-mcp). Every server has a page:
/// `/solutions/mcp/<name>`.
pub const MCP_SERVERS: [McpServer; 16] = [
    mcp(
        "atelier",
        "Media generation — image, video, and music behind ComfyUI",
        "Commissioned",
        "Commissioned 2026-10-20 · design and S1 brief complete; the image lane (Iris) is in preparation, not yet built.",
        Some("https://github.com/qompassai/atelier"),
        None,
    ),
    mcp(
        "broadside",
        "This website — build, verify, deploy, with a honeypot",
        "Commissioned",
        "Commissioned 2026-10-20 as press; renamed broadside the same day · design complete, build pending.",
        None,
        None,
    ),
    mcp(
        "buccaneer",
        "Security-assessment engine — rose-first, chains into fathom",
        "In development",
        "Tool surface source-verified; the v1 gate is pending.",
        Some("https://github.com/qompassai/buccaneer"),
        None,
    ),
    mcp(
        "bunker",
        "Signed-manifest executor for consequential actions",
        "In development",
        "The Stage C server is under construction.",
        Some("https://github.com/qompassai/bunker"),
        None,
    ),
    mcp(
        "eigenstyle",
        "Voice and style fingerprinting for drafts",
        "Published",
        "MCP Registry ai.qompass/eigenstyle 0.1.1, as a bundle.",
        Some("https://github.com/qompassai/eigenstyle"),
        Some("https://registry.modelcontextprotocol.io/?q=eigenstyle"),
    ),
    mcp(
        "fathom",
        "Reverse engineering and artifact analysis",
        "Published",
        "MCP Registry ai.qompass/fathom 0.1.1 · latest release v0.1.2 · 28 tools · DOI 10.5281/zenodo.23503521.",
        Some("https://github.com/qompassai/fathom"),
        Some("https://registry.modelcontextprotocol.io/?q=fathom"),
    ),
    mcp(
        "ghidra bridge",
        "GhidraMCP plugin surface over stdio, fronting a running Ghidra GUI",
        "External",
        "Upstream bridge declared in Diver; not a Qompass product, and not launchable headless.",
        None,
        None,
    ),
    mcp(
        "lumen",
        "Deterministic conversion between 2D, 3D, and engine packages",
        "Published",
        "MCP Registry ai.qompass/lumen 0.1.1 · release v0.1.1 · 7 tools · DOI 10.5281/zenodo.23503519.",
        Some("https://github.com/qompassai/lumen"),
        Some("https://registry.modelcontextprotocol.io/?q=lumen"),
    ),
    mcp(
        "lupin",
        "Revenue alignment across the Qompass and NTRanch lanes",
        "Published",
        "MCP Registry ai.qompass/lupin 0.1.0 · 6 tools.",
        Some("https://github.com/qompassai/lupin"),
        Some("https://registry.modelcontextprotocol.io/?q=lupin"),
    ),
    mcp(
        "periscope",
        "Neovim broker — editor state, LSP/DAP, shared attention",
        "Published",
        "MCP Registry ai.qompass/periscope 0.1.0 · 16 tools.",
        Some("https://github.com/qompassai/periscope"),
        Some("https://registry.modelcontextprotocol.io/?q=periscope"),
    ),
    mcp(
        "phaedrus",
        "Research feed — findings, watchlists, revenue candidates",
        "Published",
        "MCP Registry ai.qompass/phaedrus 0.1.0 · 7 tools.",
        Some("https://github.com/qompassai/phaedrus"),
        Some("https://registry.modelcontextprotocol.io/?q=phaedrus"),
    ),
    mcp(
        "phlow",
        "Agent runtime — CLI, TUI, and MCP over one safe core",
        "Published",
        "MCP Registry ai.qompass/phlow 0.2.1 · server name “flow” · 7 tools.",
        Some("https://github.com/qompassai/phlow"),
        Some("https://registry.modelcontextprotocol.io/?q=phlow"),
    ),
    mcp(
        "tackle",
        "Editor broker — Unity first, Godot later",
        "Commissioned",
        "Commissioned 2026-10-20 · the first channel spike is in build.",
        None,
        None,
    ),
    mcp(
        "twilight",
        "Front desk for local inference engines",
        "Live",
        "v0.1.0 · local release; public release pending.",
        None,
        None,
    ),
    mcp(
        "volta",
        "Key service for the signed-authority chain",
        "Design stage",
        "Design complete; no repository yet.",
        None,
        None,
    ),
    mcp(
        "vongola",
        "Read-only operator instrumentation for the vongola TLS gateway",
        "Live",
        "v0.1.0 · 5 read-only tools · local build; landing pending.",
        Some("https://github.com/qompassai/vongola"),
        None,
    ),
];

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

/// The MCP estate as a status table, on /solutions under the
/// cardinal directory: every server, what it is, and where it
/// stands — including the ones with no public home yet. In the
/// design system's bench-table dress, like the study pages.
#[component]
pub fn McpServers() -> impl IntoView {
    view! {
        <figure class="bench mcp-bench">
            <figcaption class="bench-title">"MCP servers — the estate, with status"</figcaption>
            <div class="bench-scroll">
                <table class="bench-table">
                    <thead>
                        <tr>
                            <th>"Server"</th>
                            <th>"Role"</th>
                            <th>"Status"</th>
                            <th>"Standing"</th>
                            <th>"Links"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {MCP_SERVERS
                            .iter()
                            .map(|s| {
                                let slug = s.name.replace(' ', "-");
                                let display = mcp_display_name(s.name);
                                view! {
                                    <tr>
                                        <td>
                                            <a href=format!("/solutions/mcp/{slug}")>{display}</a>
                                        </td>
                                        <td>{s.role}</td>
                                        <td>{s.status}</td>
                                        <td>{s.detail}</td>
                                        <td>
                                            {s.href.map(|href| view! { <a href=href>"GitHub"</a> })}
                                            {(s.href.is_some() && s.registry.is_some())
                                                .then(|| view! { <span>" · "</span> })}
                                            {s.registry.map(|href| view! { <a href=href>"Registry"</a> })}
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
            <figcaption class="bench-note">
                "Verified against the estate census, 2026-10-20. “Published” means listed on the official MCP Registry; “Live” means running and probed on our infrastructure. Each server's name links to its page: the full client config and tool list."
            </figcaption>
        </figure>
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
        assert_eq!(counts, vec![8, 11, 18, 6]);
        for (i, s) in SOLUTIONS.iter().enumerate() {
            assert!(s.cardinal < CARDINALS.len(), "{}", s.name);
            assert!(
                s.href.starts_with("https://github.com/qompassai/")
                    || s.href.starts_with("https://luarocks.org/modules/phaedrusflow/"),
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
            ("blaze.nvim", "blaze-nvim"),
            ("blaze.nvim (LuaRocks)", "blaze-nvim-luarocks"),
            ("phlow MCP server", "phlow-mcp-server"),
            ("blaze-ts.nvim", "blaze-ts-nvim"),
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
            .filter(|s| !s.href.starts_with(GITHUB))
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
    fn marks_exist_only_for_the_twelve_project_images() {
        let marked: Vec<&str> = SOLUTIONS.iter().filter(|s| s.mark().is_some()).map(|s| s.slug).collect();
        assert_eq!(marked, ["vongola", "fathom", "diver", "phlow-mcp-server", "bunker"]);
        for mark in SOLUTIONS.iter().filter_map(Solution::mark) {
            let path = format!("{}/public{mark}", env!("CARGO_MANIFEST_DIR"));
            assert!(std::path::Path::new(&path).is_file(), "{path}");
        }
    }

    #[test]
    fn the_mcp_table_is_alphabetical_unique_and_in_vocabulary() {
        assert_eq!(MCP_SERVERS.len(), 16);
        let vocab = ["Commissioned", "Design stage", "External", "In development", "Legacy", "Live", "Published"];
        for (i, s) in MCP_SERVERS.iter().enumerate() {
            assert!(vocab.contains(&s.status), "{}: {}", s.name, s.status);
            assert!(!s.role.is_empty() && !s.detail.is_empty(), "{}", s.name);
            if let Some(href) = s.href {
                assert!(href.starts_with("https://github.com/qompassai/"), "{}: {href}", s.name);
            }
            if let Some(registry) = s.registry {
                assert!(
                    registry.starts_with("https://registry.modelcontextprotocol.io/"),
                    "{}: {registry}",
                    s.name
                );
                assert_eq!(s.status, "Published", "{}", s.name);
            }
            if i > 0 {
                assert!(MCP_SERVERS[i - 1].name < s.name, "{}", s.name);
            }
        }
    }

    #[test]
    fn mcp_display_names_capitalize_every_word() {
        assert_eq!(mcp_display_name("fathom"), "Fathom");
        assert_eq!(mcp_display_name("ghidra bridge"), "Ghidra Bridge");
        assert_eq!(mcp_display_name("vongola"), "Vongola");
    }

    #[test]
    fn the_mcp_table_marks_the_registry_seven_published() {
        for name in ["eigenstyle", "fathom", "lumen", "lupin", "periscope", "phaedrus", "phlow"] {
            let s = MCP_SERVERS.iter().find(|s| s.name == name).expect(name);
            assert_eq!(s.status, "Published", "{name}");
            assert!(s.href.is_some(), "{name}");
            assert!(s.registry.is_some(), "{name}");
        }
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
