//! Batch 0 concept diagrams — the pilot pages (Qssl, Qxet, Light
//! Show). Owned ids are listed in `IDS`; the registry in
//!
//! `diagrams.rs` delegates here. Follow the authoring contract in
//! that module's header: `gfx-*` classes only, text at 10.5px or
//! larger at viewBox scale, motion only via the stylesheet's
//! reduced-motion-gated classes, and every figure must read
//! correctly as a static diagram.

use leptos::prelude::*;

/// The diagram ids this module owns.
pub const IDS: &[&str] = &["lightshow-budget", "qssl-config", "qssl-handshake", "qxet-cdc"];

/// The bare `<svg>` for one of this module's ids, or `None`.
pub fn diagram(id: &'static str) -> Option<AnyView> {
    match id {
        "qssl-config" => Some(qssl_config()),
        "qssl-handshake" => Some(qssl_handshake()),
        "qxet-cdc" => Some(qxet_cdc()),
        "lightshow-budget" => Some(lightshow_budget()),
        _ => None,
    }
}

/// Qssl: one shipped OpenSSL configuration wires the providers, the
/// ML-DSA OID registrations, and the named TLS policies a connection
/// then chooses from. Every label is a section, provider, or policy
///
/// named in `qssl/qssl.cnf` itself.
fn qssl_config() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 336"
            role="img"
            aria-label="Diagram: qssl.cnf sections wire the OpenSSL providers (the OQS provider supplies the post-quantum algorithms) and name five TLS policies; a connection picks one policy."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "ONE CONFIG FILE → PROVIDERS → NAMED POLICIES"
            </text>
            <rect class="gfx-panel" x="8" y="48" width="204" height="224" rx="6" />
            <text class="gfx-head" x="110" y="72" text-anchor="middle">
                "QSSL.CNF"
            </text>
            <text class="gfx-text" x="24" y="100">
                "[openssl_init]"
            </text>
            <text class="gfx-text" x="24" y="126">
                "[evp_settings] fips_mode"
            </text>
            <text class="gfx-text" x="24" y="152">
                "[provider_sect]"
            </text>
            <text class="gfx-text" x="24" y="178">
                "[qai_oid] ML-DSA OIDs"
            </text>
            <text class="gfx-text" x="24" y="204">
                "[ssl_configuration]"
            </text>
            <text class="gfx-text" x="24" y="230">
                "CA / req / tsa sections"
            </text>
            <text class="gfx-note" x="24" y="256">
                "config_diagnostics = 1"
            </text>
            <line class="gfx-arrow" x1="220" y1="160" x2="256" y2="160" />
            <circle class="gfx-pulse" cx="224" cy="160" r="3.5" />
            <rect class="gfx-panel" x="264" y="48" width="216" height="224" rx="6" />
            <text class="gfx-head" x="372" y="72" text-anchor="middle">
                "PROVIDERS AT STARTUP"
            </text>
            <text class="gfx-text" x="282" y="102">
                "base · default · legacy"
            </text>
            <text class="gfx-text" x="282" y="128">
                "fips · pkcs11"
            </text>
            <rect class="gfx-hot" x="280" y="146" width="184" height="56" rx="6" />
            <text class="gfx-text" x="372" y="170" text-anchor="middle">
                "oqsprovider"
            </text>
            <text class="gfx-note" x="372" y="190" text-anchor="middle">
                "ML-KEM · ML-DSA · Falcon"
            </text>
            <text class="gfx-note" x="282" y="230">
                "every provider activate = 1"
            </text>
            <text class="gfx-note" x="282" y="254">
                "default: security_level=3"
            </text>
            <line class="gfx-arrow" x1="488" y1="160" x2="524" y2="160" />
            <circle
                class="gfx-pulse"
                cx="492"
                cy="160"
                r="3.5"
                style="animation-delay:1.2s"
            />
            <rect class="gfx-chip" x="532" y="48" width="220" height="36" rx="6" />
            <text class="gfx-text" x="642" y="70" text-anchor="middle">
                "system_default"
            </text>
            <rect class="gfx-chip" x="532" y="92" width="220" height="36" rx="6" />
            <text class="gfx-text" x="642" y="114" text-anchor="middle">
                "level3 — hybrid groups"
            </text>
            <rect class="gfx-chip" x="532" y="136" width="220" height="36" rx="6" />
            <text class="gfx-text" x="642" y="158" text-anchor="middle">
                "fips — P-256:P-384:P-521"
            </text>
            <rect class="gfx-hot" x="532" y="180" width="220" height="36" rx="6" />
            <text class="gfx-text" x="642" y="202" text-anchor="middle">
                "oqs — ML-KEM groups only"
            </text>
            <rect class="gfx-chip" x="532" y="224" width="220" height="36" rx="6" />
            <text class="gfx-text" x="642" y="246" text-anchor="middle">
                "tpm2"
            </text>
            <rect class="gfx-badge" x="452" y="288" width="300" height="40" rx="6" />
            <text class="gfx-badge-text" x="602" y="313" text-anchor="middle">
                "A CONNECTION PICKS ONE POLICY"
            </text>
            <text class="gfx-note" x="8" y="313">
                "CipherString · Groups · SignatureAlgorithms per policy"
            </text>
        </svg>
    }
    .into_any()
}

/// Qssl: a TLS 1.3 handshake under the `oqs` policy — hybrid key
/// exchange in a single `key_share`, an ML-DSA certificate, and session
/// keys derived from both shared secrets. Message order per RFC 8446.
fn qssl_handshake() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 448"
            role="img"
            aria-label="Diagram: TLS 1.3 handshake sequence between client and server using the hybrid group x25519_mlkem768 and an ML-DSA certificate; both shared secrets feed the key schedule."
        >
            <text class="gfx-head" x="110" y="30" text-anchor="middle">
                "CLIENT"
            </text>
            <text class="gfx-head" x="650" y="30" text-anchor="middle">
                "SERVER"
            </text>
            <line class="gfx-arrow gfx-dash" x1="110" y1="44" x2="110" y2="392" />
            <line class="gfx-arrow gfx-dash" x1="650" y1="44" x2="650" y2="392" />
            <rect class="gfx-panel" x="126" y="138" width="508" height="158" rx="6" />
            <text class="gfx-note" x="622" y="156" text-anchor="end">
                "encrypted under handshake keys"
            </text>
            <line class="gfx-arrow" x1="110" y1="78" x2="650" y2="78" />
            <circle class="gfx-pulse" cx="118" cy="78" r="3.5" />
            <text class="gfx-text" x="380" y="68" text-anchor="middle">
                "ClientHello — key_share: x25519_mlkem768"
            </text>
            <text class="gfx-note" x="380" y="94" text-anchor="middle">
                "sigalgs offered: mldsa65 · mldsa87 · falcon512 · falcon1024"
            </text>
            <line class="gfx-arrow" x1="650" y1="118" x2="110" y2="118" />
            <circle
                class="gfx-pulse"
                cx="560"
                cy="118"
                r="3.5"
                style="animation-delay:0.8s"
            />
            <text class="gfx-text" x="380" y="108" text-anchor="middle">
                "ServerHello — hybrid group chosen"
            </text>
            <text class="gfx-text" x="380" y="182" text-anchor="middle">
                "EncryptedExtensions"
            </text>
            <line class="gfx-arrow" x1="650" y1="190" x2="110" y2="190" />
            <text class="gfx-text" x="380" y="216" text-anchor="middle">
                "Certificate — ML-DSA-65 chain"
            </text>
            <line class="gfx-arrow" x1="650" y1="224" x2="110" y2="224" />
            <text class="gfx-text" x="380" y="250" text-anchor="middle">
                "CertificateVerify · Finished"
            </text>
            <line class="gfx-arrow" x1="650" y1="258" x2="110" y2="258" />
            <text class="gfx-text" x="380" y="284" text-anchor="middle">
                "Finished"
            </text>
            <line class="gfx-arrow" x1="110" y1="292" x2="650" y2="292" />
            <line class="gfx-arrow gfx-dash" x1="110" y1="330" x2="650" y2="330" />
            <line class="gfx-arrow gfx-dash" x1="650" y1="352" x2="110" y2="352" />
            <text class="gfx-note" x="380" y="322" text-anchor="middle">
                "application data"
            </text>
            <g class="gfx-decide">
                <rect class="gfx-badge" x="160" y="392" width="440" height="46" rx="6" />
                <text class="gfx-badge-text" x="380" y="412" text-anchor="middle">
                    "KEYS FROM ss_ML-KEM ‖ ss_X25519"
                </text>
                <text class="gfx-note" x="380" y="430" text-anchor="middle">
                    "the session holds if either component holds"
                </text>
            </g>
        </svg>
    }
    .into_any()
}

/// Qxet: `FastCDC` content-defined chunking at the store's configured
///
/// parameters (16 KiB min / 64 KiB avg / 256 KiB max) — a rolling
/// Gear hash picks the cut points, chunks are stored once under
/// their SHA-256 in a 2-hex fan-out, and a canonical-JSON manifest
/// records the ordered chunk list under the artifact's content id.
fn qxet_cdc() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 468"
            role="img"
            aria-label="Diagram: an artifact's byte stream is scanned by a rolling Gear hash; cut points follow FastCDC's normalized masks between a 16 KiB minimum and a 256 KiB maximum; chunks are hashed with SHA-256, stored once in a fan-out tree, and listed in order by a manifest under the content id."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "ARTIFACT BYTES — ONE SCANNING PASS"
            </text>
            <rect class="gfx-hot" x="118" y="36" width="96" height="44" rx="4" />
            <text class="gfx-note" x="166" y="30" text-anchor="middle">
                "rolling window"
            </text>
            <rect class="gfx-panel" x="8" y="42" width="744" height="32" rx="4" />
            <line class="gfx-arrow" x1="302" y1="42" x2="302" y2="74" />
            <line class="gfx-arrow" x1="518" y1="42" x2="518" y2="74" />
            <line class="gfx-arrow" x1="688" y1="42" x2="688" y2="74" />
            <text class="gfx-note" x="12" y="94">
                "MIN 16 KiB — cuts suppressed"
            </text>
            <text class="gfx-note" x="380" y="94" text-anchor="middle">
                "AVG 64 KiB — strict mask before, lenient after"
            </text>
            <text class="gfx-note" x="748" y="94" text-anchor="end">
                "MAX 256 KiB — cut forced"
            </text>
            <text class="gfx-text" x="380" y="122" text-anchor="middle">
                "per byte: h ← (h ≪ 1) + G[b] · cut when (h & mask) = 0"
            </text>
            <line class="gfx-arrow" x1="150" y1="130" x2="150" y2="152" />
            <line class="gfx-arrow" x1="410" y1="130" x2="410" y2="152" />
            <line class="gfx-arrow" x1="610" y1="130" x2="610" y2="152" />
            <circle class="gfx-pulse-y" cx="150" cy="134" r="3.5" />
            <rect class="gfx-panel" x="30" y="154" width="220" height="42" rx="6" />
            <text class="gfx-text" x="140" y="180" text-anchor="middle">
                "chunk 1"
            </text>
            <rect class="gfx-panel" x="286" y="154" width="248" height="42" rx="6" />
            <text class="gfx-text" x="410" y="180" text-anchor="middle">
                "chunk 2"
            </text>
            <rect class="gfx-panel" x="570" y="154" width="160" height="42" rx="6" />
            <text class="gfx-text" x="650" y="180" text-anchor="middle">
                "chunk 3"
            </text>
            <text class="gfx-note" x="380" y="216" text-anchor="middle">
                "chunk sizes vary with content — boundaries move with the bytes, not the offsets"
            </text>
            <line class="gfx-arrow" x1="140" y1="222" x2="140" y2="244" />
            <line class="gfx-arrow" x1="410" y1="222" x2="410" y2="244" />
            <line class="gfx-arrow" x1="650" y1="222" x2="650" y2="244" />
            <circle
                class="gfx-pulse-y"
                cx="410"
                cy="226"
                r="3.5"
                style="animation-delay:0.7s"
            />
            <rect class="gfx-chip" x="40" y="246" width="200" height="36" rx="6" />
            <text class="gfx-text" x="140" y="269" text-anchor="middle">
                "SHA-256(chunk)"
            </text>
            <rect class="gfx-chip" x="280" y="246" width="260" height="36" rx="6" />
            <text class="gfx-text" x="410" y="269" text-anchor="middle">
                "SHA-256(chunk)"
            </text>
            <rect class="gfx-chip" x="580" y="246" width="150" height="36" rx="6" />
            <text class="gfx-text" x="655" y="269" text-anchor="middle">
                "SHA-256(chunk)"
            </text>
            <line class="gfx-arrow" x1="240" y1="300" x2="240" y2="318" />
            <line class="gfx-arrow" x1="540" y1="300" x2="540" y2="318" />
            <rect class="gfx-panel" x="8" y="320" width="470" height="62" rx="6" />
            <text class="gfx-head" x="243" y="344" text-anchor="middle">
                "CHUNK STORE — .QXET/CHUNKS/AB/CDEF…"
            </text>
            <text class="gfx-note" x="243" y="366" text-anchor="middle">
                "2-hex fan-out · an identical chunk is stored once, across all artifacts"
            </text>
            <rect class="gfx-panel" x="494" y="320" width="258" height="62" rx="6" />
            <text class="gfx-head" x="623" y="344" text-anchor="middle">
                "MANIFEST"
            </text>
            <text class="gfx-note" x="623" y="366" text-anchor="middle">
                "canonical JSON · ordered chunk ids"
            </text>
            <rect class="gfx-badge" x="130" y="398" width="500" height="40" rx="6" />
            <text class="gfx-badge-text" x="380" y="423" text-anchor="middle">
                "CONTENT ID qxet:<sha256> · READS RE-VERIFY EVERY HASH"
            </text>
            <text class="gfx-note" x="8" y="456">
                "verify — rehash every chunk"
            </text>
            <text class="gfx-note" x="752" y="456" text-anchor="end">
                "gc — mark & sweep · dry-run default"
            </text>
        </svg>
    }
    .into_any()
}

/// Light Show: the link budget the game scores every route by —
/// launch power minus each placed element's loss, landing (or not)
/// inside the GPON receive window. Loss figures are the simulation's
///
/// own constants (`osp_sim`); the 10 km span is an illustrative length.
fn lightshow_budget() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 336"
            role="img"
            aria-label="Diagram: a worked optical link budget — OLT launch at +3.0 dBm, minus a UPC connector, a fusion splice, 10 km of fiber at 1490 nm, a 1x8 splitter, and an APC connector, arriving at the ONT at −11.1 dBm, inside the GPON receive window of −8 to −27 dBm."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "ONE ROUTE, SCORED IN DECIBELS"
            </text>
            <rect class="gfx-hot" x="4" y="48" width="100" height="54" rx="6" />
            <text class="gfx-text" x="54" y="72" text-anchor="middle">
                "OLT"
            </text>
            <text class="gfx-note" x="54" y="90" text-anchor="middle">
                "+3.0 dBm"
            </text>
            <line class="gfx-arrow" x1="106" y1="75" x2="122" y2="75" />
            <rect class="gfx-chip" x="124" y="48" width="100" height="54" rx="6" />
            <text class="gfx-text" x="174" y="72" text-anchor="middle">
                "CONN. UPC"
            </text>
            <text class="gfx-note" x="174" y="90" text-anchor="middle">
                "−0.35 dB"
            </text>
            <line class="gfx-arrow" x1="226" y1="75" x2="242" y2="75" />
            <rect class="gfx-chip" x="244" y="48" width="100" height="54" rx="6" />
            <text class="gfx-text" x="294" y="72" text-anchor="middle">
                "FUSION SPLICE"
            </text>
            <text class="gfx-note" x="294" y="90" text-anchor="middle">
                "−0.075 dB"
            </text>
            <line class="gfx-arrow" x1="346" y1="75" x2="362" y2="75" />
            <circle class="gfx-pulse" cx="350" cy="75" r="3.5" />
            <rect class="gfx-chip" x="364" y="48" width="104" height="54" rx="6" />
            <text class="gfx-text" x="416" y="72" text-anchor="middle">
                "FIBER 10 KM"
            </text>
            <text class="gfx-note" x="416" y="90" text-anchor="middle">
                "−2.8 dB · 1490 nm"
            </text>
            <line class="gfx-arrow" x1="470" y1="75" x2="486" y2="75" />
            <rect class="gfx-chip" x="488" y="48" width="100" height="54" rx="6" />
            <text class="gfx-text" x="538" y="72" text-anchor="middle">
                "SPLITTER 1×8"
            </text>
            <text class="gfx-note" x="538" y="90" text-anchor="middle">
                "−10.6 dB"
            </text>
            <line class="gfx-arrow" x1="590" y1="75" x2="606" y2="75" />
            <rect class="gfx-chip" x="608" y="48" width="72" height="54" rx="6" />
            <text class="gfx-text" x="644" y="72" text-anchor="middle">
                "APC"
            </text>
            <text class="gfx-note" x="644" y="90" text-anchor="middle">
                "−0.30 dB"
            </text>
            <line class="gfx-arrow" x1="682" y1="75" x2="698" y2="75" />
            <rect class="gfx-hot" x="700" y="48" width="56" height="54" rx="6" />
            <text class="gfx-text" x="728" y="72" text-anchor="middle">
                "ONT"
            </text>
            <text class="gfx-note" x="728" y="90" text-anchor="middle">
                "rx"
            </text>
            <text class="gfx-note" x="114" y="122" text-anchor="middle">
                "2.65"
            </text>
            <text class="gfx-note" x="234" y="122" text-anchor="middle">
                "2.58"
            </text>
            <text class="gfx-note" x="354" y="122" text-anchor="middle">
                "−0.23"
            </text>
            <text class="gfx-note" x="478" y="122" text-anchor="middle">
                "−10.83"
            </text>
            <text class="gfx-note" x="598" y="122" text-anchor="middle">
                "−11.13 dBm"
            </text>
            <text class="gfx-head" x="380" y="158" text-anchor="middle">
                "GPON RECEIVE WINDOW (ITU-T G.984) — TOO HOT FAILS TOO"
            </text>
            <rect class="gfx-panel" x="60" y="176" width="640" height="26" rx="4" />
            <rect class="gfx-fill-brass" x="298" y="176" width="348" height="26" />
            <line class="gfx-arrow" x1="355" y1="168" x2="355" y2="210" />
            <text class="gfx-text" x="60" y="222">
                "+5"
            </text>
            <text class="gfx-text" x="298" y="222">
                "−8"
            </text>
            <text class="gfx-text" x="646" y="222" text-anchor="end">
                "−27"
            </text>
            <text class="gfx-text" x="700" y="222" text-anchor="end">
                "−30 dBm"
            </text>
            <text class="gfx-text" x="355" y="240" text-anchor="middle">
                "this route arrives at −11.1 dBm — inside the window"
            </text>
            <text class="gfx-note" x="380" y="276" text-anchor="middle">
                "swap in a mechanical splice (−0.4 dB) or a 1×32 splitter (−17.7 dB)"
            </text>
            <text class="gfx-note" x="380" y="296" text-anchor="middle">
                "and the same route misses — the ledger recomputes on every placement"
            </text>
            <text class="gfx-note" x="380" y="324" text-anchor="middle">
                "attenuation: 0.35 dB/km at 1310 nm · 0.28 at 1490 nm · 0.21 at 1550 nm"
            </text>
        </svg>
    }
    .into_any()
}
