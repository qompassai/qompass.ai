//! Batch 1 concept diagrams for solution pages. Owned ids are listed
//!
//! in `IDS`; the registry in `diagrams.rs` delegates here. Follow
//! the authoring contract in that module's header: `gfx-*` classes
//! only, text at 10.5px or larger at viewBox scale, motion only via
//! the stylesheet's reduced-motion-gated classes, and every figure
//! must read correctly as a static diagram.

use leptos::prelude::*;

/// The diagram ids this module owns.
pub const IDS: &[&str] = &["network-dns", "radar-pipeline", "tor-circuit", "volta-flows", "vongola-phases"];

/// The bare `<svg>` for one of this module's ids, or `None`.
pub fn diagram(id: &'static str) -> Option<AnyView> {
    match id {
        "network-dns" => Some(network_dns()),
        "radar-pipeline" => Some(radar_pipeline()),
        "tor-circuit" => Some(tor_circuit()),
        "volta-flows" => Some(volta_flows()),
        "vongola-phases" => Some(vongola_phases()),
        _ => None,
    }
}

/// Network: under the flake's NixOS module a stub-resolved query is
///
/// handed to a loopback-only Unbound, which forwards everything
/// upstream over DNS-over-TLS; `WireGuard` and the nftables template
/// are the surrounding posture. Every value is from `flake.nix` or
///
/// the shipped `nftables/nftables.conf` template.
fn network_dns() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 376"
            role="img"
            aria-label="Diagram: a DNS query flows from an application through systemd-resolved's stub listener to Unbound on loopback, which forwards it over DNS-over-TLS on port 853 to 1.1.1.1, 1.0.0.1, and 9.9.9.9; WireGuard and an nftables template form the surrounding posture."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "DNS UNDER THE HARDENED MODULE — NOTHING LEAVES IN PLAINTEXT"
            </text>
            <rect class="gfx-chip" x="8" y="44" width="104" height="64" rx="6" />
            <text class="gfx-text" x="60" y="72" text-anchor="middle">
                "APP / CLIENT"
            </text>
            <text class="gfx-note" x="60" y="92" text-anchor="middle">
                "issues a query"
            </text>
            <line class="gfx-arrow" x1="114" y1="76" x2="134" y2="76" />
            <circle class="gfx-pulse" cx="118" cy="76" r="3.5" />
            <rect class="gfx-panel" x="136" y="44" width="148" height="64" rx="6" />
            <text class="gfx-text" x="210" y="72" text-anchor="middle">
                "SYSTEMD-RESOLVED"
            </text>
            <text class="gfx-note" x="210" y="92" text-anchor="middle">
                "stub listener · DNSSEC on"
            </text>
            <line class="gfx-arrow" x1="286" y1="76" x2="306" y2="76" />
            <rect class="gfx-hot" x="308" y="44" width="148" height="64" rx="6" />
            <text class="gfx-text" x="382" y="72" text-anchor="middle">
                "UNBOUND"
            </text>
            <text class="gfx-note" x="382" y="92" text-anchor="middle">
                "loopback only · 127.0.0.1, ::1"
            </text>
            <line class="gfx-arrow" x1="458" y1="76" x2="478" y2="76" />
            <circle
                class="gfx-pulse"
                cx="462"
                cy="76"
                r="3.5"
                style="animation-delay:1.1s"
            />
            <rect class="gfx-panel" x="480" y="44" width="272" height="64" rx="6" />
            <text class="gfx-text" x="616" y="72" text-anchor="middle">
                "UPSTREAM — DNS-OVER-TLS :853"
            </text>
            <text class="gfx-note" x="616" y="92" text-anchor="middle">
                "1.1.1.1@853 · 1.0.0.1@853 · 9.9.9.9@853 (+ IPv6)"
            </text>
            <text class="gfx-note" x="210" y="126" text-anchor="middle">
                "resolved is pointed at Unbound: DNS=127.0.0.1"
            </text>
            <text class="gfx-note" x="382" y="126" text-anchor="middle">
                "NetworkManager DNS plugin: systemd-resolved"
            </text>
            <rect class="gfx-panel" x="8" y="140" width="448" height="96" rx="6" />
            <text class="gfx-head" x="232" y="164" text-anchor="middle">
                "UNBOUND HARDENING — FROM FLAKE.NIX"
            </text>
            <text class="gfx-text" x="24" y="190">
                "qname-minimisation · prefetch + prefetch-key · hide-identity · hide-version"
            </text>
            <text class="gfx-text" x="24" y="214">
                "harden-* flags on · cache TTL bounded 300–14400 s · all queries TLS-forwarded"
            </text>
            <rect class="gfx-panel" x="480" y="140" width="272" height="96" rx="6" />
            <text class="gfx-head" x="616" y="164" text-anchor="middle">
                "RESOLVED FALLBACK"
            </text>
            <text class="gfx-text" x="496" y="190">
                "fallbackDns: 1.1.1.1 · 9.9.9.9"
            </text>
            <text class="gfx-note" x="496" y="214">
                "Wi-Fi power saving off (module)"
            </text>
            <rect class="gfx-panel" x="8" y="252" width="744" height="112" rx="6" />
            <text class="gfx-head" x="380" y="276" text-anchor="middle">
                "SURROUNDING POSTURE — WIREGUARD + NFTABLES"
            </text>
            <text class="gfx-text" x="24" y="302">
                "nftables template (table inet my_filter): input policy drop · established/related + loopback accepted"
            </text>
            <text class="gfx-text" x="24" y="326">
                "WireGuard in on UDP 51820 · kernel sysctls refuse ICMP redirects (IPv4 + IPv6)"
            </text>
            <text class="gfx-note" x="24" y="350">
                "the nftables file is a standalone template — its header states nothing was applied to a live firewall"
            </text>
        </svg>
    }
    .into_any()
}

/// Radar: the project's own pipeline — scope, generate, execute,
///
/// measure, report — mapped onto the Measure function of the NIST
/// AI Risk Management Framework and its seven trustworthiness
/// characteristics. Workflow facts are from Radar's own docs; the
///
/// characteristics are the RMF's.
fn radar_pipeline() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 420"
            role="img"
            aria-label="Diagram: Radar's assessment pipeline — scope the system under test, generate attacks and tasks as entry points, execute them in a Testbed worker, measure results against the seven NIST AI RMF trustworthiness characteristics, and report — the project's own mapping onto the RMF Measure function."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "RADAR'S PIPELINE — ITS OWN MAPPING ON THE AI RMF MEASURE FUNCTION"
            </text>
            <rect class="gfx-chip" x="8" y="44" width="136" height="64" rx="6" />
            <text class="gfx-text" x="76" y="70" text-anchor="middle">
                "1 · SCOPE"
            </text>
            <text class="gfx-note" x="76" y="88" text-anchor="middle">
                "system under test"
            </text>
            <text class="gfx-note" x="76" y="102" text-anchor="middle">
                "1st / 2nd / 3rd-party"
            </text>
            <line class="gfx-arrow" x1="146" y1="76" x2="158" y2="76" />
            <rect class="gfx-chip" x="160" y="44" width="136" height="64" rx="6" />
            <text class="gfx-text" x="228" y="70" text-anchor="middle">
                "2 · GENERATE"
            </text>
            <text class="gfx-note" x="228" y="88" text-anchor="middle">
                "attacks + tasks as"
            </text>
            <text class="gfx-note" x="228" y="102" text-anchor="middle">
                "entry points (plugins)"
            </text>
            <line class="gfx-arrow" x1="298" y1="76" x2="310" y2="76" />
            <circle class="gfx-pulse" cx="302" cy="76" r="3.5" />
            <rect class="gfx-hot" x="312" y="44" width="136" height="64" rx="6" />
            <text class="gfx-text" x="380" y="70" text-anchor="middle">
                "3 · EXECUTE"
            </text>
            <text class="gfx-note" x="380" y="88" text-anchor="middle">
                "Testbed worker runs it"
            </text>
            <text class="gfx-note" x="380" y="102" text-anchor="middle">
                "Docker · job environment"
            </text>
            <line class="gfx-arrow" x1="450" y1="76" x2="462" y2="76" />
            <rect class="gfx-chip" x="464" y="44" width="136" height="64" rx="6" />
            <text class="gfx-text" x="532" y="70" text-anchor="middle">
                "4 · MEASURE"
            </text>
            <text class="gfx-note" x="532" y="88" text-anchor="middle">
                "results + artifacts"
            </text>
            <text class="gfx-note" x="532" y="102" text-anchor="middle">
                "resources snapshotted"
            </text>
            <line class="gfx-arrow" x1="602" y1="76" x2="614" y2="76" />
            <rect class="gfx-chip" x="616" y="44" width="136" height="64" rx="6" />
            <text class="gfx-text" x="684" y="70" text-anchor="middle">
                "5 · REPORT"
            </text>
            <text class="gfx-note" x="684" y="88" text-anchor="middle">
                "API · web UI · MLflow"
            </text>
            <text class="gfx-note" x="684" y="102" text-anchor="middle">
                "history fully traceable"
            </text>
            <rect class="gfx-panel" x="8" y="128" width="744" height="104" rx="6" />
            <text class="gfx-head" x="380" y="152" text-anchor="middle">
                "TESTBED REST API — SUBMISSION + LEDGER"
            </text>
            <text class="gfx-text" x="24" y="180">
                "a job is an entry-point archive plus parameter choices, submitted to the API — the heart of the architecture"
            </text>
            <text class="gfx-text" x="24" y="204">
                "tasks are Python functions in the plugin registry (attacks, defenses, data, estimators, metrics), ordered by Prefect"
            </text>
            <text class="gfx-note" x="24" y="222">
                "the API also registers experiments, logs results and artifacts, and hosts the model registry · plugins install with zero downtime"
            </text>
            <rect class="gfx-panel" x="8" y="252" width="744" height="112" rx="6" />
            <text class="gfx-head" x="380" y="276" text-anchor="middle">
                "MEASURED AGAINST — THE SEVEN TRUSTWORTHINESS CHARACTERISTICS"
            </text>
            <rect class="gfx-chip" x="8" y="288" width="170" height="32" rx="6" />
            <text class="gfx-text" x="93" y="309" text-anchor="middle">
                "valid + reliable"
            </text>
            <rect class="gfx-chip" x="186" y="288" width="110" height="32" rx="6" />
            <text class="gfx-text" x="241" y="309" text-anchor="middle">
                "safe"
            </text>
            <rect class="gfx-chip" x="304" y="288" width="180" height="32" rx="6" />
            <text class="gfx-text" x="394" y="309" text-anchor="middle">
                "secure + resilient"
            </text>
            <rect class="gfx-chip" x="492" y="288" width="260" height="32" rx="6" />
            <text class="gfx-text" x="622" y="309" text-anchor="middle">
                "accountable + transparent"
            </text>
            <rect class="gfx-chip" x="8" y="328" width="170" height="32" rx="6" />
            <text class="gfx-text" x="93" y="349" text-anchor="middle">
                "explainable"
            </text>
            <rect class="gfx-chip" x="186" y="328" width="180" height="32" rx="6" />
            <text class="gfx-text" x="276" y="349" text-anchor="middle">
                "privacy-enhanced"
            </text>
            <rect class="gfx-chip" x="374" y="328" width="110" height="32" rx="6" />
            <text class="gfx-text" x="429" y="349" text-anchor="middle">
                "fair"
            </text>
            <text class="gfx-note" x="492" y="349">
                "characteristics as the AI RMF names them"
            </text>
            <text class="gfx-note" x="380" y="404" text-anchor="middle">
                "the pipeline shape is the project's own — the RMF supplies the Measure function and the characteristics, not this workflow"
            </text>
        </svg>
    }
    .into_any()
}

/// Tor: a three-relay circuit. The client builds the circuit
///
/// incrementally (EXTEND), wrapping the payload in one encryption
/// layer per relay; each relay peels exactly its own layer, so no
/// single relay knows both ends. Per the Tor protocol spec.
fn tor_circuit() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 392"
            role="img"
            aria-label="Diagram: a Tor circuit — client, guard, middle, and exit relays, then the destination. The client wraps the payload in three encryption layers; the guard peels one, the middle peels one, and the exit peels the last, so no single relay knows both the origin and the destination."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "A THREE-RELAY CIRCUIT — ONE ENCRYPTION LAYER PEELED PER HOP"
            </text>
            <rect class="gfx-hot" x="8" y="48" width="104" height="64" rx="6" />
            <text class="gfx-text" x="60" y="76" text-anchor="middle">
                "CLIENT"
            </text>
            <text class="gfx-note" x="60" y="94" text-anchor="middle">
                "builds the circuit"
            </text>
            <line class="gfx-arrow" x1="114" y1="80" x2="148" y2="80" />
            <circle class="gfx-pulse" cx="120" cy="80" r="3.5" />
            <rect class="gfx-panel" x="156" y="48" width="120" height="64" rx="6" />
            <text class="gfx-text" x="216" y="76" text-anchor="middle">
                "GUARD"
            </text>
            <text class="gfx-note" x="216" y="94" text-anchor="middle">
                "entry relay"
            </text>
            <line class="gfx-arrow" x1="278" y1="80" x2="312" y2="80" />
            <rect class="gfx-panel" x="320" y="48" width="120" height="64" rx="6" />
            <text class="gfx-text" x="380" y="76" text-anchor="middle">
                "MIDDLE"
            </text>
            <text class="gfx-note" x="380" y="94" text-anchor="middle">
                "relay only"
            </text>
            <line class="gfx-arrow" x1="442" y1="80" x2="476" y2="80" />
            <circle
                class="gfx-pulse"
                cx="448"
                cy="80"
                r="3.5"
                style="animation-delay:1.1s"
            />
            <rect class="gfx-panel" x="484" y="48" width="120" height="64" rx="6" />
            <text class="gfx-text" x="544" y="76" text-anchor="middle">
                "EXIT"
            </text>
            <text class="gfx-note" x="544" y="94" text-anchor="middle">
                "final relay"
            </text>
            <line class="gfx-arrow" x1="606" y1="80" x2="640" y2="80" />
            <rect class="gfx-chip" x="648" y="48" width="104" height="64" rx="6" />
            <text class="gfx-text" x="700" y="76" text-anchor="middle">
                "DESTINATION"
            </text>
            <text class="gfx-note" x="700" y="94" text-anchor="middle">
                "web service"
            </text>
            <text class="gfx-text" x="60" y="140" text-anchor="middle">
                "wraps 3 layers"
            </text>
            <text class="gfx-note" x="60" y="160" text-anchor="middle">
                "one per relay"
            </text>
            <text class="gfx-text" x="216" y="140" text-anchor="middle">
                "peels layer 1"
            </text>
            <text class="gfx-note" x="216" y="160" text-anchor="middle">
                "knows the client,"
            </text>
            <text class="gfx-note" x="216" y="176" text-anchor="middle">
                "not the destination"
            </text>
            <text class="gfx-text" x="380" y="140" text-anchor="middle">
                "peels layer 2"
            </text>
            <text class="gfx-note" x="380" y="160" text-anchor="middle">
                "knows neither end"
            </text>
            <text class="gfx-text" x="544" y="140" text-anchor="middle">
                "peels layer 3"
            </text>
            <text class="gfx-note" x="544" y="160" text-anchor="middle">
                "sees the destination,"
            </text>
            <text class="gfx-note" x="544" y="176" text-anchor="middle">
                "not the client"
            </text>
            <text class="gfx-text" x="700" y="140" text-anchor="middle">
                "plaintext arrives"
            </text>
            <text class="gfx-note" x="700" y="160" text-anchor="middle">
                "replies retrace the"
            </text>
            <text class="gfx-note" x="700" y="176" text-anchor="middle">
                "circuit, re-wrapped"
            </text>
            <rect class="gfx-panel" x="8" y="196" width="744" height="88" rx="6" />
            <text class="gfx-head" x="380" y="220" text-anchor="middle">
                "HOW THE CIRCUIT IS BUILT — INCREMENTALLY, PER TOR-SPEC"
            </text>
            <text class="gfx-text" x="24" y="248">
                "the client negotiates a session key with the guard, then extends the circuit one hop at a time"
            </text>
            <text class="gfx-text" x="24" y="270">
                "each extension travels inside the existing circuit — a relay never holds another hop's key"
            </text>
            <rect class="gfx-badge" x="80" y="300" width="600" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="325" text-anchor="middle">
                "NO SINGLE RELAY KNOWS BOTH ENDS OF THE CONVERSATION"
            </text>
            <text class="gfx-note" x="380" y="364" text-anchor="middle">
                "this repo's shipped torrc also publishes v3 onion services — rendezvous circuits are specified in rend-spec-v3"
            </text>
            <text class="gfx-note" x="380" y="382" text-anchor="middle">
                "applications reach the circuit through the local SOCKS port, directly or via torsocks"
            </text>
        </svg>
    }
    .into_any()
}

/// Volta: the two keyserver flows. Publication is gated on email
///
/// verification — an address is not searchable until its owner
/// clicks the mailed token link — and lookups arrive over HKP, the
/// VKS API, and Web Key Directory. Routes are Volta's own, from its
/// book.
fn volta_flows() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 404"
            role="img"
            aria-label="Diagram: Volta's two flows — publication (submit a key, parse and clean it, store it, then publish the email address only after the owner clicks a mailed verification token) and lookup over three surfaces (HKP, the VKS API, and Web Key Directory) returning the published key."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "EMAIL-VERIFIED PUBLICATION — AND THREE LOOKUP SURFACES"
            </text>
            <rect class="gfx-panel" x="8" y="40" width="744" height="168" rx="6" />
            <text class="gfx-head" x="24" y="64">
                "A · PUBLISH + VERIFY"
            </text>
            <rect class="gfx-chip" x="24" y="80" width="148" height="52" rx="6" />
            <text class="gfx-text" x="98" y="102" text-anchor="middle">
                "SUBMIT KEY"
            </text>
            <text class="gfx-note" x="98" y="120" text-anchor="middle">
                "/pks/add · VKS upload"
            </text>
            <line class="gfx-arrow" x1="174" y1="106" x2="188" y2="106" />
            <rect class="gfx-chip" x="196" y="80" width="148" height="52" rx="6" />
            <text class="gfx-text" x="270" y="102" text-anchor="middle">
                "PARSE + CLEAN"
            </text>
            <text class="gfx-note" x="270" y="120" text-anchor="middle">
                "Sequoia parses · policy cleans"
            </text>
            <line class="gfx-arrow" x1="346" y1="106" x2="360" y2="106" />
            <circle class="gfx-pulse" cx="350" cy="106" r="3.5" />
            <rect class="gfx-chip" x="368" y="80" width="148" height="52" rx="6" />
            <text class="gfx-text" x="442" y="102" text-anchor="middle">
                "STORE"
            </text>
            <text class="gfx-note" x="442" y="120" text-anchor="middle">
                "fingerprint · key ID · email"
            </text>
            <line class="gfx-arrow" x1="518" y1="106" x2="532" y2="106" />
            <rect class="gfx-hot" x="540" y="80" width="196" height="52" rx="6" />
            <text class="gfx-text" x="638" y="102" text-anchor="middle">
                "EMAIL VERIFIED → PUBLISHED"
            </text>
            <text class="gfx-note" x="638" y="120" text-anchor="middle">
                "owner clicks the mailed token link"
            </text>
            <text class="gfx-text" x="24" y="158">
                "an address is not published as searchable until its owner confirms it — unverified and revoked material is stripped at intake"
            </text>
            <text class="gfx-note" x="24" y="182">
                "owners manage or delete addresses via tokenized /manage links · operators bulk-import keyrings with voltactl"
            </text>
            <rect class="gfx-panel" x="8" y="224" width="744" height="168" rx="6" />
            <text class="gfx-head" x="24" y="248">
                "B · LOOKUP — THREE SURFACES, ONE PUBLISHED STORE"
            </text>
            <rect class="gfx-chip" x="24" y="264" width="220" height="52" rx="6" />
            <text class="gfx-text" x="134" y="286" text-anchor="middle">
                "HKP"
            </text>
            <text class="gfx-note" x="134" y="304" text-anchor="middle">
                "/pks/lookup"
            </text>
            <rect class="gfx-chip" x="268" y="264" width="220" height="52" rx="6" />
            <text class="gfx-text" x="378" y="286" text-anchor="middle">
                "VKS API"
            </text>
            <text class="gfx-note" x="378" y="304" text-anchor="middle">
                "by-fingerprint · by-email · by-keyid"
            </text>
            <rect class="gfx-chip" x="512" y="264" width="224" height="52" rx="6" />
            <text class="gfx-text" x="624" y="286" text-anchor="middle">
                "WEB KEY DIRECTORY"
            </text>
            <text class="gfx-note" x="624" y="304" text-anchor="middle">
                "/.well-known/openpgpkey/…/hu/<hash>"
            </text>
            <line class="gfx-arrow" x1="134" y1="318" x2="134" y2="336" />
            <line class="gfx-arrow" x1="378" y1="318" x2="378" y2="336" />
            <line class="gfx-arrow" x1="624" y1="318" x2="624" y2="336" />
            <circle class="gfx-pulse-y" cx="378" cy="322" r="3.5" />
            <rect class="gfx-badge" x="184" y="338" width="392" height="36" rx="6" />
            <text class="gfx-badge-text" x="380" y="361" text-anchor="middle">
                "PUBLISHED KEY RETURNED"
            </text>
            <text class="gfx-note" x="380" y="386" text-anchor="middle">
                "WKD also serves /policy alongside the hu/ lookup path · web UI upload at /upload"
            </text>
        </svg>
    }
    .into_any()
}

/// Vongola: one HTTPS request through Pingora's phase pipeline, with
///
/// Vongola's own work in each phase — the request filter does
/// routing and auth before any upstream is touched, peer selection
/// picks how the route is served, and logging runs last through a
/// bounded redacting pipeline. Phase names are Pingora's.
fn vongola_phases() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 400"
            role="img"
            aria-label="Diagram: a request through Pingora's phases as Vongola implements them — request filter (redirect, route lookup, body bound, auth), upstream peer selection (static, chained, or round-robin over healthy upstreams), upstream filter, response filter (security headers), and logging through a bounded redacting pipeline; upstream connections are pooled and reused."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "ONE REQUEST THROUGH PINGORA'S PHASES — VONGOLA'S HOOKS"
            </text>
            <rect class="gfx-chip" x="8" y="40" width="176" height="44" rx="6" />
            <text class="gfx-text" x="96" y="67" text-anchor="middle">
                "1 · REQUEST FILTER"
            </text>
            <text class="gfx-text" x="204" y="58">
                "www → apex 308 · route by host + longest path prefix"
            </text>
            <text class="gfx-note" x="204" y="76">
                "body-size bound (413 when over) · route auth: basic · JWT · OAuth2 — before any upstream is touched"
            </text>
            <line class="gfx-arrow" x1="96" y1="86" x2="96" y2="98" />
            <circle class="gfx-pulse-y" cx="96" cy="90" r="3.5" />
            <rect class="gfx-chip" x="8" y="100" width="176" height="44" rx="6" />
            <text class="gfx-text" x="96" y="127" text-anchor="middle">
                "2 · PEER SELECTION"
            </text>
            <text class="gfx-text" x="204" y="118">
                "static files · chained fetch · round-robin over healthy upstreams"
            </text>
            <text class="gfx-note" x="204" y="136">
                "the route decides how it is served · every upstream down is a 502"
            </text>
            <line class="gfx-arrow" x1="96" y1="146" x2="96" y2="158" />
            <rect class="gfx-chip" x="8" y="160" width="176" height="44" rx="6" />
            <text class="gfx-text" x="96" y="187" text-anchor="middle">
                "3 · UPSTREAM FILTER"
            </text>
            <text class="gfx-text" x="204" y="178">
                "the request is filtered for the chosen peer (Pingora upstream_request_filter)"
            </text>
            <text class="gfx-note" x="204" y="196">
                "last touch before the request leaves for the origin"
            </text>
            <line class="gfx-arrow" x1="96" y1="206" x2="96" y2="218" />
            <circle
                class="gfx-pulse-y"
                cx="96"
                cy="210"
                r="3.5"
                style="animation-delay:0.9s"
            />
            <rect class="gfx-chip" x="8" y="220" width="176" height="44" rx="6" />
            <text class="gfx-text" x="96" y="247" text-anchor="middle">
                "4 · RESPONSE FILTER"
            </text>
            <text class="gfx-text" x="204" y="238">
                "the response is filtered on the way back to the client"
            </text>
            <text class="gfx-note" x="204" y="256">
                "routes carry the standard security header set (on by default)"
            </text>
            <line class="gfx-arrow" x1="96" y1="266" x2="96" y2="278" />
            <rect class="gfx-chip" x="8" y="280" width="176" height="44" rx="6" />
            <text class="gfx-text" x="96" y="307" text-anchor="middle">
                "5 · LOGGING"
            </text>
            <text class="gfx-text" x="204" y="298">
                "access line + metrics recorded once the response completes"
            </text>
            <text class="gfx-note" x="204" y="316">
                "through a bounded (1024) redacting log pipeline"
            </text>
            <rect class="gfx-badge" x="130" y="338" width="500" height="36" rx="6" />
            <text class="gfx-badge-text" x="380" y="361" text-anchor="middle">
                "UPSTREAM CONNECTIONS ARE POOLED + REUSED"
            </text>
            <text class="gfx-note" x="380" y="392" text-anchor="middle">
                "before phase 1: SNI selects the certificate (TlsAccept) · TLS 1.3 only · one versioned bundle, swapped atomically by POST /api/reload"
            </text>
        </svg>
    }
    .into_any()
}
