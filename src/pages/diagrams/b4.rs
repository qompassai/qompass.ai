//! Batch 4 concept diagrams for solution pages. Owned ids are listed
//!
//! in `IDS`; the registry in `diagrams.rs` delegates here. Follow
//! the authoring contract in that module's header: `gfx-*` classes
//! only, text at 10.5px or larger at viewBox scale, motion only via
//! the stylesheet's reduced-motion-gated classes, and every figure
//! must read correctly as a static diagram.

use leptos::prelude::*;

/// The diagram ids this module owns.
pub const IDS: &[&str] = &["azimuth-bus", "qjp-stack", "qluv-store", "qompass-hold-map", "qonfig-pipeline"];

/// The bare `<svg>` for one of this module's ids, or `None`.
pub fn diagram(id: &'static str) -> Option<AnyView> {
    match id {
        "azimuth-bus" => Some(azimuth_bus()),
        "qjp-stack" => Some(qjp_stack()),
        "qluv-store" => Some(qluv_store()),
        "qompass-hold-map" => Some(qompass_hold_map()),
        "qonfig-pipeline" => Some(qonfig_pipeline()),
        _ => None,
    }
}

/// Azimuth: the shipped Discord path end to end — the bot publishes
///
/// each non-bot message to `JetStream` subject `msg.in.discord`, the
/// LLM worker (subscribed to `msg.in.*`) completes it against the
/// local rose server, and the response returns on `msg.out.discord`
/// to the originating channel. Subjects and services are as named in
///
/// `cmd/bot` and `cmd/llmworker`; Postgres and the observability
/// stack are architecture-diagram elements, labelled as such.
fn azimuth_bus() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 504"
            role="img"
            aria-label="Diagram: a Discord message travels from the bot service to NATS JetStream subject msg.in.discord, to the LLM worker, to the rose LLM server, and back as a response on msg.out.discord which the bot posts to the originating channel."
        >
            <text class="gfx-head" x="380" y="26" text-anchor="middle">
                "DISCORD MESSAGE → JETSTREAM → WORKER → ROSE → REPLY"
            </text>
            <rect class="gfx-panel" x="170" y="48" width="420" height="52" rx="6" />
            <text class="gfx-head" x="380" y="72" text-anchor="middle">
                "DISCORD CHANNEL"
            </text>
            <text class="gfx-note" x="380" y="90" text-anchor="middle">
                "a user posts — bot authors are ignored"
            </text>
            <line class="gfx-arrow" x1="380" y1="100" x2="380" y2="122" />
            <circle class="gfx-pulse-y" cx="380" cy="104" r="3.5" />
            <text class="gfx-note" x="396" y="118">
                "bot publishes model.Message"
            </text>
            <rect class="gfx-hot" x="170" y="124" width="420" height="36" rx="6" />
            <text class="gfx-text" x="380" y="147" text-anchor="middle">
                "NATS JETSTREAM — msg.in.discord"
            </text>
            <line class="gfx-arrow" x1="380" y1="160" x2="380" y2="182" />
            <text class="gfx-note" x="396" y="176">
                "worker subscribes to msg.in.*"
            </text>
            <rect class="gfx-panel" x="170" y="184" width="420" height="52" rx="6" />
            <text class="gfx-head" x="380" y="208" text-anchor="middle">
                "LLM WORKER — cmd/llmworker"
            </text>
            <text class="gfx-note" x="380" y="226" text-anchor="middle">
                "unmarshals the message · calls rose"
            </text>
            <line class="gfx-arrow" x1="380" y1="236" x2="380" y2="258" />
            <circle
                class="gfx-pulse-y"
                cx="380"
                cy="240"
                r="3.5"
                style="animation-delay:0.7s"
            />
            <rect class="gfx-panel" x="170" y="260" width="420" height="52" rx="6" />
            <text class="gfx-head" x="380" y="284" text-anchor="middle">
                "ROSE — LOCAL LLM SERVER"
            </text>
            <text class="gfx-note" x="380" y="302" text-anchor="middle">
                "RoseClient.Complete(content) · mTLS to ROSE_URL"
            </text>
            <line class="gfx-arrow" x1="380" y1="312" x2="380" y2="334" />
            <text class="gfx-note" x="396" y="328">
                "model.Response"
            </text>
            <rect class="gfx-hot" x="170" y="336" width="420" height="36" rx="6" />
            <text class="gfx-text" x="380" y="359" text-anchor="middle">
                "NATS JETSTREAM — msg.out.discord"
            </text>
            <line class="gfx-arrow" x1="380" y1="372" x2="380" y2="394" />
            <circle
                class="gfx-pulse-y"
                cx="380"
                cy="376"
                r="3.5"
                style="animation-delay:1.4s"
            />
            <rect class="gfx-panel" x="170" y="396" width="420" height="52" rx="6" />
            <text class="gfx-head" x="380" y="420" text-anchor="middle">
                "BOT SERVICE — cmd/bot"
            </text>
            <text class="gfx-note" x="380" y="438" text-anchor="middle">
                "ChannelMessageSend — reply to the originating channel"
            </text>
            <text class="gfx-note" x="380" y="472" text-anchor="middle">
                "compose services: nats · postgres · rose · bot — Postgres storage and"
            </text>
            <text class="gfx-note" x="380" y="490" text-anchor="middle">
                "Prometheus / Grafana / Loki observability are in the architecture diagram"
            </text>
        </svg>
    }
    .into_any()
}

/// Qjp: the provisioning stack — `JetPack` SDK components packaged by
///
/// the Nix flake, flashed to a device in recovery mode, booting
/// through EDK2 UEFI firmware into NVIDIA's 5.10 kernel with nvgpu,
/// with the CUDA userland on top. Layer names are the project's own
///
/// (README, flake.nix, modules/default.nix).
fn qjp_stack() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 464"
            role="img"
            aria-label="Diagram: NVIDIA JetPack SDK components are packaged by Qjp's Nix flake, flashed to a Jetson in recovery mode, and boot through EDK2 UEFI firmware into the NVIDIA 5.10 kernel with nvgpu, with the CUDA and TensorRT userspace on top."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "FROM JETPACK SDK TO A RUNNING JETSON"
            </text>
            <rect class="gfx-panel" x="130" y="42" width="500" height="48" rx="6" />
            <text class="gfx-text" x="380" y="64" text-anchor="middle">
                "NVIDIA JETPACK SDK COMPONENTS"
            </text>
            <text class="gfx-note" x="380" y="80" text-anchor="middle">
                "JetPack 5.1 — firmware, kernel, CUDA stack (NVIDIA)"
            </text>
            <line class="gfx-arrow" x1="380" y1="90" x2="380" y2="108" />
            <circle class="gfx-pulse-y" cx="380" cy="94" r="3.5" />
            <rect class="gfx-hot" x="130" y="110" width="500" height="48" rx="6" />
            <text class="gfx-text" x="380" y="132" text-anchor="middle">
                "QJP NIX PACKAGING"
            </text>
            <text class="gfx-note" x="380" y="148" text-anchor="middle">
                "flake outputs: flash-* · iso_minimal · uefi-capsule-update-*"
            </text>
            <line class="gfx-arrow" x1="380" y1="158" x2="380" y2="176" />
            <rect class="gfx-panel" x="130" y="178" width="500" height="48" rx="6" />
            <text class="gfx-text" x="380" y="200" text-anchor="middle">
                "FLASH TO DEVICE"
            </text>
            <text class="gfx-note" x="380" y="216" text-anchor="middle">
                "x86_64 host · device in recovery mode (APX over USB)"
            </text>
            <line class="gfx-arrow" x1="380" y1="226" x2="380" y2="244" />
            <circle
                class="gfx-pulse-y"
                cx="380"
                cy="230"
                r="3.5"
                style="animation-delay:0.7s"
            />
            <rect class="gfx-panel" x="130" y="246" width="500" height="48" rx="6" />
            <text class="gfx-text" x="380" y="268" text-anchor="middle">
                "EDK2 UEFI FIRMWARE"
            </text>
            <text class="gfx-note" x="380" y="284" text-anchor="middle">
                "ARM Trusted Firmware · OP-TEE alongside"
            </text>
            <line class="gfx-arrow" x1="380" y1="294" x2="380" y2="312" />
            <rect class="gfx-panel" x="130" y="314" width="500" height="48" rx="6" />
            <text class="gfx-text" x="380" y="336" text-anchor="middle">
                "NVIDIA 5.10 KERNEL"
            </text>
            <text class="gfx-note" x="380" y="352" text-anchor="middle">
                "nvgpu driver"
            </text>
            <line class="gfx-arrow" x1="380" y1="362" x2="380" y2="380" />
            <circle
                class="gfx-pulse-y"
                cx="380"
                cy="366"
                r="3.5"
                style="animation-delay:1.4s"
            />
            <rect class="gfx-panel" x="130" y="382" width="500" height="48" rx="6" />
            <text class="gfx-text" x="380" y="404" text-anchor="middle">
                "CUDA USERSPACE"
            </text>
            <text class="gfx-note" x="380" y="420" text-anchor="middle">
                "CUDA · CuDNN · TensorRT · V4L2/gstreamer · Wayland/GBM/EGL/Vulkan"
            </text>
            <text class="gfx-note" x="380" y="452" text-anchor="middle">
                "target set by hardware.nvidia-jetpack.som + carrierBoard — Orin AGX/NX, Xavier AGX/NX"
            </text>
        </svg>
    }
    .into_any()
}

/// `QLuV`: side-by-side installs under `~/.qluv` and the selection
///
/// mechanism the `qluv` script actually implements — `use` rewrites
/// PATH (previous qluv bin entries removed, the selected version's
/// bin prepended) and `set-default` persists the choice in a
/// `DEFAULT_*` file that `__qluv_init` re-applies on shell load.
///
/// There is no shim; selection is PATH order.
fn qluv_store() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 392"
            role="img"
            aria-label="Diagram: Lua, LuaJIT and LuaRocks versions installed side by side under ~/.qluv; qluv use rewrites PATH so the selected version's bin directory comes first, and set-default records the choice in a DEFAULT file re-applied at shell load."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "SIDE-BY-SIDE VERSIONS UNDER ~/.qluv"
            </text>
            <rect class="gfx-panel" x="8" y="42" width="336" height="236" rx="6" />
            <text class="gfx-head" x="176" y="66" text-anchor="middle">
                "~/.qluv"
            </text>
            <rect class="gfx-chip" x="24" y="78" width="304" height="30" rx="6" />
            <text class="gfx-text" x="40" y="98">
                "lua/ — one directory per version"
            </text>
            <rect class="gfx-chip" x="24" y="116" width="304" height="30" rx="6" />
            <text class="gfx-text" x="40" y="136">
                "luajit/ — one directory per version"
            </text>
            <rect class="gfx-chip" x="24" y="154" width="304" height="30" rx="6" />
            <text class="gfx-text" x="40" y="174">
                "luarocks/ — one directory per version"
            </text>
            <rect class="gfx-chip" x="24" y="192" width="304" height="30" rx="6" />
            <text class="gfx-text" x="40" y="212">
                "src/ — downloaded tarballs"
            </text>
            <text class="gfx-note" x="24" y="246">
                "DEFAULT_LUA · DEFAULT_LUAJIT"
            </text>
            <text class="gfx-note" x="24" y="264">
                "DEFAULT_LUAROCKS — saved defaults"
            </text>
            <rect class="gfx-panel" x="368" y="42" width="384" height="64" rx="6" />
            <text class="gfx-head" x="560" y="66" text-anchor="middle">
                "qluv use <version>"
            </text>
            <text class="gfx-note" x="560" y="88" text-anchor="middle">
                "switch the active version"
            </text>
            <line class="gfx-arrow" x1="560" y1="106" x2="560" y2="128" />
            <circle class="gfx-pulse-y" cx="560" cy="110" r="3.5" />
            <rect class="gfx-panel" x="368" y="130" width="384" height="72" rx="6" />
            <text class="gfx-text" x="560" y="154" text-anchor="middle">
                "PATH IS REWRITTEN"
            </text>
            <text class="gfx-note" x="560" y="174" text-anchor="middle">
                "previous ~/.qluv bin entries removed,"
            </text>
            <text class="gfx-note" x="560" y="190" text-anchor="middle">
                "selected version's bin/ prepended"
            </text>
            <line class="gfx-arrow" x1="560" y1="202" x2="560" y2="224" />
            <rect class="gfx-hot" x="368" y="226" width="384" height="52" rx="6" />
            <text class="gfx-text" x="560" y="250" text-anchor="middle">
                "lua RESOLVES TO THE SELECTED INSTALL"
            </text>
            <text class="gfx-note" x="560" y="268" text-anchor="middle">
                "whichever bin/ sits first on PATH"
            </text>
            <text class="gfx-note" x="380" y="306" text-anchor="middle">
                "install: download → compile → PREFIX-install under ~/.qluv/lua/<version> (likewise luajit/ and luarocks/)"
            </text>
            <text class="gfx-note" x="380" y="328" text-anchor="middle">
                "set-default writes the DEFAULT_* file — __qluv_init re-applies it on the next shell load"
            </text>
            <rect class="gfx-badge" x="100" y="342" width="560" height="38" rx="6" />
            <text class="gfx-badge-text" x="380" y="366" text-anchor="middle">
                "EVERY VERSION STAYS INSTALLED — SWITCHING ONLY REORDERS PATH"
            </text>
        </svg>
    }
    .into_any()
}

/// Qompass-Hold: the mapping the service implements — Secret Service
/// D-Bus objects backed by the `pass` store. Collections map to the
///
/// `secret-service` subdirectory, items are GPG-encrypted `.gpg`
/// files, and attribute search resolves through `attributes.redb`
/// instead of decrypting every file. The diagram shows the mapping
/// mechanism only — no secret names or values.
fn qompass_hold_map() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 416"
            role="img"
            aria-label="Diagram: an application talks to Qompass-Hold over the Secret Service D-Bus API under the well-known name org.freedesktop.secrets; collections map to a subdirectory of the pass store, items are GPG-encrypted files, and attribute search uses the attributes.redb index."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "SECRET SERVICE CALLS, BACKED BY pass"
            </text>
            <rect class="gfx-panel" x="8" y="44" width="200" height="64" rx="6" />
            <text class="gfx-head" x="108" y="68" text-anchor="middle">
                "APPLICATION"
            </text>
            <text class="gfx-note" x="108" y="88" text-anchor="middle">
                "libsecret / desktop client"
            </text>
            <line class="gfx-arrow" x1="208" y1="76" x2="244" y2="76" />
            <circle class="gfx-pulse" cx="212" cy="76" r="3.5" />
            <rect class="gfx-hot" x="248" y="44" width="264" height="64" rx="6" />
            <text class="gfx-head" x="380" y="68" text-anchor="middle">
                "QOMPASS-HOLD"
            </text>
            <text class="gfx-note" x="380" y="88" text-anchor="middle">
                "Rust secret service"
            </text>
            <line class="gfx-arrow" x1="512" y1="76" x2="548" y2="76" />
            <rect class="gfx-panel" x="552" y="44" width="200" height="64" rx="6" />
            <text class="gfx-head" x="652" y="68" text-anchor="middle">
                "WELL-KNOWN NAME"
            </text>
            <text class="gfx-note" x="652" y="88" text-anchor="middle">
                "org.freedesktop.secrets"
            </text>
            <line class="gfx-arrow" x1="380" y1="108" x2="380" y2="138" />
            <circle class="gfx-pulse-y" cx="380" cy="112" r="3.5" />
            <rect class="gfx-panel" x="148" y="140" width="464" height="76" rx="6" />
            <text class="gfx-head" x="380" y="164" text-anchor="middle">
                "SECRET SERVICE API — /org/freedesktop/secrets"
            </text>
            <text class="gfx-text" x="380" y="188" text-anchor="middle">
                "Service · Session · Collection · Item"
            </text>
            <text class="gfx-note" x="380" y="204" text-anchor="middle">
                "session objects encrypt/decrypt values in transit"
            </text>
            <line class="gfx-arrow" x1="380" y1="216" x2="380" y2="238" />
            <circle
                class="gfx-pulse-y"
                cx="380"
                cy="220"
                r="3.5"
                style="animation-delay:0.7s"
            />
            <rect class="gfx-panel" x="68" y="240" width="624" height="118" rx="6" />
            <text class="gfx-head" x="380" y="264" text-anchor="middle">
                "~/.password-store — THE pass STORE"
            </text>
            <text class="gfx-text" x="88" y="292">
                "collections map to the secret-service subdirectory of the store"
            </text>
            <text class="gfx-text" x="88" y="316">
                "items are .gpg files in the pass layout — reads run gpg --decrypt"
            </text>
            <text class="gfx-text" x="88" y="340">
                "search resolves attributes in attributes.redb (redb, in the store)"
            </text>
            <text class="gfx-note" x="380" y="382" text-anchor="middle">
                "unlocking, locking and deletion run through the same D-Bus interfaces"
            </text>
            <text class="gfx-note" x="380" y="402" text-anchor="middle">
                "PASSWORD_STORE_DIR overrides the store location (default ~/.password-store)"
            </text>
        </svg>
    }
    .into_any()
}

/// Qonfig: the directive pipeline — `qonfig.yaml` is a list of tasks
/// the dispatcher hands to its plugins in order (defaults, clean,
/// link, shell). The `create` directive used in some configs has no
///
/// plugin in the shipped tree and is logged as unhandled, so it is
/// flagged below the figure rather than drawn as an applied step.
fn qonfig_pipeline() -> AnyView {
    view! {
        <svg
            class="gfx"
            viewBox="0 0 760 400"
            role="img"
            aria-label="Diagram: qonfig.yaml's list of tasks is dispatched in order to the defaults, clean, link and shell plugins; a run ends with all tasks executed successfully, or it fails."
        >
            <text class="gfx-head" x="380" y="24" text-anchor="middle">
                "QONFIG.YAML — TASKS DISPATCHED IN ORDER"
            </text>
            <rect class="gfx-panel" x="190" y="42" width="380" height="52" rx="6" />
            <text class="gfx-head" x="380" y="66" text-anchor="middle">
                "qonfig.yaml"
            </text>
            <text class="gfx-note" x="380" y="84" text-anchor="middle">
                "a list of tasks (YAML, or JSON/JSONC)"
            </text>
            <line class="gfx-arrow" x1="380" y1="94" x2="380" y2="116" />
            <circle class="gfx-pulse-y" cx="380" cy="98" r="3.5" />
            <text class="gfx-note" x="396" y="110">
                "via ./install — bin/qonfig -d BASEDIR -c qonfig.yaml"
            </text>
            <rect class="gfx-chip" x="190" y="118" width="380" height="44" rx="6" />
            <text class="gfx-text" x="380" y="146" text-anchor="middle">
                "DISPATCHER — each task to its plugin, in order"
            </text>
            <line class="gfx-arrow" x1="380" y1="162" x2="380" y2="184" />
            <rect class="gfx-panel" x="8" y="186" width="178" height="104" rx="6" />
            <text class="gfx-head" x="97" y="210" text-anchor="middle">
                "defaults"
            </text>
            <text class="gfx-note" x="97" y="234" text-anchor="middle">
                "options applied to"
            </text>
            <text class="gfx-note" x="97" y="252" text-anchor="middle">
                "the tasks that follow"
            </text>
            <rect class="gfx-panel" x="196" y="186" width="178" height="104" rx="6" />
            <text class="gfx-head" x="285" y="210" text-anchor="middle">
                "clean"
            </text>
            <text class="gfx-note" x="285" y="234" text-anchor="middle">
                "remove dead symlinks"
            </text>
            <text class="gfx-note" x="285" y="252" text-anchor="middle">
                "pointing into the"
            </text>
            <text class="gfx-note" x="285" y="270" text-anchor="middle">
                "dotfiles directory"
            </text>
            <rect class="gfx-hot" x="384" y="186" width="178" height="104" rx="6" />
            <text class="gfx-head" x="473" y="210" text-anchor="middle">
                "link"
            </text>
            <text class="gfx-note" x="473" y="234" text-anchor="middle">
                "symlink files into place"
            </text>
            <text class="gfx-note" x="473" y="252" text-anchor="middle">
                "create · relink · force"
            </text>
            <text class="gfx-note" x="473" y="270" text-anchor="middle">
                "relative options"
            </text>
            <rect class="gfx-panel" x="572" y="186" width="178" height="104" rx="6" />
            <text class="gfx-head" x="661" y="210" text-anchor="middle">
                "shell"
            </text>
            <text class="gfx-note" x="661" y="234" text-anchor="middle">
                "run shell commands"
            </text>
            <text class="gfx-note" x="661" y="252" text-anchor="middle">
                "optional description"
            </text>
            <text class="gfx-note" x="661" y="270" text-anchor="middle">
                "and stdio toggles"
            </text>
            <rect class="gfx-badge" x="130" y="308" width="500" height="38" rx="6" />
            <text class="gfx-badge-text" x="380" y="332" text-anchor="middle">
                "'ALL TASKS EXECUTED SUCCESSFULLY' — OR IT FAILS"
            </text>
            <text class="gfx-note" x="380" y="368" text-anchor="middle">
                "some configs also use a create directive — the shipped plugins are clean, link and shell;"
            </text>
            <text class="gfx-note" x="380" y="386" text-anchor="middle">
                "the dispatcher logs any unhandled action instead of applying it"
            </text>
        </svg>
    }
    .into_any()
}
