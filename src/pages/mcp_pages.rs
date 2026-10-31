//! Per-server pages — `/solutions/mcp/<slug>` — one for every server
//! in the MCP estate table: its standing, the exact client config
//! where a launch command exists, and its tool list.
//!
//! The data mirrors the internal census (`SERVERS.md`, verified
//! 2026-10-20) and the commissioning packages for the servers the
//! census predates (broadside, tackle, atelier). Vongola, restored
//! 2026-10-20, is the exception the other way: the census omitted
//! it when this page was first built (the census has since been
//! corrected), so its launch, tools, and descriptions are read
//! verbatim from the server's own source (crates/vongola-mcp in
//! its repo). A server with no launch command says so in plain
//! words — no invented configs. Tool names are the census lists;
//! descriptions are included only where the census or the
//! commissioning package states them (fathom's full documentation
//! lives in `fathom_doc`). Each page closes with a "Spec
//! compliance" section: the canonical MCP checklist from the
//! 2026-10-20 Inspector sweep, with this server's status per row
//! (data in `compliance_data`, generated from the sweep evidence).

use crate::pages::compass_services::{mcp_display_name, McpServer, McpServers, MCP_SERVERS};
use crate::pages::compliance_data::{compliance_for, status_word, ChecklistRow, CHECKLIST_ROWS};
use crate::pages::fathom_doc::FathomDoc;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

/// One tool on a server's page.
pub struct McpTool {
    pub name: &'static str,
    pub desc: &'static str,
}

const fn tool(name: &'static str) -> McpTool {
    McpTool { name, desc: "" }
}

const fn tool_d(name: &'static str, desc: &'static str) -> McpTool {
    McpTool { name, desc }
}

/// The page data for one server: how to launch it (if it can be
/// launched), what its tools are, and the honest standing prose.
pub struct McpDetail {
    pub slug: &'static str,
    pub launch: Option<(&'static str, &'static [&'static str])>,
    pub no_launch: Option<&'static str>,
    pub tools: &'static [McpTool],
    pub tools_note: Option<&'static str>,
    pub note: &'static str,
}

const fn detail(
    slug: &'static str,
    launch: Option<(&'static str, &'static [&'static str])>,
    no_launch: Option<&'static str>,
    tools: &'static [McpTool],
    tools_note: Option<&'static str>,
    note: &'static str,
) -> McpDetail {
    McpDetail { slug, launch, no_launch, tools, tools_note, note }
}

pub const MCP_DETAILS: &[McpDetail] = &[
    detail(
        "atelier",
        None,
        Some(
            "Atelier was folded into Lumen on 2026-10-27, before the standalone server \
             shipped: its image lane lives on as part of lumen 1.0.0. There is no atelier \
             binary and no launch command — this page keeps the design on record.",
        ),
        &[
            tool_d("atelier_status", "Server, active lane profile, backend, and queue state"),
            tool_d("atelier_models", "The admitted model roster for the implemented lanes"),
            tool_d("atelier_txt2img", "Text-to-image through the pinned SDXL workflow (iris)"),
            tool_d("atelier_img2img", "Image-to-image from one admitted source image (iris)"),
        ],
        Some("The S1 tool surface, as designed in the S1 brief. S1 implements the image lane (iris) only; the video lane (vera) and music lane (aria) are S2 and S3."),
        "One Rust MCP server, two config-selected lane profiles: qompass (SFW, Qompass roots) \
         and ntranch (adult work for Strobe, NTRanch roots) — a caller cannot switch profiles \
         per request, and every output is stamped with its profile. ComfyUI runs behind a \
         backend trait, started on demand on the A6000, loopback only, and stopped after the \
         job; atelier never touches rose. S1 acceptance is the Strobe generation test: three \
         catalogued scenes regenerated through atelier_img2img under the ntranch profile. \
         That fold has now happened: the generation module is part of lumen 1.0.0 \
         (2026-10-27), and this page remains as the design record.",
    ),
    detail(
        "bloch",
        None,
        Some(
            "Bloch runs as a stdio MCP server from its repository (cargo build, then the \
             bloch binary). This page does not reproduce a client config it has not \
             verified against the current release; the repo carries the launch.",
        ),
        &[
            tool("idea_lookup"),
            tool("idea_register"),
            tool("lessons_search"),
            tool("memory_get"),
            tool("memory_promote"),
            tool("memory_propose"),
            tool("memory_search"),
            tool("roadmap_rollup"),
            tool("roadmap_state"),
            tool("tracker_rollup"),
            tool("tracker_state"),
        ],
        Some("The 0.4.0 surface, verified from the migration gate evidence (2026-10-27): tools array 7,589 canonical chars, byte-identical across the rmcp migration."),
        "The estate's learning record: roadmaps with stable IDs, an idea register, and \
         the lesson loop every other server files into — memory_propose is the only write \
         channel (inbox only, fencing- and lease-checked), memory_promote is a gated \
         Owner/Pax act, and lessons carry their lane labels from birth. Bloch is also \
         the fleet's lesson home under the lane law: an unclassified NTRanch-origin \
         lesson defaults to nsfw and never crosses to a Qompass repo on its own.",
    ),
    detail(
        "broadside",
        None,
        Some(
            "Broadside is commissioned, not built: the study, program, and honeypot design are \
             complete and the build stages are queued. There is no binary and no launch \
             command yet.",
        ),
        &[
            tool_d("broadside_status", "Site tree state, dist freshness, last deployed hash"),
            tool_d("broadside_live_check", "Fetch the public pages and the published JWKS; compare status and content hashes"),
            tool_d("broadside_build", "Build the site tree into dist, with a build receipt"),
            tool_d("broadside_verify", "The gate battery: tests, data freshness, citation verification, the JWKS against the signed key record"),
            tool_d("broadside_deploy", "Deploy one dist hash, only against a passing verify receipt for that hash"),
        ],
        Some("The designed tool surface from the broadside program (P1–P3)."),
        "This website's own pipeline as an MCP server: build, verify, and deploy qompass.ai, \
         with an append-only receipt log that vongola reads (vongola stays read-only; broadside \
         holds the deploy key). The design includes a honeypot: canary endpoints and \
         credentials, injection quarantine in the fetch pipeline, and hash-chained evidence \
         packets — and no hack-back. Broadside also carries the estate's JWKS duty: it \
         publishes the Agent Card JWKS at /.well-known/jwks.json in the site dist as a \
         signed-manifest artifact, placed at build from the signed key record, and \
         broadside_verify fails the build if the published document does not match that \
         record — a swapped key file is a failed build, never a quiet edit. Signing itself \
         stays with the Owner's Ed25519 key; its private half never enters broadside or the \
         site host.",
    ),
    detail(
        "buccaneer",
        None,
        Some(
            "Buccaneer is deferred: the tool surface below is source-verified, but no built \
             binary exists yet, so there is nothing to launch. It joins the runnable set when \
             the v1 gate lands.",
        ),
        &[tool("buccaneer_scan_start"), tool("buccaneer_findings_list"), tool("buccaneer_run_record")],
        Some("Source-verified from the server's MCP module; not yet live-probed."),
        "A security-assessment engine: rose-first LLM backend, a deterministic allow/deny gate \
         over declared scope, fathom chaining for evidence, SARIF output, and no telemetry or \
         egress beyond the target and the model. Skill packs declare vulnerability or \
         conformance objectives. Slice 1 is closed out; the Stage 6 decision is pending.",
    ),
    detail(
        "bunker",
        None,
        Some(
            "Bunker's MCP server (Stage C) is under construction; there is no launch command \
             yet. The pilot action is a gated artifact_delete under the signed-authority \
             protocol; the tool names land with the build.",
        ),
        &[],
        None,
        "The executor end of the signed-authority chain: OpenAPPA/flowguard labels decide the \
         flow (a block is final), a human signs one canonical manifest per action, and the \
         executor verifies the signature, checks revocation freshness, consumes the nonce, \
         and re-runs the decision at dispatch time. Consequential actions — delete, publish, \
         send — exist only inside that envelope.",
    ),
    detail(
        "eigenstyle",
        None,
        Some(
            "Eigenstyle is distributed as an MCPB bundle through the official MCP Registry \
             (ai.qompass/eigenstyle 0.1.1) rather than as a locally launched binary; no local \
             launch command is published. Install it from the registry record linked above.",
        ),
        &[
            tool("check_draft"),
            tool("get_exemplars"),
            tool("get_profile"),
            tool("ingest_sample"),
            tool("list_corpus"),
            tool("recompute_fingerprint"),
            tool("specs"),
        ],
        Some("Source-verified 2026-10-20; the server is now 1.0.0, landed on GitHub main and published on crates.io as eigenstyle 1.0.0."),
        "Voice and style fingerprinting: an exemplar bank and a measured fingerprint of the \
         owner's writing, checked against drafts before they go out. Published on the MCP \
         Registry as a bundle; the estate's own binary build is deferred.",
    ),
    detail(
        "fathom",
        Some(("/home/phaedrus/.GH/Qompass/fathom/target/release/fathom", &["serve"])),
        None,
        &[],
        None,
        "Reverse engineering as an MCP server: artifact census, native analysis over headless \
         Ghidra, managed and JavaScript analysis, IL2CPP work, and evidence bundles sealed by \
         digest. Fathom's full tool documentation — live, building, and planned — is on this \
         page, below the config.",
    ),
    detail(
        "ghidra-bridge",
        Some(("/usr/bin/bridge-mcp-ghidra", &[])),
        None,
        &[],
        Some(
            "The bridge forwards the GhidraMCP plugin's tool surface — 279 documented tools in \
             the v6.0.0 census — from inside a running Ghidra GUI. The full census is tracked, \
             tool by tool, in fathom's parity documentation.",
        ),
        "An upstream stdio bridge, declared in Diver: it fronts the GhidraMCP plugin's HTTP \
         surface (127.0.0.1:8089) inside a running Ghidra GUI, so it is not launchable \
         headless — the config starts the bridge, and the bridge answers when a Ghidra GUI \
         with the plugin is up. Not a Qompass product; listed because the estate declares and \
         uses it. On record: the bridge itself has no authentication — it inherits the \
         loopback trust of the GUI it fronts.",
    ),
    detail(
        "lumen",
        Some(("/home/phaedrus/.GH/Qompass/lumen/target/debug/lumen", &[])),
        None,
        &[
            tool("bevy_discover"),
            tool("sprite_info"),
            tool("style_apply"),
            tool("style_list"),
            tool("validate_atlas"),
            tool("vrm_grade"),
            tool("vrm_inspect"),
        ],
        Some("Live-probed 2026-10-20 against the debug build (7 tools). The server is now 1.0.0 — 15 tools, atelier folded in, rmcp modern-era only; the added image-lane tools are recorded in the repo, and this list is the probed 2026-10-20 surface."),
        "The sprite-to-3D character factory: ingest a 2D sprite, lock the character's \
         identity, project it onto the canonical rigged base with a blendshape expression \
         set, edit body and appearance parametrically, and export glTF/GLB to Blender, \
         Unity, Unreal, and Godot. Fathom chains into lumen (apply_style_via_lumen). Launch \
         quirk: no subcommand starts the MCP stdio server; subcommands are the pipeline CLI.",
    ),
    detail(
        "lupin",
        Some(("/home/hatch/workspace/revenue-alignment-20261019/server/target/release/lupin", &[])),
        None,
        &[
            tool("opportunity_list"),
            tool("opportunity_get"),
            tool("opportunity_submit"),
            tool("capability_list"),
            tool("capability_match"),
            tool("pipeline_summary"),
        ],
        Some("Live-probed 2026-10-20."),
        "Revenue alignment: a scanned opportunity pipeline (scan, align, validate, refine, \
         queue) with the Qompass (SFW) and NTRanch lanes hard-separated server-side — only \
         the operator changes a lane. Phaedrus files research findings into lupin as drafts.",
    ),
    detail(
        "periscope",
        Some(("/home/phaedrus/.local/bin/periscope", &[])),
        None,
        &[
            tool("attention_alignment"),
            tool("attention_decide"),
            tool("attention_focus"),
            tool("attention_publish"),
            tool("broker_status"),
            tool("get_definition"),
            tool("get_diagnostics"),
            tool("get_document_symbols"),
            tool("get_hover"),
            tool("get_references"),
            tool("list_dap"),
            tool("list_formatters"),
            tool("list_linters"),
            tool("list_servers"),
            tool("run_format"),
            tool("run_lint"),
        ],
        Some("Live-probed 2026-10-20."),
        "The Neovim broker: editor state, language-server and debug-adapter brokering, and \
         the MATE shared-attention protocol. The attention_* family is shared with phlow — \
         one protocol, two servers. Declared in Diver.",
    ),
    detail(
        "phaedrus",
        Some(("/home/hatch/workspace/research-feed-20261019/server/target/release/phaedrus", &[])),
        None,
        &[
            tool("finding_list"),
            tool("finding_get"),
            tool("research_submit"),
            tool("watchlist_list"),
            tool("capability_context"),
            tool("revenue_candidate_draft"),
            tool("feed_summary"),
        ],
        Some("Live-probed 2026-10-20."),
        "The research feed: findings from watched systems, filed with evidence, drafted into \
         revenue candidates for lupin. Read-mostly by design — it observes and drafts; it \
         does not act.",
    ),
    detail(
        "phlow",
        Some(("/home/phaedrus/.local/bin/phlow", &["serve"])),
        None,
        &[
            tool("flow_run"),
            tool("flow_status"),
            tool("flow_check"),
            tool("flow_attention_alignment"),
            tool("flow_attention_decide"),
            tool("flow_attention_focus"),
            tool("flow_attention_publish"),
        ],
        Some("Live-probed 2026-10-20. The server answers to the name “flow”."),
        "The agent runtime: bounded planner, coder, validation, and reviewer workflows over \
         one safe core shared by the CLI, TUI, and MCP surfaces, with operator-approved \
         checks. Note for clients: phlow requires full initialize parameters (protocol \
         version, capabilities, client info); an empty initialize gets no answer.",
    ),
    detail(
        "seahawk",
        None,
        Some(
            "Seahawk is a lab broker: it runs on Qompass infrastructure in front of a \
             live Unreal Editor, under token auth. It is not distributed as a bundle, \
             and this page does not publish a client config for it.",
        ),
        &[
            tool("gui_act_verified"),
            tool("gui_ground"),
            tool("gui_observe"),
        ],
        Some("Seahawk's own surface is 14 tools: the brokered Unreal toolset calls (reached through its curated, decide()-gated surface — the downstream census counted 830 tools across 52 toolsets) plus the three gui_* computer-use tools above, gated and benchmarked in the S5 round (2026-10-28)."),
        "The estate's born-modern broker for Unreal Engine 5.8's built-in MCP server: \
         token auth, a curated tool surface, execute-class tools default-deny through \
         a deterministic decide() gate, and an append-only operation log. S4 opened \
         computer use across the stack through the same gated surface. Lab-only; \
         registry publication is not authorized, and the broker cannot firewall \
         Epic's raw editor port — lab discipline is the recorded mitigation.",
    ),
    detail(
        "slipway",
        None,
        Some(
            "Slipway runs as a stdio MCP server from its repository or the crates.io \
             slipway-mcp package. This page does not reproduce a client config it has \
             not verified against the current release; the repo carries the launch.",
        ),
        &[
            tool("slipway_build"),
            tool("slipway_bundle_check"),
            tool("slipway_certify"),
            tool("slipway_device_close"),
            tool("slipway_device_open"),
            tool("slipway_device_verify"),
            tool("slipway_drive_app"),
            tool("slipway_inspect"),
        ],
        Some("Tool names read from the server source (2026-10-28); the published record is crates.io slipway-mcp 1.0.0 (the registry entry ai.qompass/slipway remains 0.1.0)."),
        "Release certification as an MCP server: build a release bundle, check it, \
         and certify it against the channel gates (Google Play, F-Droid, itch) with \
         device verification. Its first witness-bundle consumer is ache; the \
         light-show Play AAB vs F-Droid APK comparison is its L2 consumer. Four \
         certifications (light-show and ONTrack, Play and F-Droid) remain honestly \
         not_certified, blocked on owner-side items recorded in the repo.",
    ),
    detail(
        "tackle",
        None,
        Some(
            "Tackle is built but not yet landed: the U2 write core is gated (crate 0.3.0, \
             2026-10-28) and its landing follows the phlow migration. This page publishes \
             the standing; no client config is shown until the landed release is probed.",
        ),
        &[
            tool_d("tackle_editor_state", "Editor connection state; a clear “no editor connected” when the session is absent"),
            tool_d("tackle_scene_hierarchy", "The connected editor's scene hierarchy"),
        ],
        Some("The designed U0 tool surface from the spike brief."),
        "The game-editor broker: a Rust MCP server and a C# editor package joined by a \
         capability-file handshake on loopback (127.0.0.1:9770), with idempotency and undo \
         participation designed in from the study of godot-ai and unity-mcp. Unity first, \
         Godot later. Lumen consumes tackle; tackle never hosts lumen's work.",
    ),
    detail(
        "theodolite",
        None,
        Some(
            "Theodolite runs as a stdio MCP server from its repository, with a bridge \
             config (bridge.toml) naming the language servers it pins. This page does \
             not reproduce a client config it has not verified against the current \
             release; the repo carries the launch and the example config.",
        ),
        &[
            tool("code_actions"),
            tool("declaration"),
            tool("definition"),
            tool("diagnostics"),
            tool("document_symbols"),
            tool("hover"),
            tool("implementation"),
            tool("references"),
            tool("session_status"),
            tool("type_definition"),
            tool("workspace_symbols"),
        ],
        Some("The read surface, from the server's README (2026-10-28): diagnostics are push-accumulated and merged with provenance. Mutating actions (code_action_apply, rename, edit) exist only behind the server's mutation allowlist and are not part of the read surface listed here."),
        "Periscope's read-only complement: periscope brokers LSP through a live \
         Neovim running the Diver config; theodolite serves editor-free contexts — \
         CI, brokers like seahawk, and headless gates — from pinned language \
         servers. Read-only by design in its first version: it has no format or \
         lint tool, deliberately.",
    ),
    detail(
        "twilight",
        Some(("/home/phaedrus/twilight/result/bin/twilight", &[])),
        None,
        &[tool("engine_detect"), tool("engine_health"), tool("twilight_info")],
        Some("Live-probed 2026-10-20. All three A1 tools are read-only."),
        "The front desk for local inference engines: it detects which engine answers on a \
         host by API signature (never by open port), reports engine health, and identifies \
         itself. Stage A1 of the studio-parity program; the catalog, completion, and Create \
         stages follow. Rose is the default engine. Config is fail-closed TOML \
         ($TWILIGHT_CONFIG or ~/.config/twilight/twilight.toml).",
    ),
    detail(
        "volta",
        None,
        Some(
            "Volta is at design stage: no repository exists yet, so there is nothing to \
             launch and no tool surface to list. The design is complete; the build follows \
             the bunker pilot.",
        ),
        &[],
        None,
        "The key service for the signed-authority chain: it serves the action-signing \
         subkey with tiered revocation freshness, so executors like bunker can verify a \
         signed manifest against a key whose revocation state is at most a minute old for \
         the most consequential classes.",
    ),
    detail(
        "vongola",
        Some(("/home/phaedrus/workspace/repos/vongola/target/debug/vongola-mcp", &[])),
        None,
        &[
            tool_d("cert_status", "Per-SNI certificate subject, issuer, notAfter, and days remaining; flags anything inside 30 days."),
            tool_d("log_counters", "Emitted log lines by level and source (vongola, pingora, other), plus lines dropped by the bounded log pipeline."),
            tool_d("routes_list", "Configured routes with their upstreams, chains, and SNI names."),
            tool_d("tls_stats", "TLS handshakes by negotiated group (PQ hybrids by name, classical groups), classical-fallback rate, and handshake failures by class."),
            tool_d("upstream_health", "Per-upstream health (up/down/unknown) and upstream latency summary from the daemon's histogram."),
        ],
        Some("All five tools are read-only. The server answers to the name “vongola-mcp”, version 0.1.0; a debug build is what exists locally."),
        "The operator's window into a running vongola daemon — the estate's TLS \
         gateway. The daemon's instrumentation (TLS handshakes by negotiated group, \
         certificate expiry, upstream health, log volume) is served to an \
         operator-local stdio client; the server opens no listener and carries no \
         write tools. Its only outbound traffic is loopback reads of the daemon's \
         admin surfaces, and when the daemon is unreachable the tools answer with a \
         structured unavailable error — never cached or invented values. It is also \
         the designated reader of broadside's append-only receipt log, so the site \
         pipeline is observed by a server that cannot act on it. The binary is its \
         own crate in the vongola workspace, so a default vongola build is \
         unchanged. Usage: vongola-mcp [--config PATH], default ./vongola.yaml.",
    ),
];

/// The page data for one roster name, if the roster carries it.
pub fn mcp_detail(slug: &str) -> Option<&'static McpDetail> {
    MCP_DETAILS.iter().find(|d| d.slug == slug)
}

/// The roster slug for a server name ("ghidra bridge" → "ghidra-bridge").
pub fn mcp_slug(name: &str) -> String {
    name.replace(' ', "-")
}

/// The roster entry for a slug, if any.
pub fn mcp_server(slug: &str) -> Option<&'static McpServer> {
    MCP_SERVERS.iter().find(|s| mcp_slug(s.name) == slug)
}

/// The full `mcpServers` JSON for one launchable server.
fn config_json(name: &str, command: &str, args: &[&str]) -> String {
    let args = args.iter().map(|a| format!("\"{a}\"")).collect::<Vec<_>>().join(", ");
    format!(
        "{{\n  \"mcpServers\": {{\n    \"{name}\": {{\n      \"command\": \"{command}\",\n      \"args\": [{args}]\n    }}\n  }}\n}}"
    )
}

/// The "Spec compliance" section closing each page: every row of
/// the canonical checklist, grouped by section, with this server's
/// status from the Inspector sweep evidence. A server the sweep
/// could not probe reads "Not probed" on every row — never an
/// invented pass.
#[component]
#[allow(clippy::unit_arg, reason = "the empty fragment is the deliberate no-record rendering")]
fn ComplianceSection(slug: &'static str) -> impl IntoView {
    let Some(comp) = compliance_for(slug) else {
        return view! { <></> }.into_any();
    };
    let codes: Vec<char> = comp.statuses.chars().collect();
    let mut groups: Vec<(&'static str, Vec<(usize, &'static ChecklistRow)>)> = Vec::new();
    for (i, row) in CHECKLIST_ROWS.iter().enumerate() {
        if let Some((section, rows)) = groups.last_mut()
            && *section == row.section
        {
            rows.push((i, row));
            continue;
        }
        groups.push((row.section, vec![(i, row)]));
    }
    view! {
        <h2>"Spec compliance"</h2>
        <p>{comp.standing}</p>
        <p class="mcp-tools-note">
            "Every row of the estate's canonical MCP checklist (Inspector sweep, 2026-10-20), "
            "with this server's status: Pass, Partial, Gap, Not applicable, or Not probed — "
            "a Not probed row is one the sweep's evidence neither passes nor fails."
        </p>
        {groups
            .into_iter()
            .map(|(section, rows)| {
                view! {
                    <h3>{section}</h3>
                    <figure class="bench">
                        <div class="bench-scroll">
                            <table class="bench-table">
                                <thead>
                                    <tr>
                                        <th>"#"</th>
                                        <th>"Requirement"</th>
                                        <th>"Era"</th>
                                        <th>"Level"</th>
                                        <th>"Status"</th>
                                        <th>"Spec"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {rows
                                        .into_iter()
                                        .map(|(i, row)| {
                                            let code = codes.get(i).copied().unwrap_or('U');
                                            view! {
                                                <tr>
                                                    <td>{row.id}</td>
                                                    <td>{row.requirement}</td>
                                                    <td>{row.era}</td>
                                                    <td>{row.level}</td>
                                                    <td>{status_word(code)}</td>
                                                    <td>
                                                        <a href=row.spec_url>{row.spec_label}</a>
                                                    </td>
                                                </tr>
                                            }
                                        })
                                        .collect_view()}
                                </tbody>
                            </table>
                        </div>
                    </figure>
                }
            })
            .collect_view()}
    }
    .into_any()
}

/// One server's page: standing, config, and tools.
#[component]
fn McpPage(server: &'static McpServer, detail: &'static McpDetail) -> impl IntoView {
    let name = server.name;
    let display = mcp_display_name(name);
    view! {
        <Title text=format!("{display} MCP - Qompass AI") />
        <section class="page mcp-page">
            <p class="page-bearing">"CHART — THE FLEET / MCP SERVERS"</p>
            <h1>{display}</h1>
            <p class="page-lede">{server.role}</p>
            <figure class="bench mcp-facts">
                <div class="bench-scroll">
                    <table class="bench-table">
                        <tbody>
                            <tr>
                                <td>"Status"</td>
                                <td>{server.status}</td>
                            </tr>
                            <tr>
                                <td>"Standing"</td>
                                <td>{server.detail}</td>
                            </tr>
                            {server
                                .href
                                .map(|href| {
                                    view! {
                                        <tr>
                                            <td>"Source"</td>
                                            <td>
                                                <a href=href>"GitHub"</a>
                                            </td>
                                        </tr>
                                    }
                                })}
                            {server
                                .registry
                                .map(|href| {
                                    view! {
                                        <tr>
                                            <td>"Registry"</td>
                                            <td>
                                                <a href=href>"MCP Registry"</a>
                                            </td>
                                        </tr>
                                    }
                                })}
                        </tbody>
                    </table>
                </div>
            </figure>
            <h2>"Connect"</h2>
            {detail
                .launch
                .map(|(command, args)| {
                    let json = config_json(name, command, args);
                    view! {
                        <pre class="mcp-config">
                            <code>{json}</code>
                        </pre>
                    }
                    .into_any()
                })
                .or_else(|| {
                    detail.no_launch.map(|text| {
                        view! { <p>{text}</p> }.into_any()
                    })
                })}
            <p>{detail.note}</p>
            {(detail.slug == "fathom")
                .then(|| {
                    view! { <FathomDoc /> }
                })}
            {(detail.slug != "fathom" && !detail.tools.is_empty())
                .then(|| {
                    view! {
                        <h2>"Tools"</h2>
                        {detail
                            .tools_note
                            .map(|note| view! { <p class="mcp-tools-note">{note}</p> })}
                        <figure class="bench">
                            <div class="bench-scroll">
                                <table class="bench-table">
                                    <thead>
                                        <tr>
                                            <th>"Tool"</th>
                                            <th>"What it does"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {detail
                                            .tools
                                            .iter()
                                            .map(|t| {
                                                view! {
                                                    <tr>
                                                        <td>
                                                            <code>{t.name}</code>
                                                        </td>
                                                        <td>{t.desc}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect_view()}
                                    </tbody>
                                </table>
                            </div>
                        </figure>
                    }
                })}
            {(detail.slug == "ghidra-bridge")
                .then(|| {
                    view! {
                        <p>
                            "The parity census lives with "
                            <a href="/solutions/mcp/fathom">"fathom's tool documentation"</a>
                            "."
                        </p>
                    }
                })}
            <ComplianceSection slug=detail.slug />
            <p class="mcp-back">
                <a href="/solutions/mcp">"← MCP servers"</a>
            </p>
        </section>
    }
}

/// The dispatch: `/solutions/mcp/:name` renders that server's page, or
/// the standard fallback for a name the roster does not carry.
#[component]
pub fn McpDispatch() -> impl IntoView {
    let params = use_params_map();
    let slug = params.with(|map| map.get("name")).unwrap_or_default();
    match (mcp_server(&slug), mcp_detail(&slug)) {
        (Some(server), Some(detail)) => view! { <McpPage server=server detail=detail /> }.into_any(),
        _ => view! {
            <section class="page">
                <h1>"Off chart"</h1>
                <p>"That bearing does not exist on this chart."</p>
            </section>
        }
        .into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_roster_server_has_a_detail_page() {
        assert_eq!(MCP_DETAILS.len(), MCP_SERVERS.len());
        for s in MCP_SERVERS {
            let slug = mcp_slug(s.name);
            assert!(mcp_detail(&slug).is_some(), "{}", s.name);
            assert!(mcp_server(&slug).is_some(), "{}", s.name);
        }
    }

    #[test]
    fn launchable_servers_carry_tools_or_a_tools_note() {
        for d in MCP_DETAILS {
            if d.launch.is_some() && d.slug != "fathom" {
                assert!(!d.tools.is_empty() || d.tools_note.is_some(), "{}", d.slug);
            }
            if d.launch.is_none() {
                assert!(d.no_launch.is_some(), "{}", d.slug);
            }
        }
    }

    #[test]
    fn config_json_is_the_launch_verbatim() {
        let json = config_json("fathom", "/x/fathom", &["serve"]);
        assert!(json.contains("\"command\": \"/x/fathom\""), "{json}");
        assert!(json.contains("\"args\": [\"serve\"]"), "{json}");
    }

    #[test]
    fn every_roster_server_has_a_compliance_record() {
        use crate::pages::compliance_data::COMPLIANCE;
        assert_eq!(COMPLIANCE.len(), MCP_SERVERS.len());
        for s in MCP_SERVERS {
            let slug = mcp_slug(s.name);
            let comp = compliance_for(&slug).unwrap_or_else(|| panic!("{}", s.name));
            assert_eq!(comp.statuses.chars().count(), CHECKLIST_ROWS.len(), "{}", s.name);
            assert!(comp.statuses.chars().all(|c| "PHGNU".contains(c)), "{}", s.name);
            assert!(!comp.standing.is_empty(), "{}", s.name);
        }
    }

    #[test]
    fn compliance_statuses_match_the_sweep_evidence() {
        fn status(slug: &str, row_id: &str) -> char {
            let comp = compliance_for(slug).expect(slug);
            let idx = CHECKLIST_ROWS.iter().position(|r| r.id == row_id).expect(row_id);
            comp.statuses.chars().nth(idx).expect(row_id)
        }
        // Modern-era lifecycle: the dual-era three pass discover,
        // the legacy-only servers gap it.
        for slug in ["phlow", "lumen", "twilight"] {
            assert_eq!(status(slug, "1.1"), 'P', "{slug}");
        }
        for slug in ["fathom", "periscope", "lupin", "phaedrus", "vongola"] {
            assert_eq!(status(slug, "1.1"), 'G', "{slug}");
        }
        // Servers the sweep could not probe are never marked passing.
        for slug in ["atelier", "bloch", "broadside", "buccaneer", "bunker", "seahawk", "slipway", "tackle", "theodolite", "volta", "ghidra-bridge"] {
            assert_eq!(status(slug, "1.1"), 'U', "{slug}");
            assert_eq!(status(slug, "4.1"), 'U', "{slug}");
        }
        // Packaging rows read from each published server's server.json.
        for slug in ["eigenstyle", "fathom", "lumen", "lupin", "periscope", "phaedrus", "phlow"] {
            assert_eq!(status(slug, "9.1"), 'P', "{slug}");
        }
    }
}

/// The MCP index — `/solutions/mcp`: the estate table itself, with
/// the reading rules stated once. Every per-server page links back
/// here, and the prerenderer scrapes this page's table for the
/// per-server routes.
#[component]
pub fn McpIndex() -> impl IntoView {
    view! {
        <Title text="MCP Servers - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — THE FLEET, BY PROTOCOL"</p>
            <h1>"MCP servers"</h1>
            <p class="page-lede">
                "Every Model Context Protocol server in the estate, with its standing. "
                "Presentation follows the estate's evidence rule — the Elson S. Floyd "
                "Standard for MCP Presentation and Records: every claim carries its "
                "basis, grades attach to releases rather than servers, and a server "
                "the evidence could not probe reads “Not probed”, never a quiet pass. "
                "In October 2026 the fleet migrated to rmcp and the modern protocol "
                "era only (2026-07-28); the registry bundles named “Published” are "
                "the published builds, and each server's page says where its code "
                "has moved since."
            </p>
            <McpServers />
            <p>
                "Each server's name links to its page: its standing, the exact client "
                "config where a verified launch command exists — and plain words "
                "where none does — its tool list, and its spec-compliance record."
            </p>
        </section>
    }
}
