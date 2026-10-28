//! Fathom's complete tool documentation, in three states — live,
//! building, planned — plus the deferred-by-design remainder.
//!
//! Sources: the tool registrations in fathom's own source (read
//! 2026-10-20, including the three il2cpp tools landed by Stage A)
//! and the Pax-gated parity matrix against the Ghidra-MCP v6.0.0
//! surface (279 distinct upstream tools: 31 covered, 177 to build in
//! stages G1–G4, 71 deferred). The tables are generated from those
//! sources (`fathom_doc_data`); upstream names in the building and
//! planned lists are the Ghidra-side tools fathom is implementing —
//! fathom's own names land with the code.

use crate::pages::fathom_doc_data::{DEFER_FAMILIES, G1_TOOLS, G2_TOOLS, G3_TOOLS, G4_TOOLS, GHIDRA_HAVE, LIVE_TOOLS};
use leptos::prelude::*;

/// Group (tool, family) pairs by family, preserving the sorted order
/// of the generated tables.
fn families<'a>(tools: &'a [(&'a str, &'a str)]) -> Vec<(&'a str, Vec<&'a str>)> {
    let mut out: Vec<(&str, Vec<&str>)> = Vec::new();
    for (name, family) in tools {
        match out.iter_mut().find(|(f, _)| f == family) {
            Some((_, names)) => names.push(name),
            None => out.push((family, vec![name])),
        }
    }
    out
}

/// One family's tool names as chips.
fn family_block(family: &'static str, names: Vec<&'static str>) -> impl IntoView {
    view! {
        <h3>{family}</h3>
        <ul class="tool-chips">
            {names
                .into_iter()
                .map(|n| {
                    view! {
                        <li>
                            <code>{n}</code>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}

/// A planned stage: its families behind a disclosure, so the page
/// stays readable while the full list is on the record.
fn planned_stage(stage: &'static str, count: usize, tools: &'static [(&'static str, &'static str)]) -> impl IntoView {
    view! {
        <details class="mcp-stage">
            <summary>{format!("Stage {stage} — {count} tools, by family")}</summary>
            {families(tools)
                .into_iter()
                .map(|(f, names)| family_block(f, names))
                .collect_view()}
        </details>
    }
}

/// The fathom documentation block on fathom's MCP page.
#[component]
pub fn FathomDoc() -> impl IntoView {
    view! {
        <h2>"Tools — live"</h2>
        <p>
            "Twenty-eight tools are registered in fathom's source and answer today: the \
             twenty-five of the core census plus the three IL2CPP tools landed by Stage A \
             (the chart-lift work of 2026-10-20). Descriptions are the tools' own registered \
             descriptors."
        </p>
        <figure class="bench">
            <div class="bench-scroll">
                <table class="bench-table">
                    <thead>
                        <tr>
                            <th>"Tool"</th>
                            <th>"Class"</th>
                            <th>"Provider"</th>
                            <th>"What it does"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {LIVE_TOOLS
                            .iter()
                            .map(|t| {
                                view! {
                                    <tr>
                                        <td>
                                            <code>{t.name}</code>
                                        </td>
                                        <td>{t.class}</td>
                                        <td>{t.provider}</td>
                                        <td>{t.desc}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </figure>
        <details class="mcp-stage">
            <summary>"Ghidra-MCP surface already covered — 31 upstream tools"</summary>
            <p>
                "The parity census maps thirty-one tools of the upstream Ghidra-MCP surface \
                 onto fathom tools that already exist. “Partial” is the matrix's own word: \
                 the fathom tool covers the call, with the noted field-level gaps."
            </p>
            <figure class="bench">
                <div class="bench-scroll">
                    <table class="bench-table">
                        <thead>
                            <tr>
                                <th>"Upstream tool"</th>
                                <th>"Covered by"</th>
                                <th>"Matrix note"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {GHIDRA_HAVE
                                .iter()
                                .map(|h| {
                                    view! {
                                        <tr>
                                            <td>
                                                <code>{h.upstream}</code>
                                            </td>
                                            <td>
                                                <code>{h.fathom}</code>
                                            </td>
                                            <td>{h.note}</td>
                                        </tr>
                                    }
                                })
                                .collect_view()}
                        </tbody>
                    </table>
                </div>
            </figure>
        </details>
        <h2>"Tools — building"</h2>
        <p>
            "The Ghidra read core (stage G1, 78 tools) is in build now: fathom grows its own \
             implementations of the upstream Ghidra-MCP read surface, registered through the \
             same registry and descriptor discipline as the live tools, over the headless \
             batch-export model — no second engine. The names below are the upstream tools \
             of the parity census, grouped by family; three of them (add_memory_reference, \
             remove_reference, batch_apply_documentation) are mutation-flavoured and are \
             classed honestly as such in the build, never dressed as reads."
        </p>
        {families(G1_TOOLS)
            .into_iter()
            .map(|(f, names)| family_block(f, names))
            .collect_view()}
        <h2>"Tools — planned"</h2>
        <p>
            "After G1: the write-and-refine stages of the parity program (G2–G4, 99 tools), \
             then the other two engines of the RE toolchain. The full planned lists are on \
             the record, by family:"
        </p>
        {planned_stage("G2", G2_TOOLS.len(), G2_TOOLS)}
        {planned_stage("G3", G3_TOOLS.len(), G3_TOOLS)}
        {planned_stage("G4", G4_TOOLS.len(), G4_TOOLS)}
        <h3>"IDA — a fathom crate against a bridge"</h3>
        <p>
            "IDA support is planned as a fathom crate speaking to a bridge over IDA's own \
             automation, built and tested first against a mock bridge. Live validation waits \
             on a current IDA (9.4+) installation on the analysis host — the licensed, \
             GUI-bound tool itself is the operator's install, and fathom does not inject \
             into closed applications."
        </p>
        <h3>"Hopper — a rebuild over the documented API"</h3>
        <p>
            "Hopper support is planned as a from-scratch rebuild over Hopper's documented \
             Python API, with a Rust core. The one existing Hopper MCP mechanism was studied \
             and rejected in full — it works by injecting code into the application, against \
             macOS platform protections; fathom does not do that. First stage is a \
             feasibility recon (H0) against the installed Hopper v4 (package 5.19.4)."
        </p>
        <h2>"Deferred by design — 71 upstream tools"</h2>
        <p>
            "The rest of the census is not planned at all, on stated grounds: stateful \
             project/session lifecycle (22) — fathom keeps its own per-subject SHA-256 \
             project cache over analyzeHeadless; TraceRmi live debugging (18) — outside \
             fathom's headless posture; Ghidra Server collaboration (17) — there is no \
             estate Ghidra Server; bridge-instance tooling for the upstream Python bridge \
             (7) — fathom is its own MCP server; and GUI-only tools (7) — fathom is \
             headless."
        </p>
        <figure class="bench">
            <div class="bench-scroll">
                <table class="bench-table">
                    <thead>
                        <tr>
                            <th>"Deferred family"</th>
                            <th>"Tools"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {DEFER_FAMILIES
                            .iter()
                            .map(|(family, count)| {
                                view! {
                                    <tr>
                                        <td>{*family}</td>
                                        <td>{*count}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </figure>
        <p>
            "The deep history behind these three engines — and why the Ghidra line runs \
             through the NSA — is its own page: "
            <a href="/solutions/mcp/fathom/re-history">"The RE toolchain: Ghidra, IDA, Hopper"</a>
            "."
        </p>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_doc_tables_match_their_sources() {
        assert_eq!(LIVE_TOOLS.len(), 28);
        assert_eq!(GHIDRA_HAVE.len(), 31);
        assert_eq!(G1_TOOLS.len(), 78);
        assert_eq!(G2_TOOLS.len(), 82);
        assert_eq!(G3_TOOLS.len(), 15);
        assert_eq!(G4_TOOLS.len(), 2);
        assert_eq!(DEFER_FAMILIES.iter().map(|(_, c)| c).sum::<usize>(), 71);
    }
}
