//! Batch 3 concept diagrams for solution pages. Owned ids are listed
//!
//! in `IDS`; the registry in `diagrams.rs` delegates here. Follow
//! the authoring contract in that module's header: `gfx-*` classes
//! only, text at 10.5px or larger at viewBox scale, motion only via
//! the stylesheet's reduced-motion-gated classes, and every figure
//! must read correctly as a static diagram.

use leptos::prelude::*;

/// The diagram ids this module owns.
pub const IDS: &[&str] =
    &["blaze-editor", "blaze-grammar", "diver-arch", "qudaz-stack", "rose-lifecycle", "rose-nvim-loop"];

/// The bare `<svg>` for one of this module's ids, or `None`.
pub fn diagram(id: &'static str) -> Option<AnyView> {
    match id {
        "blaze-editor" => Some(blaze_editor()),
        "blaze-grammar" => Some(blaze_grammar()),
        "diver-arch" => Some(diver_arch()),
        "qudaz-stack" => Some(qudaz_stack()),
        "rose-lifecycle" => Some(rose_lifecycle()),
        "rose-nvim-loop" => Some(rose_nvim_loop()),
        _ => None,
    }
}

/// Diver: the architecture map — `init.lua` loads the config
///
/// aggregator, which bootstraps per-language LSP servers on first
/// `FileType`; every language gets the same four tool lanes (LSP,
/// formatters, linters, DAP), and the research/journal modules, the
/// in-editor rose module, and the MCP bridge to phlow sit alongside.
///
/// Subsystems, lanes, and wiring are Diver's own, per its `init.lua`
/// and `lua/config/init.lua`.
fn diver_arch() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 428"
            role="img"
            aria-label="Diagram: Diver architecture — init.lua loads the config aggregator with core, cicd, cloud, edu, lang, nav and ui enabled; the aggregator bootstraps per-language LSP servers from lsp files on first FileType and fans out to four tool lanes (LSP, formatters, linters, DAP); below sit the research and journal modules, the in-editor rose AI module, and the MCP stdio bridge to phlow."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "INIT.LUA → CORE CONFIG → PER-LANGUAGE TOOL LANES"
            </text>
            <rect class="gfx-panel" x="8" y="44" width="196" height="160" rx="6" />
            <text class="gfx-head" x="106" y="68" text-anchor="middle">
                "NEOVIM START"
            </text>
            <text class="gfx-text" x="24" y="94">
                "init.lua"
            </text>
            <text class="gfx-note" x="24" y="116">
                "Space leader · Lua loader"
            </text>
            <text class="gfx-text" x="24" y="142">
                "config.init loads:"
            </text>
            <text class="gfx-note" x="24" y="164">
                "core · cicd · cloud · edu"
            </text>
            <text class="gfx-note" x="24" y="182">
                "lang · nav · ui"
            </text>
            <line class="gfx-arrow" x1="204" y1="124" x2="228" y2="124" />
            <circle class="gfx-pulse" cx="208" cy="124" r="3.5" />
            <rect class="gfx-panel" x="228" y="44" width="220" height="160" rx="6" />
            <text class="gfx-head" x="338" y="68" text-anchor="middle">
                "CORE CONFIG"
            </text>
            <text class="gfx-text" x="244" y="94">
                "LSP bootstrap on FileType"
            </text>
            <text class="gfx-text" x="244" y="118">
                "one lsp/*_ls.lua per server"
            </text>
            <text class="gfx-text" x="244" y="142">
                "UI · security · dev · utils"
            </text>
            <text class="gfx-text" x="244" y="166">
                "mappings loader, 11 modules"
            </text>
            <text class="gfx-note" x="244" y="188">
                "linters deferred to BufReadPre"
            </text>
            <line class="gfx-arrow" x1="448" y1="61" x2="472" y2="61" />
            <line class="gfx-arrow" x1="448" y1="103" x2="472" y2="103" />
            <line class="gfx-arrow" x1="448" y1="145" x2="472" y2="145" />
            <line class="gfx-arrow" x1="448" y1="187" x2="472" y2="187" />
            <circle class="gfx-pulse" cx="452" cy="61" r="3.5" />
            <rect class="gfx-chip" x="472" y="44" width="280" height="34" rx="6" />
            <text class="gfx-text" x="488" y="66">
                "LSP — servers · code actions"
            </text>
            <rect class="gfx-chip" x="472" y="86" width="280" height="34" rx="6" />
            <text class="gfx-text" x="488" y="108">
                "FORMATTERS — 138 adapters"
            </text>
            <rect class="gfx-chip" x="472" y="128" width="280" height="34" rx="6" />
            <text class="gfx-text" x="488" y="150">
                "LINTERS — deferred to first read"
            </text>
            <rect class="gfx-chip" x="472" y="170" width="280" height="34" rx="6" />
            <text class="gfx-text" x="488" y="192">
                "DAP — debug adapters per language"
            </text>
            <line class="gfx-arrow" x1="106" y1="204" x2="106" y2="228" />
            <line class="gfx-arrow" x1="338" y1="204" x2="338" y2="228" />
            <line class="gfx-arrow" x1="612" y1="204" x2="612" y2="228" />
            <circle class="gfx-pulse-y" cx="338" cy="208" r="3.5" />
            <rect class="gfx-panel" x="8" y="228" width="240" height="120" rx="6" />
            <text class="gfx-head" x="128" y="252" text-anchor="middle">
                "RESEARCH + JOURNAL"
            </text>
            <text class="gfx-text" x="128" y="278" text-anchor="middle">
                "journal profiles · ORCID"
            </text>
            <text class="gfx-text" x="128" y="300" text-anchor="middle">
                "Zotero · review templates"
            </text>
            <text class="gfx-note" x="128" y="324" text-anchor="middle">
                "the research registry in-tree"
            </text>
            <rect class="gfx-hot" x="260" y="228" width="240" height="120" rx="6" />
            <text class="gfx-head" x="380" y="252" text-anchor="middle">
                "AI — THE ROSE MODULE"
            </text>
            <text class="gfx-text" x="380" y="278" text-anchor="middle">
                "local chat inside Neovim"
            </text>
            <text class="gfx-text" x="380" y="300" text-anchor="middle">
                "planner → coder → reviewer"
            </text>
            <text class="gfx-note" x="380" y="324" text-anchor="middle">
                "backend: the rose server"
            </text>
            <rect class="gfx-panel" x="512" y="228" width="240" height="120" rx="6" />
            <text class="gfx-head" x="632" y="252" text-anchor="middle">
                "MCP BRIDGE"
            </text>
            <text class="gfx-text" x="632" y="278" text-anchor="middle">
                "MCP stdio client"
            </text>
            <text class="gfx-text" x="632" y="300" text-anchor="middle">
                "to the phlow runtime"
            </text>
            <text class="gfx-note" x="632" y="324" text-anchor="middle">
                "private socket: editor tools"
            </text>
            <rect class="gfx-badge" x="160" y="364" width="440" height="30" rx="6" />
            <text class="gfx-badge-text" x="380" y="384" text-anchor="middle">
                "ONE CONFIG — EVERY LANGUAGE, THE SAME FOUR LANES"
            </text>
            <text class="gfx-note" x="380" y="416" text-anchor="middle">
                "servers enable on first FileType; the AI stack is set up last in init.lua"
            </text>
        </svg>
    }
    .into_any()
}

/// Rose.nvim: the bounded in-editor agent loop — isolated planner,
///
/// coder, and reviewer contexts, a host-owned validation gate, and
/// at most one repair cycle by default — with the MCP stdio link to
/// phlow and the private editor socket beside it. Bounds are the
///
/// plugin's documented defaults.
fn rose_nvim_loop() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 440"
            role="img"
            aria-label="Diagram: the bounded Rose agent loop inside Neovim — planner, then coder (the only stage with file_write), then a host-owned validation gate of named checks and lint evidence, then an isolated reviewer that must approve; failure routes to at most one bounded repair cycle before the loop stops. Below: the MCP stdio link to phlow and the private editor socket."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "THE BOUNDED LOOP — :RoseAgent INSIDE NEOVIM"
            </text>
            <rect class="gfx-panel" x="8" y="44" width="172" height="84" rx="6" />
            <text class="gfx-head" x="94" y="68" text-anchor="middle">
                "1 · PLANNER"
            </text>
            <text class="gfx-text" x="94" y="92" text-anchor="middle">
                "isolated context"
            </text>
            <text class="gfx-note" x="94" y="112" text-anchor="middle">
                "at most 2 rounds"
            </text>
            <line class="gfx-arrow" x1="180" y1="86" x2="196" y2="86" />
            <circle class="gfx-pulse" cx="184" cy="86" r="3.5" />
            <rect class="gfx-hot" x="196" y="44" width="172" height="84" rx="6" />
            <text class="gfx-head" x="282" y="68" text-anchor="middle">
                "2 · CODER"
            </text>
            <text class="gfx-text" x="282" y="92" text-anchor="middle">
                "only stage with file_write"
            </text>
            <text class="gfx-note" x="282" y="112" text-anchor="middle">
                "6 rounds · 8 tool calls each"
            </text>
            <line class="gfx-arrow" x1="368" y1="86" x2="384" y2="86" />
            <circle
                class="gfx-pulse"
                cx="372"
                cy="86"
                r="3.5"
                style="animation-delay:0.8s"
            />
            <rect class="gfx-panel" x="384" y="44" width="172" height="84" rx="6" />
            <text class="gfx-head" x="470" y="66" text-anchor="middle">
                "3 · VALIDATION"
            </text>
            <text class="gfx-text" x="470" y="90" text-anchor="middle">
                "host-owned gate"
            </text>
            <text class="gfx-note" x="470" y="110" text-anchor="middle">
                "checks ok + lint evidence"
            </text>
            <line class="gfx-arrow" x1="556" y1="86" x2="572" y2="86" />
            <rect class="gfx-panel" x="572" y="44" width="172" height="84" rx="6" />
            <text class="gfx-head" x="658" y="68" text-anchor="middle">
                "4 · REVIEWER"
            </text>
            <text class="gfx-text" x="658" y="92" text-anchor="middle">
                "isolated context"
            </text>
            <text class="gfx-note" x="658" y="112" text-anchor="middle">
                "must return approved: true"
            </text>
            <line class="gfx-arrow" x1="380" y1="128" x2="380" y2="152" />
            <circle class="gfx-pulse-y" cx="380" cy="132" r="3.5" />
            <rect class="gfx-badge" x="230" y="152" width="300" height="36" rx="6" />
            <text class="gfx-badge-text" x="380" y="175" text-anchor="middle">
                "APPROVED + EVIDENCE COMPLETE?"
            </text>
            <line class="gfx-arrow" x1="300" y1="188" x2="208" y2="212" />
            <line class="gfx-arrow" x1="460" y1="188" x2="552" y2="212" />
            <circle class="gfx-pulse-y" cx="520" cy="196" r="3.5" />
            <text class="gfx-note" x="216" y="206">
                "yes"
            </text>
            <text class="gfx-note" x="520" y="206">
                "no"
            </text>
            <rect class="gfx-panel" x="68" y="212" width="280" height="64" rx="6" />
            <text class="gfx-head" x="208" y="236" text-anchor="middle">
                "STOP — DONE"
            </text>
            <text class="gfx-note" x="208" y="258" text-anchor="middle">
                "evidence verified, review approved"
            </text>
            <rect class="gfx-panel" x="412" y="212" width="280" height="64" rx="6" />
            <text class="gfx-head" x="552" y="236" text-anchor="middle">
                "REVISE — BOUNDED REPAIR"
            </text>
            <text class="gfx-note" x="552" y="258" text-anchor="middle">
                "1 repair round by default, then stop"
            </text>
            <line class="gfx-arrow gfx-dash" x1="380" y1="276" x2="380" y2="300" />
            <rect class="gfx-panel" x="8" y="300" width="368" height="96" rx="6" />
            <text class="gfx-head" x="192" y="324" text-anchor="middle">
                "MCP STDIO → PHLOW (:RoseFlow)"
            </text>
            <text class="gfx-text" x="192" y="348" text-anchor="middle">
                "newline JSON-RPC over stdio"
            </text>
            <text class="gfx-note" x="192" y="370" text-anchor="middle">
                "flow serve, called as flow_run"
            </text>
            <text class="gfx-note" x="192" y="388" text-anchor="middle">
                "fails closed if the bridge is unavailable"
            </text>
            <rect class="gfx-panel" x="384" y="300" width="368" height="96" rx="6" />
            <text class="gfx-head" x="568" y="324" text-anchor="middle">
                "PRIVATE EDITOR SOCKET"
            </text>
            <text class="gfx-text" x="568" y="348" text-anchor="middle">
                "phlow calls back into Neovim"
            </text>
            <text class="gfx-note" x="568" y="370" text-anchor="middle">
                "editor tools over the socket"
            </text>
            <text class="gfx-note" x="568" y="388" text-anchor="middle">
                "passed as the --nvim socket argument"
            </text>
            <text class="gfx-note" x="380" y="420" text-anchor="middle">
                "bounds are the documented defaults: 6 coder rounds, 8 tool calls per response, 1 repair round"
            </text>
        </svg>
    }
    .into_any()
}

/// Rose: one request's lifecycle through the server — HTTP API,
/// scheduler, runner lanes — over the GGUF model store whose path
/// follows the XDG convention and the `ROSE_MODELS` override.
///
/// Defaults are the server's own `envconfig` values.
fn rose_lifecycle() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 408"
            role="img"
            aria-label="Diagram: a request through the rose server — the HTTP API on loopback hands it to the scheduler, which queues up to 512 requests and loads models from the GGUF store at the XDG data path (ROSE_MODELS overrides it), and runner lanes serve the planner, coder, and reviewer specialists; the CLI drives the same server."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "ONE REQUEST THROUGH THE ROSE SERVER"
            </text>
            <rect class="gfx-panel" x="8" y="44" width="216" height="92" rx="6" />
            <text class="gfx-head" x="116" y="68" text-anchor="middle">
                "HTTP API"
            </text>
            <text class="gfx-text" x="116" y="92" text-anchor="middle">
                "POST /api/chat"
            </text>
            <text class="gfx-note" x="116" y="114" text-anchor="middle">
                "rose serve · 127.0.0.1:11434"
            </text>
            <line class="gfx-arrow" x1="224" y1="90" x2="248" y2="90" />
            <circle class="gfx-pulse" cx="228" cy="90" r="3.5" />
            <rect class="gfx-hot" x="248" y="44" width="216" height="92" rx="6" />
            <text class="gfx-head" x="356" y="68" text-anchor="middle">
                "SCHEDULER"
            </text>
            <text class="gfx-text" x="356" y="92" text-anchor="middle">
                "queue — max 512 waiting"
            </text>
            <text class="gfx-note" x="356" y="114" text-anchor="middle">
                "keep-alive 5m · load timeout 5m"
            </text>
            <line class="gfx-arrow" x1="464" y1="90" x2="488" y2="90" />
            <circle
                class="gfx-pulse"
                cx="468"
                cy="90"
                r="3.5"
                style="animation-delay:0.8s"
            />
            <rect class="gfx-panel" x="488" y="44" width="264" height="92" rx="6" />
            <text class="gfx-head" x="620" y="68" text-anchor="middle">
                "RUNNER LANES"
            </text>
            <text class="gfx-text" x="620" y="92" text-anchor="middle">
                "planner · coder · reviewer"
            </text>
            <text class="gfx-note" x="620" y="114" text-anchor="middle">
                "the specialists phlow calls"
            </text>
            <line class="gfx-arrow" x1="380" y1="136" x2="380" y2="160" />
            <circle class="gfx-pulse-y" cx="380" cy="140" r="3.5" />
            <text class="gfx-note" x="396" y="152">
                "loads weights from"
            </text>
            <rect class="gfx-panel" x="8" y="160" width="744" height="104" rx="6" />
            <text class="gfx-head" x="24" y="186">
                "MODEL STORE — GGUF FILES ON DISK"
            </text>
            <text class="gfx-text" x="24" y="212">
                "$XDG_DATA_HOME/rose/models"
            </text>
            <text class="gfx-note" x="330" y="212">
                "fallback: ~/.local/share/rose/models"
            </text>
            <text class="gfx-text" x="24" y="238">
                "ROSE_MODELS overrides the store path — it always wins"
            </text>
            <text class="gfx-note" x="560" y="238">
                "names resolve via harbor.qompass.ai"
            </text>
            <rect class="gfx-panel" x="8" y="288" width="744" height="64" rx="6" />
            <text class="gfx-head" x="380" y="312" text-anchor="middle">
                "THE SAME SERVER FROM THE CLI"
            </text>
            <text class="gfx-text" x="380" y="336" text-anchor="middle">
                "rose pull · rose run · rose ls · rose ps · rose stop · rose rm · rose create"
            </text>
            <rect class="gfx-badge" x="180" y="364" width="400" height="28" rx="6" />
            <text class="gfx-badge-text" x="380" y="383" text-anchor="middle">
                "ONLY ROSE_* VARIABLES CONFIGURE ROSE"
            </text>
        </svg>
    }
    .into_any()
}

/// Blaze-ts.nvim: the grammar pipeline — `grammar.js` generates the
///
/// C parser and external scanner, the parser builds the concrete
/// syntax tree, and queries turn the tree into highlights, indents,
/// and folds. The corpus suite gates the grammar; the parser ships
///
/// bundled or is built by nvim-treesitter.
fn blaze_grammar() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 410"
            role="img"
            aria-label="Diagram: the Blaze-ts grammar pipeline — grammar.js, a Python-superset base extended with Mojo constructs, generates the C parser and external scanner; the parser builds a concrete syntax tree; queries over the tree provide highlights, indents, and folds. A 24-case corpus suite gates the grammar, and the parser ships prebuilt or is built by nvim-treesitter."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "FROM GRAMMAR.JS TO A LIVE SYNTAX TREE"
            </text>
            <rect class="gfx-panel" x="8" y="44" width="172" height="96" rx="6" />
            <text class="gfx-head" x="94" y="68" text-anchor="middle">
                "GRAMMAR.JS"
            </text>
            <text class="gfx-text" x="94" y="90" text-anchor="middle">
                "Python-superset base"
            </text>
            <text class="gfx-note" x="94" y="110" text-anchor="middle">
                "+ fn · struct · trait"
            </text>
            <text class="gfx-note" x="94" y="126" text-anchor="middle">
                "var · let · alias · raises"
            </text>
            <line class="gfx-arrow" x1="180" y1="92" x2="196" y2="92" />
            <circle class="gfx-pulse" cx="184" cy="92" r="3.5" />
            <rect class="gfx-panel" x="196" y="44" width="172" height="96" rx="6" />
            <text class="gfx-head" x="282" y="68" text-anchor="middle">
                "GENERATE"
            </text>
            <text class="gfx-text" x="282" y="90" text-anchor="middle">
                "parser.c · scanner.c"
            </text>
            <text class="gfx-note" x="282" y="110" text-anchor="middle">
                "external scanner:"
            </text>
            <text class="gfx-note" x="282" y="126" text-anchor="middle">
                "indent 128 · delim 32"
            </text>
            <line class="gfx-arrow" x1="368" y1="92" x2="384" y2="92" />
            <circle
                class="gfx-pulse"
                cx="372"
                cy="92"
                r="3.5"
                style="animation-delay:0.8s"
            />
            <rect class="gfx-panel" x="384" y="44" width="172" height="96" rx="6" />
            <text class="gfx-head" x="470" y="68" text-anchor="middle">
                "SYNTAX TREE"
            </text>
            <text class="gfx-text" x="470" y="90" text-anchor="middle">
                "concrete tree of"
            </text>
            <text class="gfx-text" x="470" y="108" text-anchor="middle">
                "the parsed buffer"
            </text>
            <text class="gfx-note" x="470" y="126" text-anchor="middle">
                "incremental reparse on edit"
            </text>
            <line class="gfx-arrow" x1="556" y1="92" x2="572" y2="92" />
            <rect class="gfx-hot" x="572" y="44" width="172" height="96" rx="6" />
            <text class="gfx-head" x="658" y="68" text-anchor="middle">
                "QUERIES"
            </text>
            <text class="gfx-text" x="658" y="90" text-anchor="middle">
                "highlights · indents"
            </text>
            <text class="gfx-text" x="658" y="108" text-anchor="middle">
                "folds · locals"
            </text>
            <text class="gfx-note" x="658" y="126" text-anchor="middle">
                "resolved from runtime/"
            </text>
            <line class="gfx-arrow" x1="380" y1="140" x2="380" y2="164" />
            <circle class="gfx-pulse-y" cx="380" cy="144" r="3.5" />
            <rect class="gfx-panel" x="158" y="164" width="444" height="72" rx="6" />
            <text class="gfx-head" x="380" y="188" text-anchor="middle">
                "GATED BY THE CORPUS SUITE"
            </text>
            <text class="gfx-text" x="380" y="212" text-anchor="middle">
                "test/corpus — 24 cases, malformed input included"
            </text>
            <text class="gfx-note" x="380" y="230" text-anchor="middle">
                "regenerate and rebuild with the tree-sitter CLI"
            </text>
            <line class="gfx-arrow gfx-dash" x1="192" y1="236" x2="192" y2="260" />
            <line class="gfx-arrow gfx-dash" x1="568" y1="236" x2="568" y2="260" />
            <circle class="gfx-pulse-y" cx="192" cy="240" r="3.5" />
            <rect class="gfx-panel" x="8" y="260" width="368" height="88" rx="6" />
            <text class="gfx-head" x="192" y="284" text-anchor="middle">
                "BUNDLED WITH THE PLUGIN"
            </text>
            <text class="gfx-text" x="192" y="308" text-anchor="middle">
                "parser/mojo.so — prebuilt"
            </text>
            <text class="gfx-text" x="192" y="328" text-anchor="middle">
                "parser/mojo.wasm — WebAssembly"
            </text>
            <rect class="gfx-panel" x="384" y="260" width="368" height="88" rx="6" />
            <text class="gfx-head" x="568" y="284" text-anchor="middle">
                "OR BUILT BY NVIM-TREESITTER"
            </text>
            <text class="gfx-text" x="568" y="308" text-anchor="middle">
                ":TSInstall mojo"
            </text>
            <text class="gfx-note" x="568" y="328" text-anchor="middle">
                "nvim-treesitter not required for the bundled path"
            </text>
            <text class="gfx-note" x="380" y="374" text-anchor="middle">
                "adapted from tree-sitter-python's architecture, extended with Mojo constructs"
            </text>
            <text class="gfx-note" x="380" y="394" text-anchor="middle">
                "argument conventions: inout · owned · borrowed · out · read · mut · ref"
            </text>
        </svg>
    }
    .into_any()
}

/// Blaze.nvim: filetype detection for one Mojo buffer fans out to
///
/// five subsystems — Tree-sitter, LSP, formatter, linter, DAP —
/// wired in the plugin's setup order, with `:Fever` as the health
/// check over the whole stack. Defaults are the plugin's own.
fn blaze_editor() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 414"
            role="img"
            aria-label="Diagram: one Mojo buffer in Blaze.nvim — filetype detection for .mojo and the fire-emoji extension fans out to Tree-sitter highlighting via blaze-ts.nvim with a bundled syntax fallback, the Mojo LSP server, mojo format on save, linting, and DAP debugging; setup enables LSP, Pixi, and Magic in order, and :Fever health-checks the stack."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "ONE .MOJO BUFFER — FIVE SUBSYSTEMS WIRED"
            </text>
            <rect class="gfx-hot" x="230" y="40" width="300" height="52" rx="6" />
            <text class="gfx-head" x="380" y="62" text-anchor="middle">
                "BUFFER: *.mojo / *.🔥"
            </text>
            <text class="gfx-note" x="380" y="82" text-anchor="middle">
                "FileType autocmd fires on open"
            </text>
            <line class="gfx-arrow" x1="380" y1="92" x2="380" y2="116" />
            <line class="gfx-arrow" x1="84" y1="116" x2="680" y2="116" />
            <line class="gfx-arrow" x1="84" y1="116" x2="84" y2="140" />
            <line class="gfx-arrow" x1="230" y1="116" x2="230" y2="140" />
            <line class="gfx-arrow" x1="380" y1="116" x2="380" y2="140" />
            <line class="gfx-arrow" x1="530" y1="116" x2="530" y2="140" />
            <line class="gfx-arrow" x1="680" y1="116" x2="680" y2="140" />
            <circle class="gfx-pulse-y" cx="380" cy="120" r="3.5" />
            <rect class="gfx-panel" x="16" y="140" width="136" height="110" rx="6" />
            <text class="gfx-head" x="84" y="164" text-anchor="middle">
                "TREE-SITTER"
            </text>
            <text class="gfx-text" x="84" y="188" text-anchor="middle">
                "blaze-ts.nvim"
            </text>
            <text class="gfx-text" x="84" y="206" text-anchor="middle">
                "highlighting"
            </text>
            <text class="gfx-note" x="84" y="228" text-anchor="middle">
                "fallback: bundled"
            </text>
            <text class="gfx-note" x="84" y="244" text-anchor="middle">
                "syntax file"
            </text>
            <rect class="gfx-panel" x="162" y="140" width="136" height="110" rx="6" />
            <text class="gfx-head" x="230" y="164" text-anchor="middle">
                "LSP"
            </text>
            <text class="gfx-text" x="230" y="188" text-anchor="middle">
                "Mojo LSP server"
            </text>
            <text class="gfx-text" x="230" y="206" text-anchor="middle">
                "diagnostics"
            </text>
            <text class="gfx-note" x="230" y="228" text-anchor="middle">
                "lsp.enabled"
            </text>
            <rect class="gfx-panel" x="312" y="140" width="136" height="110" rx="6" />
            <text class="gfx-head" x="380" y="164" text-anchor="middle">
                "FORMATTER"
            </text>
            <text class="gfx-text" x="380" y="188" text-anchor="middle">
                "mojo format"
            </text>
            <text class="gfx-text" x="380" y="206" text-anchor="middle">
                "on save"
            </text>
            <text class="gfx-note" x="380" y="228" text-anchor="middle">
                "line_length 88"
            </text>
            <text class="gfx-note" x="380" y="244" text-anchor="middle">
                "format_on_save"
            </text>
            <rect class="gfx-panel" x="462" y="140" width="136" height="110" rx="6" />
            <text class="gfx-head" x="530" y="164" text-anchor="middle">
                "LINTER"
            </text>
            <text class="gfx-text" x="530" y="188" text-anchor="middle">
                "nvim-lint"
            </text>
            <text class="gfx-text" x="530" y="206" text-anchor="middle">
                "null-ls"
            </text>
            <text class="gfx-note" x="530" y="228" text-anchor="middle">
                "enable_linting"
            </text>
            <rect class="gfx-panel" x="612" y="140" width="136" height="110" rx="6" />
            <text class="gfx-head" x="680" y="164" text-anchor="middle">
                "DAP"
            </text>
            <text class="gfx-text" x="680" y="188" text-anchor="middle">
                "nvim-dap"
            </text>
            <text class="gfx-text" x="680" y="206" text-anchor="middle">
                "debug adapter"
            </text>
            <text class="gfx-note" x="680" y="228" text-anchor="middle">
                "dap.enabled"
            </text>
            <rect class="gfx-panel" x="8" y="274" width="744" height="64" rx="6" />
            <text class="gfx-head" x="380" y="298" text-anchor="middle">
                "SETUP ORDER — THE BLAZE SETUP CALL"
            </text>
            <text class="gfx-text" x="380" y="322" text-anchor="middle">
                "LSP servers → Pixi → Magic → 🔥 filetype, then indentation per buffer"
            </text>
            <rect class="gfx-chip" x="230" y="362" width="300" height="36" rx="6" />
            <text class="gfx-text" x="380" y="385" text-anchor="middle">
                ":Fever — health check, whole stack"
            </text>
        </svg>
    }
    .into_any()
}

/// Qudaz: the binding stack — Zig code calls the library's public
///
/// API, the bindings sit over NVIDIA's driver API, NVRTC, and
/// cuRAND libraries, and `build.zig` finds and links CUDA at build
/// time. Layers and link list are the repo's own `build.zig` and
///
/// `src/lib.zig` exports.
fn qudaz_stack() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 452"
            role="img"
            aria-label="Diagram: the qudaz stack — Zig code calls the public API (Device, Compile, Rng, CudaSlice); the qudaz bindings sit over the CUDA driver API, NVRTC runtime compilation, and cuRAND; at build time build.zig locates the CUDA installation and links libc plus the cuda, nvrtc, curand, and cudart libraries."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "ZIG ON TOP — CUDA UNDERNEATH, LINKED AT BUILD TIME"
            </text>
            <rect class="gfx-panel" x="188" y="44" width="384" height="72" rx="6" />
            <text class="gfx-head" x="380" y="66" text-anchor="middle">
                "YOUR ZIG CODE"
            </text>
            <text class="gfx-text" x="380" y="88" text-anchor="middle">
                "Device.default() · alloc · htodCopy"
            </text>
            <text class="gfx-note" x="380" y="106" text-anchor="middle">
                "public API: Device · Compile · Rng · CudaSlice"
            </text>
            <line class="gfx-arrow" x1="380" y1="116" x2="380" y2="132" />
            <circle class="gfx-pulse-y" cx="380" cy="120" r="3.5" />
            <rect class="gfx-hot" x="188" y="132" width="384" height="72" rx="6" />
            <text class="gfx-head" x="380" y="154" text-anchor="middle">
                "QUDAZ — src/lib.zig"
            </text>
            <text class="gfx-text" x="380" y="176" text-anchor="middle">
                "high-level Zig bindings, CAPI underneath"
            </text>
            <text class="gfx-note" x="380" y="194" text-anchor="middle">
                "LaunchConfig · CudaError · Compile → PTX"
            </text>
            <line class="gfx-arrow" x1="380" y1="204" x2="380" y2="216" />
            <line class="gfx-arrow" x1="128" y1="216" x2="632" y2="216" />
            <line class="gfx-arrow" x1="128" y1="216" x2="128" y2="228" />
            <line class="gfx-arrow" x1="380" y1="216" x2="380" y2="228" />
            <line class="gfx-arrow" x1="632" y1="216" x2="632" y2="228" />
            <circle class="gfx-pulse-y" cx="380" cy="208" r="3.5" />
            <rect class="gfx-panel" x="8" y="228" width="240" height="96" rx="6" />
            <text class="gfx-head" x="128" y="252" text-anchor="middle">
                "CUDA DRIVER API"
            </text>
            <text class="gfx-text" x="128" y="276" text-anchor="middle">
                "libcuda"
            </text>
            <text class="gfx-note" x="128" y="296" text-anchor="middle">
                "devices · memory · modules"
            </text>
            <text class="gfx-note" x="128" y="312" text-anchor="middle">
                "Device calls land here"
            </text>
            <rect class="gfx-panel" x="260" y="228" width="240" height="96" rx="6" />
            <text class="gfx-head" x="380" y="252" text-anchor="middle">
                "NVRTC"
            </text>
            <text class="gfx-text" x="380" y="276" text-anchor="middle">
                "libnvrtc"
            </text>
            <text class="gfx-note" x="380" y="296" text-anchor="middle">
                "CUDA source → PTX at runtime"
            </text>
            <text class="gfx-note" x="380" y="312" text-anchor="middle">
                "Compile.cudaText / cudaFile"
            </text>
            <rect class="gfx-panel" x="512" y="228" width="240" height="96" rx="6" />
            <text class="gfx-head" x="632" y="252" text-anchor="middle">
                "CURAND"
            </text>
            <text class="gfx-text" x="632" y="276" text-anchor="middle">
                "libcurand"
            </text>
            <text class="gfx-note" x="632" y="296" text-anchor="middle">
                "GPU random numbers"
            </text>
            <text class="gfx-note" x="632" y="312" text-anchor="middle">
                "the Rng export"
            </text>
            <rect class="gfx-panel" x="8" y="348" width="744" height="64" rx="6" />
            <text class="gfx-head" x="380" y="370" text-anchor="middle">
                "BUILD TIME — build.zig FINDS AND LINKS CUDA"
            </text>
            <text class="gfx-text" x="380" y="392" text-anchor="middle">
                "-DCUDA_PATH first, else probe /usr · /usr/local/cuda · /opt/cuda · /usr/lib/cuda"
            </text>
            <text class="gfx-note" x="380" y="408" text-anchor="middle">
                "links libc + cuda · nvrtc · curand · cudart"
            </text>
            <text class="gfx-note" x="380" y="436" text-anchor="middle">
                "minimum Zig 0.14.0 · package version 0.1.0 — the build.zig.zon contract"
            </text>
        </svg>
    }
    .into_any()
}
