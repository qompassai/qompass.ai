//! Batch 5 concept diagrams for solution pages. Owned ids are listed
//!
//! in `IDS`; the registry in `diagrams.rs` delegates here. Follow
//! the authoring contract in that module's header: `gfx-*` classes
//! only, text at 10.5px or larger at viewBox scale, motion only via
//! the stylesheet's reduced-motion-gated classes, and every figure
//! must read correctly as a static diagram.

use leptos::prelude::*;

/// The diagram ids this module owns.
pub const IDS: &[&str] = &[
    "pathfinders-pathway",
    "phlow-coordination",
    "redteam-collection",
    "telco-answer-keys",
    "tuxtype-loop",
    "unreal-pipeline",
];

/// The bare `<svg>` for one of this module's ids, or `None`.
pub fn diagram(id: &'static str) -> Option<AnyView> {
    match id {
        "pathfinders-pathway" => Some(pathfinders_pathway()),
        "phlow-coordination" => Some(phlow_coordination()),
        "redteam-collection" => Some(redteam_collection()),
        "telco-answer-keys" => Some(telco_answer_keys()),
        "tuxtype-loop" => Some(tuxtype_loop()),
        "unreal-pipeline" => Some(unreal_pipeline()),
        _ => None,
    }
}

/// The Telco Quizzes page's references: the cryptography the
/// answer-key envelopes use, and the code authorities the quiz sets
/// study. Returned as (title, url, note) triples for the dedicated
///
/// page component, which the coordinator wires. Every URL was
/// curl-verified live (final 200) at authoring time, 2026-10-18.
pub fn telco_references() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Web Crypto API — W3C Recommendation",
            "https://www.w3.org/TR/WebCryptoAPI/",
            "The browser API (crypto.subtle) the page uses to derive the key with PBKDF2 and decrypt each answer-key envelope locally with AES-GCM.",
        ),
        (
            "NIST SP 800-38D — Galois/Counter Mode (GCM) and GMAC",
            "https://csrc.nist.gov/pubs/sp/800/38/d/final",
            "The NIST specification of AES-GCM, the authenticated-encryption mode sealing every answer-key envelope.",
        ),
        (
            "NFPA 70 — National Electrical Code (NEC), NFPA catalog entry",
            "https://www.nfpa.org/codes-and-standards/nfpa-70-standard-development/70",
            "NFPA's own page for NFPA 70, the National Electrical Code the NEC quiz sets study; the code text itself is sold by NFPA, so this is the code's catalog entry.",
        ),
        (
            "Washington RCW 19.28 — Electricians and Electrical Installations",
            "https://app.leg.wa.gov/rcw/default.aspx?cite=19.28",
            "The Washington statute the RCW 19.28 quiz set studies, from the Washington State Legislature's official code site.",
        ),
        (
            "Washington WAC 296-46B — Electrical Safety Standards, Administration, and Installation",
            "https://app.leg.wa.gov/wac/default.aspx?cite=296-46B",
            "The Washington administrative code chapter the WAC 296-46B quiz set studies, from the Washington State Legislature's official code site.",
        ),
    ]
}

/// `PathFinders`: the program pathway as the project's own README
///
/// structures it — recruit (no coding prerequisite), build (the
/// Lessons curriculum), hybrid deploy (cloud-native generative AI
/// services, cloud and on device, regulation-aligned from the
/// start), and leadership prep. A program, not software: there is
///
/// no external authority for the pathway, and the figure says so.
fn pathfinders_pathway() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 352"
            role="img"
            aria-label="Diagram: the PathFinders program pathway in four stages — recruit with no coding prerequisite (character and curiosity outweigh coding ability), build through the Lessons levels 1 to 9 and Youth tutorials, deploy cloud-native generative AI services hybrid in the cloud and on device with regulation alignment from the start, and prepare as future strategic leaders."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "RECRUIT → BUILD → HYBRID DEPLOY → LEAD"
            </text>
            <rect class="gfx-panel" x="8" y="48" width="180" height="168" rx="6" />
            <text class="gfx-head" x="98" y="74" text-anchor="middle">
                "1 · RECRUIT"
            </text>
            <text class="gfx-text" x="98" y="104" text-anchor="middle">
                "no coding"
            </text>
            <text class="gfx-text" x="98" y="126" text-anchor="middle">
                "prerequisite"
            </text>
            <text class="gfx-note" x="98" y="158" text-anchor="middle">
                "Character & Curiosity"
            </text>
            <text class="gfx-note" x="98" y="178" text-anchor="middle">
                "outweighs any"
            </text>
            <text class="gfx-note" x="98" y="198" text-anchor="middle">
                "coding ability"
            </text>
            <line class="gfx-arrow" x1="188" y1="132" x2="196" y2="132" />
            <circle class="gfx-pulse" cx="190" cy="132" r="3.5" />
            <rect class="gfx-panel" x="196" y="48" width="180" height="168" rx="6" />
            <text class="gfx-head" x="286" y="74" text-anchor="middle">
                "2 · BUILD"
            </text>
            <text class="gfx-text" x="286" y="104" text-anchor="middle">
                "Lessons/ Levels 1–9"
            </text>
            <text class="gfx-text" x="286" y="126" text-anchor="middle">
                "Youth/ tutorials"
            </text>
            <text class="gfx-note" x="286" y="158" text-anchor="middle">
                "Quick Start setup:"
            </text>
            <text class="gfx-note" x="286" y="178" text-anchor="middle">
                "Nix symlinks +"
            </text>
            <text class="gfx-note" x="286" y="198" text-anchor="middle">
                "dev shell"
            </text>
            <line class="gfx-arrow" x1="376" y1="132" x2="384" y2="132" />
            <circle
                class="gfx-pulse"
                cx="378"
                cy="132"
                r="3.5"
                style="animation-delay:0.6s"
            />
            <rect class="gfx-hot" x="384" y="48" width="180" height="168" rx="6" />
            <text class="gfx-head" x="474" y="74" text-anchor="middle">
                "3 · HYBRID DEPLOY"
            </text>
            <text class="gfx-text" x="474" y="104" text-anchor="middle">
                "cloud-native genAI"
            </text>
            <text class="gfx-text" x="474" y="126" text-anchor="middle">
                "services — cloud"
            </text>
            <text class="gfx-text" x="474" y="148" text-anchor="middle">
                "and on device"
            </text>
            <text class="gfx-note" x="474" y="178" text-anchor="middle">
                "on-device PQC (NIST-"
            </text>
            <text class="gfx-note" x="474" y="198" text-anchor="middle">
                "endorsed) + open models"
            </text>
            <line class="gfx-arrow" x1="564" y1="132" x2="572" y2="132" />
            <circle
                class="gfx-pulse"
                cx="566"
                cy="132"
                r="3.5"
                style="animation-delay:1.2s"
            />
            <rect class="gfx-panel" x="572" y="48" width="180" height="168" rx="6" />
            <text class="gfx-head" x="662" y="74" text-anchor="middle">
                "4 · LEADERSHIP PREP"
            </text>
            <text class="gfx-text" x="662" y="104" text-anchor="middle">
                "preparing as future"
            </text>
            <text class="gfx-text" x="662" y="126" text-anchor="middle">
                "strategic leaders"
            </text>
            <text class="gfx-note" x="662" y="158" text-anchor="middle">
                "regulation from the"
            </text>
            <text class="gfx-note" x="662" y="178" text-anchor="middle">
                "beginning of any"
            </text>
            <text class="gfx-note" x="662" y="198" text-anchor="middle">
                "product or service"
            </text>
            <rect class="gfx-badge" x="130" y="240" width="500" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="265" text-anchor="middle">
                "A PROGRAM, NOT SOFTWARE — THE PROJECT'S OWN DESIGN"
            </text>
            <text class="gfx-note" x="380" y="306" text-anchor="middle">
                "regulation cited in the README: OMB guidance on responsible AI acquisition (Oct 3, 2024)"
            </text>
            <text class="gfx-note" x="380" y="328" text-anchor="middle">
                "and the Executive Order on safe, secure and trustworthy AI (Oct 30, 2023)"
            </text>
        </svg>
    }
    .into_any()
}

/// Phlow: the multi-agent coordination decision on a task claim, as
/// the study page specifies the `flow_attention_decide` rule — the
/// project's own protocol design. A snapshot-hash mismatch rejects
///
/// the claim; an empty focus set proceeds in parallel; a shared top
/// target serializes with the lexicographically smaller agent id as
/// holder; anything else proceeds in parallel.
fn phlow_coordination() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 456"
            role="img"
            aria-label="Diagram: phlow's coordination decision on a task claim. If the agent's snapshot hash does not match the shared frame, the claim is rejected. If the focus set is empty, the agent proceeds in parallel. If another agent shares the same top target, the agents serialize and the holder is the lexicographically smaller agent id. Otherwise the agent proceeds in parallel."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "ONE CLAIM, ONE DECISION — EVERY AGENT COMPUTES THE SAME RULE"
            </text>
            <rect class="gfx-panel" x="205" y="36" width="350" height="40" rx="6" />
            <text class="gfx-head" x="380" y="61" text-anchor="middle">
                "AGENT CLAIMS A TASK"
            </text>
            <line class="gfx-arrow" x1="380" y1="76" x2="380" y2="100" />
            <circle class="gfx-pulse-y" cx="380" cy="80" r="3.5" />
            <rect class="gfx-panel" x="190" y="102" width="380" height="56" rx="6" />
            <text class="gfx-head" x="380" y="126" text-anchor="middle">
                "SNAPSHOT HASH MATCHES?"
            </text>
            <text class="gfx-note" x="380" y="146" text-anchor="middle">
                "the shared saliency frame, identified by its SHA-256"
            </text>
            <line class="gfx-arrow" x1="570" y1="130" x2="596" y2="130" />
            <text class="gfx-note" x="583" y="120" text-anchor="middle">
                "no"
            </text>
            <rect class="gfx-hot" x="596" y="102" width="156" height="56" rx="6" />
            <text class="gfx-head" x="674" y="126" text-anchor="middle">
                "REJECT"
            </text>
            <text class="gfx-note" x="674" y="146" text-anchor="middle">
                "stale frame"
            </text>
            <line class="gfx-arrow" x1="380" y1="158" x2="380" y2="182" />
            <circle
                class="gfx-pulse-y"
                cx="380"
                cy="162"
                r="3.5"
                style="animation-delay:0.5s"
            />
            <text class="gfx-note" x="394" y="174">
                "yes"
            </text>
            <rect class="gfx-panel" x="190" y="184" width="380" height="56" rx="6" />
            <text class="gfx-head" x="380" y="208" text-anchor="middle">
                "FOCUS SET EMPTY?"
            </text>
            <text class="gfx-note" x="380" y="228" text-anchor="middle">
                "no targets claimed yet in this frame"
            </text>
            <line class="gfx-arrow" x1="164" y1="212" x2="190" y2="212" />
            <text class="gfx-note" x="177" y="202" text-anchor="middle">
                "yes"
            </text>
            <rect class="gfx-chip" x="8" y="184" width="156" height="56" rx="6" />
            <text class="gfx-head" x="86" y="208" text-anchor="middle">
                "PARALLEL"
            </text>
            <text class="gfx-note" x="86" y="228" text-anchor="middle">
                "proceed at once"
            </text>
            <line class="gfx-arrow" x1="380" y1="240" x2="380" y2="264" />
            <circle
                class="gfx-pulse-y"
                cx="380"
                cy="244"
                r="3.5"
                style="animation-delay:1s"
            />
            <text class="gfx-note" x="394" y="256">
                "no"
            </text>
            <rect class="gfx-panel" x="190" y="266" width="380" height="56" rx="6" />
            <text class="gfx-head" x="380" y="290" text-anchor="middle">
                "SAME TOP TARGET?"
            </text>
            <text class="gfx-note" x="380" y="310" text-anchor="middle">
                "another agent's top focus element is the same target"
            </text>
            <line class="gfx-arrow" x1="570" y1="294" x2="596" y2="294" />
            <text class="gfx-note" x="583" y="284" text-anchor="middle">
                "no"
            </text>
            <rect class="gfx-chip" x="596" y="266" width="156" height="56" rx="6" />
            <text class="gfx-head" x="674" y="290" text-anchor="middle">
                "PARALLEL"
            </text>
            <text class="gfx-note" x="674" y="310" text-anchor="middle">
                "different targets"
            </text>
            <line class="gfx-arrow" x1="380" y1="322" x2="380" y2="346" />
            <circle
                class="gfx-pulse-y"
                cx="380"
                cy="326"
                r="3.5"
                style="animation-delay:1.5s"
            />
            <text class="gfx-note" x="394" y="338">
                "yes"
            </text>
            <rect class="gfx-hot" x="190" y="348" width="380" height="64" rx="6" />
            <text class="gfx-head" x="380" y="374" text-anchor="middle">
                "SERIALIZE"
            </text>
            <text class="gfx-note" x="380" y="396" text-anchor="middle">
                "holder = lexicographically smaller agent id"
            </text>
            <text class="gfx-note" x="380" y="440" text-anchor="middle">
                "the deferred run ends with the holder named · decided after planning, within one shared frame"
            </text>
        </svg>
    }
    .into_any()
}

/// Redteam: a curated collection, not one program — its taxonomy of
/// areas, and the structural validation gate its own test suite
/// (`tests/test_collection.py`) runs over every change.
fn redteam_collection() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 404"
            role="img"
            aria-label="Diagram: the Redteam collection taxonomy — Gov/NIST, OGRed, Reports, BlackArch, Kali, Flipper and RedAI areas — above its structural validation gate: pytest checks every PDF's structure, the Flipper tarball, the wifi-coconut build, the report's input graph, and the BlackArch shell scripts."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "A CURATED COLLECTION, GATED — NOT ONE PROGRAM"
            </text>
            <rect class="gfx-panel" x="8" y="48" width="744" height="150" rx="6" />
            <text class="gfx-head" x="380" y="74" text-anchor="middle">
                "COLLECTION TAXONOMY — BROWSE BY AREA"
            </text>
            <rect class="gfx-chip" x="16" y="88" width="104" height="34" rx="6" />
            <text class="gfx-text" x="68" y="110" text-anchor="middle">
                "Gov/NIST"
            </text>
            <rect class="gfx-chip" x="128" y="88" width="84" height="34" rx="6" />
            <text class="gfx-text" x="170" y="110" text-anchor="middle">
                "OGRed"
            </text>
            <rect class="gfx-chip" x="220" y="88" width="96" height="34" rx="6" />
            <text class="gfx-text" x="268" y="110" text-anchor="middle">
                "Reports"
            </text>
            <rect class="gfx-chip" x="324" y="88" width="106" height="34" rx="6" />
            <text class="gfx-text" x="377" y="110" text-anchor="middle">
                "BlackArch"
            </text>
            <rect class="gfx-chip" x="438" y="88" width="72" height="34" rx="6" />
            <text class="gfx-text" x="474" y="110" text-anchor="middle">
                "Kali"
            </text>
            <rect class="gfx-chip" x="518" y="88" width="88" height="34" rx="6" />
            <text class="gfx-text" x="562" y="110" text-anchor="middle">
                "Flipper"
            </text>
            <rect class="gfx-chip" x="614" y="88" width="122" height="34" rx="6" />
            <text class="gfx-text" x="675" y="110" text-anchor="middle">
                "RedAI"
            </text>
            <text class="gfx-note" x="24" y="148">
                "Gov/NIST: AI RMF Playbook · OGRed: handbook, MITRE ATT&CK, Hak5 tools, books"
            </text>
            <text class="gfx-note" x="24" y="172">
                "Reports: LaTeX pentest report · BlackArch: packaging · Kali: ARM builds + recipes"
            </text>
            <text class="gfx-note" x="748" y="148" text-anchor="end">
                "Flipper: firmware tarball"
            </text>
            <text class="gfx-note" x="748" y="172" text-anchor="end">
                "RedAI: AI red-teaming PDFs"
            </text>
            <line class="gfx-arrow" x1="380" y1="198" x2="380" y2="222" />
            <circle class="gfx-pulse-y" cx="380" cy="202" r="3.5" />
            <rect class="gfx-panel" x="8" y="224" width="744" height="110" rx="6" />
            <text class="gfx-head" x="380" y="250" text-anchor="middle">
                "STRUCTURAL VALIDATION GATE — python3 -m pytest tests/ -q"
            </text>
            <text class="gfx-text" x="24" y="282">
                "every PDF: %PDF header + %%EOF marker"
            </text>
            <text class="gfx-text" x="24" y="308">
                "Flipper tarball readable"
            </text>
            <text class="gfx-text" x="400" y="282">
                "wifi-coconut builds (CMake, scratch copy)"
            </text>
            <text class="gfx-text" x="400" y="308">
                "report \\input graph · BlackArch scripts: bash -n"
            </text>
            <rect class="gfx-badge" x="180" y="350" width="400" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="375" text-anchor="middle">
                "RUN THE GATE AFTER ANY CHANGE"
            </text>
        </svg>
    }
    .into_any()
}

/// Telco Quizzes: the answer-key protection flow, as the page's own
///
/// code states it — answer-key documents are encrypted per set at
/// build time by `scripts/generate-answer-keys.mjs` (read as
/// evidence for this figure, never executed by the author), the
/// static site ships ciphertext only, and the candidate's password
/// decrypts the key locally in the browser via `WebCrypto`.
fn telco_answer_keys() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 388"
            role="img"
            aria-label="Diagram: the Telco Quizzes answer-key protection flow — quiz sets are encrypted per set at build time (PBKDF2-SHA256 key derivation, AES-256-GCM envelope), the static site ships ciphertext only, the candidate enters the set password, WebCrypto decrypts the key in the browser, and the runner checks answers locally; the password is never stored or sent."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "ANSWER KEYS: CIPHERTEXT ONLY, DECRYPTED IN THE BROWSER"
            </text>
            <rect class="gfx-panel" x="8" y="48" width="236" height="118" rx="6" />
            <text class="gfx-head" x="126" y="74" text-anchor="middle">
                "1 · QUIZ SETS"
            </text>
            <text class="gfx-text" x="126" y="102" text-anchor="middle">
                "12 sets · 359 questions"
            </text>
            <text class="gfx-note" x="126" y="130" text-anchor="middle">
                "NEC · RCW 19.28"
            </text>
            <text class="gfx-note" x="126" y="150" text-anchor="middle">
                "WAC 296-46B · theory"
            </text>
            <line class="gfx-arrow" x1="244" y1="107" x2="262" y2="107" />
            <circle class="gfx-pulse" cx="248" cy="107" r="3.5" />
            <rect class="gfx-hot" x="262" y="48" width="236" height="118" rx="6" />
            <text class="gfx-head" x="380" y="74" text-anchor="middle">
                "2 · BUILD-TIME ENCRYPTION"
            </text>
            <text class="gfx-text" x="380" y="102" text-anchor="middle">
                "generate-answer-keys.mjs"
            </text>
            <text class="gfx-note" x="380" y="130" text-anchor="middle">
                "one password per set"
            </text>
            <text class="gfx-note" x="380" y="150" text-anchor="middle">
                "PBKDF2-SHA256 · 310,000 iter · AES-256-GCM"
            </text>
            <line class="gfx-arrow" x1="498" y1="107" x2="518" y2="107" />
            <circle
                class="gfx-pulse"
                cx="502"
                cy="107"
                r="3.5"
                style="animation-delay:0.6s"
            />
            <rect class="gfx-panel" x="518" y="48" width="236" height="118" rx="6" />
            <text class="gfx-head" x="636" y="74" text-anchor="middle">
                "3 · STATIC SITE SHIPS"
            </text>
            <text class="gfx-text" x="636" y="102" text-anchor="middle">
                "ciphertext only"
            </text>
            <text class="gfx-note" x="636" y="130" text-anchor="middle">
                "/answer-keys/<slug>.json"
            </text>
            <text class="gfx-note" x="636" y="150" text-anchor="middle">
                "no plaintext key on the site"
            </text>
            <line class="gfx-arrow" x1="636" y1="166" x2="636" y2="190" />
            <circle class="gfx-pulse-y" cx="636" cy="170" r="3.5" />
            <rect class="gfx-panel" x="518" y="192" width="236" height="118" rx="6" />
            <text class="gfx-head" x="636" y="218" text-anchor="middle">
                "4 · PASSWORD ENTRY"
            </text>
            <text class="gfx-text" x="636" y="246" text-anchor="middle">
                "candidate enters the"
            </text>
            <text class="gfx-text" x="636" y="268" text-anchor="middle">
                "set password"
            </text>
            <text class="gfx-note" x="636" y="294" text-anchor="middle">
                "that set's own password"
            </text>
            <line class="gfx-arrow" x1="498" y1="251" x2="518" y2="251" />
            <circle
                class="gfx-pulse"
                cx="514"
                cy="251"
                r="3.5"
                style="animation-delay:1.2s"
            />
            <rect class="gfx-hot" x="262" y="192" width="236" height="118" rx="6" />
            <text class="gfx-head" x="380" y="218" text-anchor="middle">
                "5 · BROWSER DECRYPTS"
            </text>
            <text class="gfx-text" x="380" y="246" text-anchor="middle">
                "WebCrypto decrypts"
            </text>
            <text class="gfx-text" x="380" y="268" text-anchor="middle">
                "locally"
            </text>
            <text class="gfx-note" x="380" y="294" text-anchor="middle">
                "key derived in the browser only"
            </text>
            <line class="gfx-arrow" x1="244" y1="251" x2="262" y2="251" />
            <circle
                class="gfx-pulse"
                cx="258"
                cy="251"
                r="3.5"
                style="animation-delay:1.8s"
            />
            <rect class="gfx-panel" x="8" y="192" width="236" height="118" rx="6" />
            <text class="gfx-head" x="126" y="218" text-anchor="middle">
                "6 · CHECKED LOCALLY"
            </text>
            <text class="gfx-text" x="126" y="246" text-anchor="middle">
                "runner checks answers"
            </text>
            <text class="gfx-text" x="126" y="268" text-anchor="middle">
                "on the page"
            </text>
            <text class="gfx-note" x="126" y="294" text-anchor="middle">
                "Hide clears the rendered key"
            </text>
            <rect class="gfx-badge" x="105" y="330" width="550" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="355" text-anchor="middle">
                "PASSWORD IS NEVER STORED OR SENT — THE KEY NEVER LEAVES THE BROWSER"
            </text>
        </svg>
    }
    .into_any()
}

/// Tuxtype: the game loop across its activities — a prompt (falling
/// letters/words, or a phrase) is typed, accuracy and speed are
/// scored, and the next word or lesson step follows. The WPM the
///
/// scoring reports is the equation in the guide TOML.
fn tuxtype_loop() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 336"
            role="img"
            aria-label="Diagram: the TuxType game loop — a prompt (falling letters or words in Fish Cascade and Comet Zap, or a phrase) is typed as keystrokes, accuracy and speed are scored including the reported WPM, and the next word or lesson step follows; scripted lessons run 43 progressive finger exercises in order."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "THE GAME LOOP — TYPE WHAT FALLS"
            </text>
            <rect class="gfx-panel" x="8" y="48" width="168" height="120" rx="6" />
            <text class="gfx-head" x="92" y="74" text-anchor="middle">
                "1 · PROMPT"
            </text>
            <text class="gfx-text" x="92" y="102" text-anchor="middle">
                "letters + words"
            </text>
            <text class="gfx-text" x="92" y="124" text-anchor="middle">
                "fall, or a phrase"
            </text>
            <text class="gfx-note" x="92" y="150" text-anchor="middle">
                "Fish Cascade · Comet Zap"
            </text>
            <line class="gfx-arrow" x1="176" y1="108" x2="200" y2="108" />
            <circle class="gfx-pulse" cx="180" cy="108" r="3.5" />
            <rect class="gfx-panel" x="200" y="48" width="168" height="120" rx="6" />
            <text class="gfx-head" x="284" y="74" text-anchor="middle">
                "2 · KEYSTROKES"
            </text>
            <text class="gfx-text" x="284" y="102" text-anchor="middle">
                "type the letters"
            </text>
            <text class="gfx-text" x="284" y="124" text-anchor="middle">
                "to clear the word"
            </text>
            <text class="gfx-note" x="284" y="150" text-anchor="middle">
                "Tux eats fish, zaps comets"
            </text>
            <line class="gfx-arrow" x1="368" y1="108" x2="392" y2="108" />
            <circle
                class="gfx-pulse"
                cx="372"
                cy="108"
                r="3.5"
                style="animation-delay:0.6s"
            />
            <rect class="gfx-hot" x="392" y="48" width="168" height="120" rx="6" />
            <text class="gfx-head" x="476" y="74" text-anchor="middle">
                "3 · SCORE"
            </text>
            <text class="gfx-text" x="476" y="102" text-anchor="middle">
                "accuracy + speed"
            </text>
            <text class="gfx-text" x="476" y="124" text-anchor="middle">
                "measured"
            </text>
            <text class="gfx-note" x="476" y="150" text-anchor="middle">
                "WPM — equation below"
            </text>
            <line class="gfx-arrow" x1="560" y1="108" x2="584" y2="108" />
            <circle
                class="gfx-pulse"
                cx="564"
                cy="108"
                r="3.5"
                style="animation-delay:1.2s"
            />
            <rect class="gfx-panel" x="584" y="48" width="168" height="120" rx="6" />
            <text class="gfx-head" x="668" y="74" text-anchor="middle">
                "4 · NEXT"
            </text>
            <text class="gfx-text" x="668" y="102" text-anchor="middle">
                "next word, next"
            </text>
            <text class="gfx-text" x="668" y="124" text-anchor="middle">
                "lesson step"
            </text>
            <text class="gfx-note" x="668" y="150" text-anchor="middle">
                "the loop repeats"
            </text>
            <line class="gfx-arrow gfx-dash" x1="668" y1="168" x2="668" y2="196" />
            <line class="gfx-arrow gfx-dash" x1="92" y1="196" x2="668" y2="196" />
            <line class="gfx-arrow gfx-dash" x1="92" y1="168" x2="92" y2="196" />
            <rect class="gfx-panel" x="8" y="216" width="744" height="96" rx="6" />
            <text class="gfx-head" x="380" y="242" text-anchor="middle">
                "SCRIPTED LESSONS + PHRASE TYPING"
            </text>
            <text class="gfx-text" x="380" y="270" text-anchor="middle">
                "XML scripts run activities in order — Practice + basic_lesson_01 … basic_lesson_43"
            </text>
            <text class="gfx-note" x="380" y="294" text-anchor="middle">
                "phrase typing: complete sentences, accuracy and typing speed measured · custom word lists via the Word List Editor"
            </text>
        </svg>
    }
    .into_any()
}

/// Unreal: the setup pipeline the repo's quickstart script runs —
/// installer choices, the engine source build, the checked-in
/// configuration tree, and the bundled sample projects. The engine
///
/// itself is cloned from Epic at install time; this repo is setup
/// and content, not an engine binary.
fn unreal_pipeline() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 316"
            role="img"
            aria-label="Diagram: the Unreal setup pipeline — the quickstart installer takes version, platform and build-system choices, clones and builds Epic's UnrealEngine source, applies the checked-in UnrealBuildTool configuration tree, and opens the bundled sample projects (Learning Agents and ML Deformer) in the editor."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "INSTALLER → ENGINE BUILD → CONFIG → SAMPLES"
            </text>
            <rect class="gfx-panel" x="8" y="48" width="180" height="150" rx="6" />
            <text class="gfx-head" x="98" y="74" text-anchor="middle">
                "1 · INSTALLER"
            </text>
            <text class="gfx-text" x="98" y="102" text-anchor="middle">
                "quickstart.sh"
            </text>
            <text class="gfx-text" x="98" y="124" text-anchor="middle">
                "version: 5.6 · 5.5"
            </text>
            <text class="gfx-text" x="98" y="146" text-anchor="middle">
                "5.4 · custom"
            </text>
            <text class="gfx-note" x="98" y="172" text-anchor="middle">
                "platforms (default Linux)"
            </text>
            <line class="gfx-arrow" x1="188" y1="123" x2="196" y2="123" />
            <circle class="gfx-pulse" cx="190" cy="123" r="3.5" />
            <rect class="gfx-hot" x="196" y="48" width="180" height="150" rx="6" />
            <text class="gfx-head" x="286" y="74" text-anchor="middle">
                "2 · ENGINE BUILD"
            </text>
            <text class="gfx-text" x="286" y="102" text-anchor="middle">
                "clone + build source"
            </text>
            <text class="gfx-note" x="212" y="124">
                "Setup.sh"
            </text>
            <text class="gfx-note" x="212" y="143">
                "GenerateProjectFiles.sh"
            </text>
            <text class="gfx-note" x="212" y="162">
                "RunUAT BuildGraph:"
            </text>
            <text class="gfx-note" x="212" y="181">
                "Installed Build (or Makefile)"
            </text>
            <line class="gfx-arrow" x1="376" y1="123" x2="384" y2="123" />
            <circle
                class="gfx-pulse"
                cx="378"
                cy="123"
                r="3.5"
                style="animation-delay:0.6s"
            />
            <rect class="gfx-panel" x="384" y="48" width="180" height="150" rx="6" />
            <text class="gfx-head" x="474" y="74" text-anchor="middle">
                "3 · CONFIG TREE"
            </text>
            <text class="gfx-text" x="474" y="102" text-anchor="middle">
                ".config/ checked in"
            </text>
            <text class="gfx-text" x="474" y="124" text-anchor="middle">
                "UnrealBuildTool"
            </text>
            <text class="gfx-text" x="474" y="146" text-anchor="middle">
                "BuildConfiguration.xml"
            </text>
            <text class="gfx-note" x="474" y="172" text-anchor="middle">
                "Cpp20 · AVX2 minimum"
            </text>
            <line class="gfx-arrow" x1="564" y1="123" x2="572" y2="123" />
            <circle
                class="gfx-pulse"
                cx="566"
                cy="123"
                r="3.5"
                style="animation-delay:1.2s"
            />
            <rect class="gfx-panel" x="572" y="48" width="180" height="150" rx="6" />
            <text class="gfx-head" x="662" y="74" text-anchor="middle">
                "4 · SAMPLES"
            </text>
            <text class="gfx-text" x="662" y="102" text-anchor="middle">
                "edu/ Learning Agents"
            </text>
            <text class="gfx-note" x="662" y="124" text-anchor="middle">
                "car project · UE 5.3+"
            </text>
            <text class="gfx-text" x="662" y="146" text-anchor="middle">
                "Examples/MLDeformer/"
            </text>
            <text class="gfx-note" x="662" y="168" text-anchor="middle">
                "ML Deformer · 5.5.2"
            </text>
            <rect class="gfx-badge" x="130" y="222" width="500" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="247" text-anchor="middle">
                "LAUNCH: UnrealEditor [YourProject.uproject]"
            </text>
            <text class="gfx-note" x="380" y="290" text-anchor="middle">
                "tools symlinked into ~/.local/bin · the engine is cloned from EpicGames/UnrealEngine at install time"
            </text>
        </svg>
    }
    .into_any()
}
