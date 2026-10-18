//! Batch 6 concept diagrams — the shared diagrams for generated
//!
//! solution pages (no guide TOML): the MCP session for the MCP
//! servers, Fathom's pipeline, and the Diver module map for the ten
//! Diver rocks. Owned ids are listed in `IDS`; the registry in
//!
//! `diagrams.rs` delegates here. Follow the authoring contract in
//! that module's header: `gfx-*` classes only, text at 10.5px or
//! larger at viewBox scale, motion only via the stylesheet's
//! reduced-motion-gated classes, and every figure must read
//! correctly as a static diagram.

use leptos::prelude::*;

/// The diagram ids this module owns.
pub const IDS: &[&str] = &["diver-modules", "fathom-pipeline", "mcp-flow"];

/// The bare `<svg>` for one of this module's ids, or `None`.
pub fn diagram(id: &'static str) -> Option<AnyView> {
    match id {
        "mcp-flow" => Some(mcp_flow()),
        "fathom-pipeline" => Some(fathom_pipeline()),
        "diver-modules" => Some(diver_modules()),
        _ => None,
    }
}

/// The MCP session: a host's client and a server speak JSON-RPC 2.0
/// over stdio — initialize, then tool discovery and calls. This is
///
/// the shared, defining concept of every MCP server page it appears
/// on; the per-server tools differ, the protocol does not.
fn mcp_flow() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 356"
            role="img"
            aria-label="Diagram: an MCP session — the host application and its MCP client exchange JSON-RPC 2.0 messages with the server over stdio: initialize, notifications/initialized, tools/list, then tools/call and its result."
        >
            <rect class="gfx-panel" x="8" y="40" width="180" height="120" rx="6" />
            <text class="gfx-head" x="98" y="66" text-anchor="middle">
                "HOST"
            </text>
            <text class="gfx-text" x="98" y="94" text-anchor="middle">
                "editor · agent app"
            </text>
            <text class="gfx-note" x="98" y="120" text-anchor="middle">
                "the user works here"
            </text>
            <text class="gfx-note" x="98" y="140" text-anchor="middle">
                "approves tool calls"
            </text>
            <rect class="gfx-panel" x="252" y="40" width="180" height="120" rx="6" />
            <text class="gfx-head" x="342" y="66" text-anchor="middle">
                "MCP CLIENT"
            </text>
            <text class="gfx-text" x="342" y="94" text-anchor="middle">
                "one per server"
            </text>
            <text class="gfx-note" x="342" y="120" text-anchor="middle">
                "spawns the server"
            </text>
            <text class="gfx-note" x="342" y="140" text-anchor="middle">
                "frames the messages"
            </text>
            <rect class="gfx-hot" x="496" y="40" width="256" height="120" rx="6" />
            <text class="gfx-head" x="624" y="66" text-anchor="middle">
                "MCP SERVER — THIS PAGE"
            </text>
            <text class="gfx-text" x="624" y="94" text-anchor="middle">
                "tools · resources · prompts"
            </text>
            <text class="gfx-note" x="624" y="120" text-anchor="middle">
                "does the work, returns results"
            </text>
            <text class="gfx-note" x="624" y="140" text-anchor="middle">
                "never reaches past its contract"
            </text>
            <line class="gfx-arrow" x1="196" y1="100" x2="244" y2="100" />
            <line class="gfx-arrow" x1="440" y1="100" x2="488" y2="100" />
            <circle class="gfx-pulse" cx="444" cy="100" r="3.5" />
            <text class="gfx-note" x="467" y="88" text-anchor="middle">
                "stdio"
            </text>
            <text class="gfx-head" x="380" y="196" text-anchor="middle">
                "THE SESSION, IN ORDER"
            </text>
            <text class="gfx-text" x="60" y="228">
                "1 · initialize →"
            </text>
            <text class="gfx-note" x="216" y="228">
                "protocol version + capabilities exchanged"
            </text>
            <text class="gfx-text" x="60" y="256">
                "2 · notifications/initialized →"
            </text>
            <text class="gfx-note" x="330" y="256">
                "the client is ready"
            </text>
            <text class="gfx-text" x="60" y="284">
                "3 · tools/list →  ← tool schemas"
            </text>
            <text class="gfx-note" x="390" y="284">
                "names, descriptions, input schemas"
            </text>
            <text class="gfx-text" x="60" y="312">
                "4 · tools/call →  ← result"
            </text>
            <text class="gfx-note" x="330" y="312">
                "the work itself, as structured content"
            </text>
            <rect class="gfx-badge" x="180" y="322" width="400" height="28" rx="6" />
            <text class="gfx-badge-text" x="380" y="341" text-anchor="middle">
                "EVERY MESSAGE IS A JSON-RPC 2.0 ENVELOPE"
            </text>
        </svg>
    }
    .into_any()
}

/// Fathom: a target flows through the provider engines that fit its
/// kind into evidence envelopes — observed, derived, or honestly
/// not established — served over MCP and the CLI.
fn fathom_pipeline() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 380"
            role="img"
            aria-label="Diagram: Fathom's pipeline — native binaries, JavaScript bundles, .NET assemblies, and observed processes flow through provider engines (Ghidra and goblin, oxc, an ECMA-335 reader, process/HAR/EVM observation) into fail-closed evidence envelopes, served over MCP and the CLI."
        >
            <rect class="gfx-panel" x="8" y="56" width="168" height="200" rx="6" />
            <text class="gfx-head" x="92" y="82" text-anchor="middle">
                "TARGET"
            </text>
            <text class="gfx-text" x="24" y="112">
                "ELF / PE binary"
            </text>
            <text class="gfx-text" x="24" y="140">
                "JS bundle · ASAR"
            </text>
            <text class="gfx-text" x="24" y="168">
                ".NET assembly"
            </text>
            <text class="gfx-text" x="24" y="196">
                "live process"
            </text>
            <text class="gfx-note" x="24" y="230">
                "ground truth is"
            </text>
            <text class="gfx-note" x="24" y="246">
                "the artifact itself"
            </text>
            <line class="gfx-arrow" x1="184" y1="156" x2="220" y2="156" />
            <circle class="gfx-pulse" cx="188" cy="156" r="3.5" />
            <rect class="gfx-panel" x="228" y="56" width="252" height="200" rx="6" />
            <text class="gfx-head" x="354" y="82" text-anchor="middle">
                "PROVIDER ENGINES"
            </text>
            <text class="gfx-text" x="244" y="112">
                "native — Ghidra headless · goblin"
            </text>
            <text class="gfx-text" x="244" y="140">
                "js — oxc parser · ASAR reader"
            </text>
            <text class="gfx-text" x="244" y="168">
                "managed — ECMA-335 metadata"
            </text>
            <text class="gfx-text" x="244" y="196">
                "observe — process · HAR · EVM"
            </text>
            <text class="gfx-note" x="244" y="230">
                "each engine declares what"
            </text>
            <text class="gfx-note" x="244" y="246">
                "it does not implement"
            </text>
            <line class="gfx-arrow" x1="488" y1="156" x2="524" y2="156" />
            <circle
                class="gfx-pulse"
                cx="492"
                cy="156"
                r="3.5"
                style="animation-delay:1s"
            />
            <rect class="gfx-hot" x="532" y="56" width="220" height="200" rx="6" />
            <text class="gfx-head" x="642" y="82" text-anchor="middle">
                "EVIDENCE ENVELOPE"
            </text>
            <text class="gfx-text" x="548" y="112">
                "observed — measured"
            </text>
            <text class="gfx-text" x="548" y="140">
                "derived — shown how"
            </text>
            <text class="gfx-text" x="548" y="168">
                "not established — said so"
            </text>
            <g class="gfx-decide">
                <rect class="gfx-badge" x="548" y="190" width="188" height="40" rx="6" />
                <text class="gfx-badge-text" x="642" y="215" text-anchor="middle">
                    "FAIL-CLOSED"
                </text>
            </g>
            <text class="gfx-note" x="642" y="248" text-anchor="middle">
                "truncated input is an error"
            </text>
            <line class="gfx-arrow" x1="642" y1="264" x2="642" y2="292" />
            <circle class="gfx-pulse-y" cx="642" cy="268" r="3.5" />
            <rect class="gfx-chip" x="452" y="296" width="180" height="36" rx="6" />
            <text class="gfx-text" x="542" y="319" text-anchor="middle">
                "MCP — stdio + HTTP"
            </text>
            <rect class="gfx-chip" x="644" y="296" width="108" height="36" rx="6" />
            <text class="gfx-text" x="698" y="319" text-anchor="middle">
                "CLI"
            </text>
            <text class="gfx-note" x="380" y="356" text-anchor="middle">
                "the same envelopes reach agents (MCP, A2A) and the operator (CLI) unchanged"
            </text>
        </svg>
    }
    .into_any()
}

/// The ten Diver module rocks: one Neovim environment, split into
/// separately published subsystems. Roles are the registry's own
/// one-line descriptions of each rock.
fn diver_modules() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 320"
            role="img"
            aria-label="Diagram: the ten Diver LuaRocks modules — AI, Comms, Config, DAP, Dev, Games, Lintfmt, Research, Security, Utils — compose the single Diver Neovim environment."
        >
            <rect class="gfx-hot" x="230" y="16" width="300" height="52" rx="6" />
            <text class="gfx-head" x="380" y="40" text-anchor="middle">
                "DIVER"
            </text>
            <text class="gfx-note" x="380" y="58" text-anchor="middle">
                "one Neovim environment"
            </text>
            <line class="gfx-arrow" x1="380" y1="68" x2="380" y2="88" />
            <line class="gfx-arrow" x1="76" y1="88" x2="684" y2="88" />
            <rect class="gfx-panel" x="8" y="96" width="136" height="84" rx="6" />
            <text class="gfx-text" x="76" y="122" text-anchor="middle">
                "Diver-AI"
            </text>
            <text class="gfx-note" x="76" y="144" text-anchor="middle">
                "ai.* harness:"
            </text>
            <text class="gfx-note" x="76" y="160" text-anchor="middle">
                "registry · policy"
            </text>
            <rect class="gfx-panel" x="156" y="96" width="136" height="84" rx="6" />
            <text class="gfx-text" x="224" y="122" text-anchor="middle">
                "Diver-Comms"
            </text>
            <text class="gfx-note" x="224" y="144" text-anchor="middle">
                "mail clients:"
            </text>
            <text class="gfx-note" x="224" y="160" text-anchor="middle">
                "aerc · neomutt"
            </text>
            <rect class="gfx-panel" x="304" y="96" width="136" height="84" rx="6" />
            <text class="gfx-text" x="372" y="122" text-anchor="middle">
                "Diver-Config"
            </text>
            <text class="gfx-note" x="372" y="144" text-anchor="middle">
                "config.ui.* library:"
            </text>
            <text class="gfx-note" x="372" y="160" text-anchor="middle">
                "icons · themes"
            </text>
            <rect class="gfx-panel" x="452" y="96" width="136" height="84" rx="6" />
            <text class="gfx-text" x="520" y="122" text-anchor="middle">
                "Diver-DAP"
            </text>
            <text class="gfx-note" x="520" y="144" text-anchor="middle">
                "debug adapters:"
            </text>
            <text class="gfx-note" x="520" y="160" text-anchor="middle">
                "session engine"
            </text>
            <rect class="gfx-panel" x="600" y="96" width="152" height="84" rx="6" />
            <text class="gfx-text" x="676" y="122" text-anchor="middle">
                "Diver-Dev"
            </text>
            <text class="gfx-note" x="676" y="144" text-anchor="middle">
                "dev.* tooling:"
            </text>
            <text class="gfx-note" x="676" y="160" text-anchor="middle">
                "BSP · SCIP · browser"
            </text>
            <rect class="gfx-panel" x="8" y="196" width="136" height="84" rx="6" />
            <text class="gfx-text" x="76" y="222" text-anchor="middle">
                "Diver-Games"
            </text>
            <text class="gfx-note" x="76" y="244" text-anchor="middle">
                "engine tooling:"
            </text>
            <text class="gfx-note" x="76" y="260" text-anchor="middle">
                "Godot · Unity · …"
            </text>
            <rect class="gfx-panel" x="156" y="196" width="136" height="84" rx="6" />
            <text class="gfx-text" x="224" y="222" text-anchor="middle">
                "Diver-Lintfmt"
            </text>
            <text class="gfx-note" x="224" y="244" text-anchor="middle">
                "138 formatter"
            </text>
            <text class="gfx-note" x="224" y="260" text-anchor="middle">
                "adapters + engine"
            </text>
            <rect class="gfx-panel" x="304" y="196" width="136" height="84" rx="6" />
            <text class="gfx-text" x="372" y="222" text-anchor="middle">
                "Diver-Research"
            </text>
            <text class="gfx-note" x="372" y="244" text-anchor="middle">
                "journals · ORCID"
            </text>
            <text class="gfx-note" x="372" y="260" text-anchor="middle">
                "Zotero · Zenodo"
            </text>
            <rect class="gfx-panel" x="452" y="196" width="136" height="84" rx="6" />
            <text class="gfx-text" x="520" y="222" text-anchor="middle">
                "Diver-Security"
            </text>
            <text class="gfx-note" x="520" y="244" text-anchor="middle">
                "guards · red/blue"
            </text>
            <text class="gfx-note" x="520" y="260" text-anchor="middle">
                "suites · pass"
            </text>
            <rect class="gfx-panel" x="600" y="196" width="152" height="84" rx="6" />
            <text class="gfx-text" x="676" y="222" text-anchor="middle">
                "Diver-Utils"
            </text>
            <text class="gfx-note" x="676" y="244" text-anchor="middle">
                "utils.* foundations"
            </text>
            <text class="gfx-note" x="676" y="260" text-anchor="middle">
                "shared library"
            </text>
            <text class="gfx-note" x="380" y="306" text-anchor="middle">
                "each rock is published on LuaRocks and versioned on its own; the environment pins the set"
            </text>
        </svg>
    }
    .into_any()
}
