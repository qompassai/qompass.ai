//! Light Show — the game's home on this site (`/light-show`) and its
//! tester-recruitment page (`/light-show/testers`).
//!
//! The home renders the Google Play listing text verbatim (the
//! fastlane metadata in the light-show repo is the source), the
//! listing's phone screenshots, and one row per store channel with
//! the channel's honest current state. A channel row becomes a live
//! link only when that channel is actually open to everyone; until
//! then it renders in the site's not-yet-live treatment, the same
//! convention the download boxes use for unpublished platforms.
//!
//! The testers page explains the closed-test gate a personal
//! developer account lives under, what a tester does, and a
//! play-by-play of the real Google opt-in flow. Each play-by-play
//! step carries a screenshot slot: the slot renders a schematic
//! placeholder until a genuine capture of that screen exists (the
//! first walk-through on a real phone supplies them), at which point
//! dropping the image path into `Step::shot` is the whole change —
//! no markup or layout moves.
use leptos::prelude::*;
use leptos_meta::Title;

/// The app's Google Play listing address.
///
/// Prepared for the day the production listing opens. While the game
/// is in closed testing the badge below renders inert; when production
/// goes live, wrap the badge in an anchor to this URL and drop the
/// allow.
#[allow(dead_code)]
const PLAY_STORE_URL: &str = "https://play.google.com/store/apps/details?id=ai.qompass.lightshow";

/// The Play listing's feature bullets, verbatim from the listing
/// text.
const FEATURES: &[&str] = &[
    "Real link-budget math: fusion vs. mechanical splices, UPC vs. APC connectors, PON splitters, wavelength-dependent attenuation — all pulled from real OSP practice.",
    "Outage events: fiber cuts, storm damage, water intrusion, and macrobends force a live reroute under a countdown — diagnose and fix the link before the customer-complaint timer runs out, then see your result on a dedicated pass/fail debrief screen.",
    "Pick your companion before you start: Séraphine (Fiber), Ondine (Coax), Linka (Mobile), or Lattice (Ethernet) — each one is tied to a real access technology, has her own dialogue, and reacts in her own voice to every splice, outage, and level result.",
    "An original 8-bit-inspired chiptune soundtrack scores the menu, splicing, and outage states, with its own win/fail jingles.",
    "64x64 pixel-art visuals with an anime-inspired look, including on-board icons for every component you place.",
    "No ads. No in-app purchases. No tracking. No network permission at all — this game works completely offline and always will.",
];

/// The Play listing's phone screenshots, web-sized copies under
/// `public/img/light-show/` (the listing originals stay in the
/// light-show repo's fastlane tree).
const SCREENSHOTS: &[(&str, &str)] = &[
    ("/img/light-show/ls_phone_01_menu.webp", "Title menu"),
    ("/img/light-show/ls_phone_02_gameplay.webp", "Gameplay"),
    ("/img/light-show/ls_phone_03_credits.webp", "Credits"),
    ("/img/light-show/ls_phone_04_level_select.webp", "Level select"),
    ("/img/light-show/ls_phone_05_gameplay_world1.webp", "Gameplay — world 1"),
    ("/img/light-show/ls_phone_06_gameplay_world2.webp", "Gameplay — world 2"),
    ("/img/light-show/ls_phone_07_result.webp", "Result debrief"),
    ("/img/light-show/ls_phone_08_warehouse.webp", "Warehouse"),
];

/// Companion portraits: `(image, name, role)`.
///
/// Web-sized copies of the light-show repo's
/// `assets/art/<name>/companions/<name>_portrait.jpg` at 9c21ca1; each
/// role is the Role column of `docs/characters/README.md`. Clara, Aino,
/// and Hikari are left out until the repo settles which of their
/// generations is canonical (their character pages flag it).
const PORTRAITS: &[(&str, &str, &str)] = &[
    ("/img/light-show/portraits/seraphine.webp", "Séraphine", "Fiber-optic OSP splicing"),
    ("/img/light-show/portraits/ondine.webp", "Ondine", "Coax / broadband RF"),
    ("/img/light-show/portraits/linka.webp", "Linka", "Mobile / cellular RF"),
    ("/img/light-show/portraits/lattice.webp", "Lattice", "Ethernet / copper LAN"),
    ("/img/light-show/portraits/lea.webp", "Léa", "WA 09 Telecom Admin prep"),
    ("/img/light-show/portraits/bianca.webp", "Bianca", "Warehouse host — test bench"),
    ("/img/light-show/portraits/tessa.webp", "Tessa", "Warehouse host — field"),
];

/// One soundtrack entry: `(title, artist, length)`.
type Track = (&'static str, &'static str, &'static str);

/// The game's music, grouped by the game state that plays it.
///
/// Groups run in the order a run meets them. Source of truth is the
/// light-show repo's `game/src/audio.rs` at 9c21ca1 (track per state,
/// tier rotation order, the companion-theme order); titles and artists
/// come from `docs/CREDITS.md`, lengths from ffprobe. The per-companion
/// ambience hums are excluded: the game layers them under the music on
/// their own channel, not as music tracks.
const SOUNDTRACK: &[(&str, &[Track])] = &[
    ("TITLE MENU AND CREDITS", &[("We're all under the stars", "Eric Skiff", "4:33")]),
    (
        "COMPANION SELECT — EACH COMPANION'S THEME",
        &[
            ("Séraphine", "companion theme", "0:17"),
            ("Ondine", "companion theme", "0:16"),
            ("Linka", "companion theme", "0:14"),
            ("Lattice", "companion theme", "0:17"),
            ("Clara", "companion theme", "0:15"),
            ("Aino", "companion theme", "0:18"),
            ("Hikari", "companion theme", "0:15"),
            ("Léa", "companion theme", "0:20"),
        ],
    ),
    ("WORLD 1 — SPLICE SCHOOL", &[("Intensive puzzle resolution", "Komiku", "1:34")]),
    ("WORLD 2", &[("Searching", "Eric Skiff", "2:20"), ("Ascending", "Eric Skiff", "3:13")]),
    (
        "WORLD 3",
        &[("Chibi Ninja", "Eric Skiff", "2:03"), ("Underclocked (underunderclocked mix)", "Eric Skiff", "3:09")],
    ),
    (
        "WORLD 4 AND BEYOND",
        &[
            ("Exhilarate", "Kevin MacLeod", "2:25"),
            ("Action2 - Army Approaching", "Juhani Junkala / SubspaceAudio", "2:04"),
        ],
    ),
    (
        "OUTAGE",
        &[
            ("Basic Metal 5", "TeknoAXE", "3:27"),
            ("Action3 - Preparing For Battle", "Juhani Junkala / SubspaceAudio", "0:55"),
        ],
    ),
    ("RESULTS", &[("We're the Resistors", "Eric Skiff", "2:22"), ("In a Heartbeat", "Kevin MacLeod", "3:38")]),
];

/// The music attribution, as `docs/CREDITS.md` in the light-show repo
/// states it: artist, source, license, and the required credit text
/// where the artist asks for one.
const MUSIC_CREDITS: &[&str] = &[
    "Eric Skiff — Resistor Anthems (http://ericskiff.com/music): \u{201c}We're all under the stars\u{201d}, \u{201c}Searching\u{201d}, \u{201c}Ascending\u{201d}, \u{201c}Chibi Ninja\u{201d}, \u{201c}Underclocked (underunderclocked mix)\u{201d}, \u{201c}We're the Resistors\u{201d} — CC-BY 4.0. Eric Skiff - [Song] - Resistor Anthems - Available at http://EricSkiff.com/music",
    "Kevin MacLeod (https://incompetech.com/music/royalty-free/): \u{201c}Exhilarate\u{201d} — CC-BY 4.0; \u{201c}In a Heartbeat\u{201d} — CC-BY 3.0.",
    "TeknoAXE: \u{201c}Basic Metal 5\u{201d} — CC-BY 4.0. Basic Metal 5 by TeknoAXE | http://teknoaxe.com — Royalty Free Music by https://www.free-stock-music.com — CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/",
    "Juhani Junkala / SubspaceAudio — JRPG Pack 5: Action (https://opengameart.org/content/jrpg-pack-5-action): \u{201c}Action2 - Army Approaching\u{201d}, \u{201c}Action3 - Preparing For Battle\u{201d} — CC0 (courtesy credit).",
    "Komiku (https://freemusicarchive.org/): \u{201c}Intensive puzzle resolution\u{201d} — CC0 1.0 Universal (courtesy credit).",
];

#[component]
pub fn LightShow() -> impl IntoView {
    view! {
        <Title text="Light Show: Fiber Optic Puzzle - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — LIGHT SHOW"</p>
            <article class="project-article">
                <header class="study-head">
                    <img
                        class="project-mark"
                        src="/img/projects/light-show.webp"
                        alt="Light Show"
                    />
                    <h1>"Light Show: Fiber Optic Puzzle"</h1>
                    <p class="page-lede">
                        "Route real fiber-optic light from A to B. Splice, split, survive outages"
                    </p>
                </header>
                <section class="study-section">
                    <p class="band-label">"THE GAME"</p>
                    <p>
                        "Light Show is a puzzle game built on real Outside Plant (OSP) fiber-optic "
                        "engineering. You're a field tech routing light from the OLT to the "
                        "customer's ONT — every splice, connector, and splitter you place is modeled "
                        "on real-world dB loss figures. Land the receive power inside the target "
                        "window, or the customer's link goes down."
                    </p>
                    <h3 class="study-subhead">"FEATURES"</h3>
                    <ul class="feature-list">
                        {FEATURES
                            .iter()
                            .map(|f| view! { <li>{*f}</li> })
                            .collect_view()}
                    </ul>
                    <p>
                        "This first release ships Splice School (your first fusion splice) and a "
                        "Storm Season outage drill, with more worlds — connector types, PON "
                        "split budgeting, and long-haul wavelength puzzles — in active "
                        "development for future updates."
                    </p>
                    <p>
                        "Whether you're a working telecom tech, a network engineering student, or "
                        "just enjoy a good logic puzzle, Light Show teaches real fiber-optic "
                        "concepts through play."
                    </p>
                </section>
                <section class="study-section">
                    <p class="band-label">"SCREENSHOTS"</p>
                    <div class="shot-gallery">
                        {SCREENSHOTS
                            .iter()
                            .map(|(src, caption)| {
                                view! {
                                    <figure class="shot">
                                        <img
                                            src={*src}
                                            alt={format!("Light Show — {caption}")}
                                            loading="lazy"
                                        />
                                        <figcaption>{*caption}</figcaption>
                                    </figure>
                                }
                            })
                            .collect_view()}
                    </div>
                </section>
                <section class="study-section">
                    <p class="band-label">"PORTRAITS"</p>
                    <div class="shot-gallery">
                        {PORTRAITS
                            .iter()
                            .map(|(src, name, role)| {
                                view! {
                                    <figure class="shot">
                                        <img src={*src} alt={*name} loading="lazy" />
                                        <figcaption>{*name}<br />{*role}</figcaption>
                                    </figure>
                                }
                            })
                            .collect_view()}
                    </div>
                    <p class="channel-note">"Portraits: AI-generated anime art."</p>
                </section>
                <section class="study-section">
                    <p class="band-label">"SOUNDTRACK"</p>
                    <p>
                        "Every music track the game ships, in the order a run meets them."
                    </p>
                    {SOUNDTRACK
                        .iter()
                        .map(|(state, tracks)| {
                            view! {
                                <h3 class="study-subhead">{*state}</h3>
                                <ul class="feature-list">
                                    {tracks
                                        .iter()
                                        .map(|(title, artist, length)| {
                                            view! { <li>{format!("{title} — {artist} · {length}")}</li> }
                                        })
                                        .collect_view()}
                                </ul>
                            }
                        })
                        .collect_view()}
                    <h3 class="study-subhead">"MUSIC CREDITS"</h3>
                    <ul class="feature-list">
                        {MUSIC_CREDITS.iter().map(|c| view! { <li>{*c}</li> }).collect_view()}
                    </ul>
                    <p class="channel-note">
                        "The companion themes are synthesized from scratch by the game's own "
                        "audio tools — no samples, no licensed material."
                    </p>
                </section>
                <section class="study-section">
                    <p class="band-label">"GET THE GAME"</p>
                    <div class="channel-rows">
                        <div class="channel-row">
                            <span class="play-badge-slot">
                                <img
                                    class="play-badge"
                                    src="/img/light-show/google-play-badge.png"
                                    alt="Get it on Google Play"
                                />
                            </span>
                            <div class="channel-text">
                                <p class="channel-line">
                                    <span class="channel-name">"Google Play"</span>
                                    <span class="channel-state">"Closed testing"</span>
                                </p>
                                <p class="channel-note">
                                    "The badge goes live with the production listing. Right now the "
                                    "game is in its closed test — the testers below are the way in."
                                </p>
                            </div>
                        </div>
                        <div class="channel-row">
                            <div class="channel-text">
                                <p class="channel-line">
                                    <span class="channel-name">"F-Droid"</span>
                                    <span class="channel-state">"Submission in preparation"</span>
                                </p>
                                <p class="channel-note">
                                    "The F-Droid build is built and validated; the store submission "
                                    "is being prepared. This row becomes the listing link when it lands."
                                </p>
                            </div>
                        </div>
                        <a
                            class="channel-row"
                            href="https://github.com/qompassai/light-show/releases"
                        >
                            <div class="channel-text">
                                <p class="channel-line">
                                    <span class="channel-name">"Windows"</span>
                                    <span class="channel-state is-live">"Trial build"</span>
                                </p>
                                <p class="channel-note">
                                    "A trial build for Windows is published with the project's "
                                    "releases on GitHub."
                                </p>
                            </div>
                        </a>
                    </div>
                </section>
                <section class="study-section">
                    <p class="band-label">"TESTERS"</p>
                    <p>
                        "Light Show opens to everyone when twelve testers have had it installed "
                        "for fourteen days — that is Google's gate for a personal developer "
                        "account, and it is the only thing between the finished game and its "
                        "launch. If you run fiber for a living, or teach it, or just like a hard "
                        "puzzle, your two weeks move the date."
                    </p>
                    <p class="study-link">
                        <a href="/light-show/testers">"Become a tester →"</a>
                    </p>
                </section>
            </article>
        </section>
    }
}

/// One step of the opt-in play-by-play.
///
/// Each step names what the screen is, its identifying landmarks, and
/// what the tester does there. `shot` is the genuine phone capture of
/// that screen once one exists; an empty `shot` renders the schematic
/// placeholder slot.
struct Step {
    title: &'static str,
    body: &'static [&'static str],
    shot: &'static str,
    shot_caption: &'static str,
}

const STEPS: &[Step] = &[
    Step {
        title: "Open the opt-in link on your phone",
        body: &[
            "The portal gives you a Google Play testing link. Open it on the phone you will play on — and before you tap anything, check which Google account the phone is signed into. The link only works for the account you registered with; if a page later tells you the app is not available, it is almost always the wrong account, and switching accounts and reopening the link fixes it.",
        ],
        shot: "",
        shot_caption: "The testing link, opened on the phone",
    },
    Step {
        title: "The opt-in page",
        body: &[
            "You land on Google's testing page for the game. Landmarks: the Light Show app icon — a swirl of colored light — and the name \u{201c}Light Show\u{201d} at the top, with the testing program explained underneath: you are about to get the test version, test builds can be unstable, and feedback you send goes to the developer. One action matters on this page: the button labeled \u{201c}BECOME A TESTER\u{201d}. Tap it.",
        ],
        shot: "",
        shot_caption: "The opt-in page, before joining",
    },
    Step {
        title: "\u{201c}You're now a tester\u{201d}",
        body: &[
            "The same page confirms, under the heading \u{201c}You're now a tester\u{201d}, that this Google account is in the program. Below the confirmation sits the way onward: a link labeled \u{201c}Download it on Google Play\u{201d}. Tap it — it takes you to the game's store listing with your tester access attached.",
        ],
        shot: "",
        shot_caption: "The confirmation, with the download link",
    },
    Step {
        title: "The listing, as a tester sees it",
        body: &[
            "The listing looks like any Play listing — icon, the name \u{201c}Light Show: Fiber Optic Puzzle\u{201d}, the screenshots from the game's home page — with one extra: a notice on the listing that you are a tester for this app. The button is the ordinary \u{201c}Install\u{201d} button. If Install has not appeared yet, give the opt-in a few minutes to register and reopen the listing.",
        ],
        shot: "",
        shot_caption: "The listing with tester access",
    },
    Step {
        title: "Install, and open the game",
        body: &[
            "Install like any other app. Open it and you are at the title screen: pick a companion, and Splice School — the first fusion splice — is the first lesson. The game asks for no account and no permissions, and it plays fully offline, so a splice tray, a truck cab, or a couch all work as test sites.",
        ],
        shot: "",
        shot_caption: "First launch — the title screen",
    },
    Step {
        title: "How updates arrive",
        body: &[
            "New test builds arrive through Google Play automatically, the way updates to any app do — there is nothing to side-load and no file to fetch. If an update is announced and has not reached you, open the Play Store's updates list and pull it from there. Playing the newest build is the one standing request: a bug reported against an old build may already be fixed.",
        ],
        shot: "",
        shot_caption: "A test update waiting in Google Play",
    },
    Step {
        title: "Sending feedback",
        body: &[
            "Two routes, both private. The tester portal — the page you joined through — takes reports in your own words: what you were doing, what you expected, what happened instead. And the Play listing itself carries a private feedback box for testers, whose contents go only to the developer; it is not a public review and nobody else sees it. A level name and a sentence about where you were beats a long report with neither.",
        ],
        shot: "",
        shot_caption: "The private feedback box on the listing",
    },
    Step {
        title: "Leaving the test",
        body: &[
            "Reopen the same opt-in link from step one: the page that once offered \u{201c}BECOME A TESTER\u{201d} now offers \u{201c}Leave the program\u{201d}. Tapping it removes the account from the test; uninstall the test build afterwards, and install the public version fresh when it launches. One honest ask: unless something forces your hand, stay in for the full fourteen days — an account that leaves and rejoins restarts its own fourteen-day count, and the gate counts testers, not intentions.",
        ],
        shot: "",
        shot_caption: "The opt-in page, offering Leave the program",
    },
];

#[component]
pub fn LightShowTesters() -> impl IntoView {
    view! {
        <Title text="Test Light Show - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — LIGHT SHOW TESTERS"</p>
            <article class="project-article">
                <header class="study-head">
                    <h1>"Help test Light Show"</h1>
                    <p class="page-lede">
                        "Fourteen days, twelve testers, one launch gate."
                    </p>
                </header>
                <section class="study-section">
                    <p class="band-label">"THE GATE"</p>
                    <p>
                        "Light Show is published through a personal Google Play developer account, "
                        "and Google holds accounts like it to a rule the big publishers never see: "
                        "before the game can open to everyone, it must run a closed test with at "
                        "least 12 testers opted in for at least 14 days. Only then does the "
                        "\u{201c}Apply for production\u{201d} door unlock. The game is finished and "
                        "sitting behind that gate; testers are what open it."
                    </p>
                </section>
                <section class="study-section">
                    <p class="band-label">"WHAT A TESTER DOES"</p>
                    <ul class="feature-list">
                        <li>
                            "Opt in with the Google account you registered with — the play-by-play "
                            "below walks every screen."
                        </li>
                        <li>
                            "Install the game from Google Play, exactly like any other app, and "
                            "keep it installed for the fourteen days."
                        </li>
                        <li>
                            "Play it — a few real sessions across the two weeks, on your own "
                            "schedule. Work through Splice School; chase the Storm Season drill."
                        </li>
                        <li>
                            "Say what breaks, what confuses, and what feels wrong, through the "
                            "portal or the listing's private feedback. Blunt is useful."
                        </li>
                    </ul>
                </section>
                <section class="study-section">
                    <p class="band-label">"WHO IT SUITS"</p>
                    <p>
                        "Working field service technicians first — the game is built out of your "
                        "day: link budgets, splice loss, splitter ratios, outage drills. If you "
                        "have ever landed a receive level inside the window on a real ONT, you are "
                        "the reader this was written for. Network engineering students and "
                        "instructors are the next best fit, and after that, anyone who simply "
                        "likes a logic puzzle with real numbers in it."
                    </p>
                </section>
                <section class="study-section">
                    <p class="band-label">"WHAT YOU'LL SEE — THE PLAY-BY-PLAY"</p>
                    <p>
                        "Every screen between the invitation and the game, in order, with the "
                        "landmarks to check yourself against. The walkthrough is text-first for "
                        "now; genuine captures from the first walk-through slot into each step "
                        "as they land."
                    </p>
                    <ol class="tester-steps">
                        {STEPS
                            .iter()
                            .enumerate()
                            .map(|(n, step)| {
                                view! {
                                    <li class="tester-step">
                                        <p class="entry-kind">
                                            {format!("STEP {}", n + 1)}
                                        </p>
                                        <h3 class="study-subhead">{step.title}</h3>
                                        {step
                                            .body
                                            .iter()
                                            .map(|p| view! { <p>{*p}</p> })
                                            .collect_view()}
                                        <figure class="step-shot">
                                            {if step.shot.is_empty() {
                                                view! {
                                                    <span class="step-shot-slot">
                                                        "Phone screenshot — to come"
                                                    </span>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <img
                                                        src={step.shot}
                                                        alt={step.shot_caption}
                                                        loading="lazy"
                                                    />
                                                }
                                                    .into_any()
                                            }}
                                            <figcaption>{step.shot_caption}</figcaption>
                                        </figure>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ol>
                </section>
                <section class="study-section">
                    <p class="band-label">"JOIN"</p>
                    <p>
                        "Registration, the opt-in link, and the feedback box all live at the "
                        "tester portal. Bring the Google account you actually use on your phone."
                    </p>
                    <p class="study-link">
                        <a href="https://testers.qompass.ai">
                            "Join the test at testers.qompass.ai →"
                        </a>
                    </p>
                    <p>
                        "New to the game itself? Its home is "
                        <a href="/light-show">"here"</a>
                        "."
                    </p>
                </section>
            </article>
        </section>
    }
}
