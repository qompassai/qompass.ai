//! Field Notes — fiber notes from a working technician (`/field-notes`).
//!
//! Renamed from `/field-service` on 2026-10-13; the notes live under
//! `/field-notes/notes/<slug>`. The landing also carries the US fiber
//! timeline, the glossary, and the telco-admin practice quizzes.
//!
//! Every figure in a note is a typical or standard-reference value and
//! says so on the page; standards are named, never quoted. Nothing here
//! describes any employer's systems or procedures — only general trade
//! knowledge from public standards.
//!
//! Adding a note: write its component, add a [`NOTES`] entry (kept in
//! alphabetical order by title), and add the match arm in
//! [`FieldNoteDispatch`]. `scripts/prerender.sh` prerenders every note
//! the index links, so the index is the one list to extend.
use crate::pages::fiber_timeline::FiberTimeline;
use crate::pages::fiber_trainer::{fiber_name, swatch_class, FIBERS};
use crate::pages::field_glossary::FieldGlossary;
use crate::pages::telco_quizzes::TelcoQuizzes;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

/// One entry in the Field Notes index.
struct NoteEntry {
    slug: &'static str,
    title: &'static str,
    blurb: &'static str,
}

/// The Field Notes index, alphabetical by title.
const NOTES: &[NoteEntry] = &[
    NoteEntry {
        slug: "light-loss-budgets",
        title: "Light-loss budgets",
        blurb: "dB math from zero, the budget equation, and a PON link worked end to end.",
    },
    NoteEntry {
        slug: "reading-an-otdr-trace",
        title: "Reading an OTDR trace",
        blurb: "What the axes mean, reflective vs. non-reflective events, dead zones, and the gainer that isn't.",
    },
    NoteEntry {
        slug: "tia-598-color-code",
        title: "TIA-598 color code",
        blurb: "The twelve colors, how they name fibers and units, and why it all matters at the splice tray.",
    },
];

/// The public L&I record for the license, the same link the /about identity list uses.
const LICENSE_URL: &str = "https://secure.lni.wa.gov/verify/Detail.aspx?UBI=&LIC=PORTEM*744M3&SAW=";

/// Where Field Notes sits in the site's flow — Standards, then
/// Analysis, then Field Notes, promoted into Research — with working
/// links both ways. Shared by the landing and the trainer.
pub fn flow_block() -> impl IntoView {
    view! {
        <section class="study-section fn-flow">
            <p class="band-label">"THE FLOW"</p>
            <p>
                "These are not raw sightings. They are the concrete results of work done under "
                <a href="/standards">"Standards"</a>
                " and weighed in "
                <a href="/analysis">"Analysis"</a>
                " — and the lessons those results taught. What holds is promoted into "
                <a href="/research">"Research"</a>
                "; what fails changes the practice, and in time the Standards themselves."
            </p>
            <nav class="fn-flow-nav" aria-label="Site flow">
                <a href="/analysis">"← Analysis: the weighing"</a>
                <a href="/research">"Research: the long arc →"</a>
            </nav>
        </section>
    }
}

#[component]
pub fn FieldNotes() -> impl IntoView {
    view! {
        <Title text="Field Notes - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"Step Three — Learn"</p>
            <article class="project-article">
                <header class="study-head">
                    <h1>"Field Notes"</h1>
                    <p class="page-lede">"Learning fiber inside out, and teaching it forward."</p>
                </header>
                <section class="study-section">
                    <p class="band-label">"WHY THIS PAGE"</p>
                    <p>
                        "I'm a working Field Service Technician and a Washington State Electrical "
                        "Administrator, Telecommunications (09) specialty — L&I license "
                        <a href=LICENSE_URL>"PORTEM*744M3"</a>
                        ". Most of what I know about fiber I learned the hard way: on a ladder, at "
                        "a splice tray, or staring at an OTDR trace that made no sense yet."
                    </p>
                    <p>
                        "These notes are the explanations I wish someone had handed me on day one, "
                        "written for the people doing the same work — Field Service Network "
                        "Specialists, Field Service Network Technicians, and Field Service "
                        "Technicians. Plain language first, then the mechanism underneath, then "
                        "one worked example. Everything comes from public standards and general "
                        "trade practice; every number is labeled for what it is."
                    </p>
                </section>
                <section class="study-section" id="two-working-lives">
                    <p class="band-label">"TWO WORKING LIVES"</p>
                    <p>
                        "Part of my working life happens at a desk. Quantum physics, AI, "
                        "programming — I value that work, and I think it matters. But it lives in "
                        "the abstract and in software. You can do it for a long time without "
                        "touching anything you could hold in your hand."
                    </p>
                    <p>
                        "The other part happens in the field, and fiber is the opposite. A "
                        "fiber-optic network is tangible: glass in a buffer tube, a drop on a pole, "
                        "a splice in a closure, light at an ONT. It is practical, and it actually "
                        "connects people."
                    </p>
                    <p>
                        "Neither half is the point on its own. The point is the combination: the "
                        "big ideation that comes from inside work — from being an indoor cat — "
                        "joined with the practical, hands-on knowledge that comes from every "
                        "aspect of even the most basic residential install: the route the drop "
                        "takes, the light that reaches the ONT, the wall it goes through, the "
                        "person waiting on it."
                    </p>
                </section>
                <section class="study-section">
                    <p class="band-label">"FIELD NOTES"</p>
                    <div class="fs-notes">
                        {NOTES
                            .iter()
                            .map(|note| {
                                view! {
                                    <a class="fs-note" href=format!("/field-notes/notes/{}", note.slug)>
                                        <span class="entry-kind">"FIELD NOTE"</span>
                                        <span class="fs-note-title">{note.title}</span>
                                        <span class="fs-note-blurb">{note.blurb}</span>
                                    </a>
                                }
                            })
                            .collect_view()}
                    </div>
                </section>
                <section class="study-section">
                    <p class="band-label">"TIA-598 TRAINER"</p>
                    <p>
                        "The fiber color code is the alphabet of the splice tray, and like any "
                        "alphabet it only gets fast with reps. The trainer drills it three ways — "
                        "color to position, position to color, and the full sequence in order — "
                        "with instant feedback and a score per round. Big buttons, built for a "
                        "phone in a truck cab."
                    </p>
                    <p class="study-link">
                        <a href="/field-notes/trainer">"Open the trainer →"</a>
                    </p>
                </section>
                <section class="study-section">
                    <p class="band-label">"LIGHT SHOW"</p>
                    <p>
                        "Light Show is a training artifact built by an FST: a puzzle game where "
                        "you route light from the OLT to the customer's ONT and every splice, "
                        "connector, and splitter costs real dB. If the light-loss budget note "
                        "is the theory, Light Show is the practice round."
                    </p>
                    <p class="study-link">
                        <a href="/light-show">"See Light Show →"</a>
                    </p>
                </section>
                <FiberTimeline />
                <FieldGlossary />
                <TelcoQuizzes />
                <section class="study-section">
                    <p class="band-label">"LEARNING IN PUBLIC"</p>
                    <p>
                        "I'm still studying — the standards, the test gear, and the field itself — "
                        "and working through continuing education toward my next license renewal. "
                        "When I learn something worth passing on, it lands here. When a note is "
                        "wrong, it gets corrected here, in the open; if you spot an error, "
                        <a href="/about#identity">"tell me"</a>
                        "."
                    </p>
                </section>
                {flow_block()}
            </article>
        </section>
    }
}

/// Header shared by every note: bearing, title, lede.
fn note_head(title: &'static str, lede: &'static str) -> impl IntoView {
    view! {
        <Title text=format!("{title} - Field Notes - Qompass AI") />
        <p class="page-bearing">"FIELD NOTES — NOTE"</p>
        <header class="study-head">
            <h1>{title}</h1>
            <p class="page-lede">{lede}</p>
        </header>
    }
}

/// Footer shared by every note: back to the index.
fn note_foot() -> impl IntoView {
    view! {
        <p class="study-link fs-back">
            <a href="/field-notes">"← All field notes"</a>
        </p>
    }
}

#[component]
pub fn FieldNoteDispatch() -> impl IntoView {
    let params = use_params_map();
    let slug = params.with(|map| map.get("slug").unwrap_or_default());
    let note = match slug.as_str() {
        "light-loss-budgets" => view! { <LightLossBudgets /> }.into_any(),
        "reading-an-otdr-trace" => view! { <ReadingOtdr /> }.into_any(),
        "tia-598-color-code" => view! { <ColorCode /> }.into_any(),
        _ => {
            return view! {
                <section class="page">
                    <h1>"Off chart"</h1>
                    <p>"That bearing does not exist on this chart."</p>
                </section>
            }
            .into_any();
        }
    };
    view! {
        <section class="page">
            <article class="project-article">{note}</article>
        </section>
    }
    .into_any()
}

#[component]
fn ReadingOtdr() -> impl IntoView {
    view! {
        {note_head("Reading an OTDR trace", "Sonar for glass: what the squiggle is telling you.")}
        <section class="study-section">
            <p class="band-label">"THE SHORT VERSION"</p>
            <p>
                "An OTDR (optical time-domain reflectometer) fires short pulses of light into one "
                "end of a fiber and listens to what comes back. Glass scatters a tiny, steady "
                "fraction of light backward everywhere along its length, and anything abrupt — "
                "a connector, a break, the far end — throws back a brighter flash. Time the "
                "echoes and you know where things are; measure how faint they've grown and you "
                "know how much light was lost getting there. It's sonar, with photons."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"THE AXES"</p>
            <p>
                <strong>"Horizontal: distance."</strong>
                " The instrument measures round-trip time and converts it with the fiber's group "
                "index of refraction (IOR): distance = (speed of light × time) ÷ (2 × IOR). The "
                "÷2 is the round trip. A wrong IOR setting stretches or shrinks every distance on "
                "the trace by the same proportion, so set it from the cable manufacturer's data "
                "before you trust a locate."
            </p>
            <p>
                <strong>"Vertical: relative power, in dB."</strong>
                " It's a logarithmic scale of backscattered power, and the OTDR scales it so a "
                "drop on screen reads as one-way loss — the light actually went out and back, "
                "and the instrument halves that for you. The absolute height means little; "
                "slopes and steps are what you read."
            </p>
            <p>
                "A healthy trace reads left to right: a tall spike at the front panel, a long "
                "straight line sloping gently down, small steps where events are, and a final "
                "spike (or a plain drop) where the fiber ends, after which only the instrument's "
                "noise floor remains."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"EVENTS"</p>
            <h3 class="study-subhead">"Reflective events"</h3>
            <p>
                "Where light crosses a glass-air-glass boundary, some of it bounces straight "
                "back (Fresnel reflection) — a spike above the backscatter line, followed by a "
                "step down for the loss. Connectors and mechanical splices do this, and so do a "
                "cleaved or cracked end. A spike at an unexpected distance on a link with no "
                "connector there is worth a hard look: it may be a crack."
            </p>
            <h3 class="study-subhead">"Non-reflective events"</h3>
            <p>
                "A fusion splice melts two fibers into continuous glass, so there's no boundary "
                "to reflect from — just a step down. A macrobend (a fiber bent tighter than it "
                "tolerates) leaks light out of the core and also shows as a plain step."
            </p>
            <p>
                "Telling a splice from a bend: bend loss grows with wavelength, so a bend shows a "
                "noticeably bigger step at 1550 nm than at 1310 nm, while a fusion splice reads "
                "about the same at both. Shooting two wavelengths is the cheapest bend detector "
                "you own."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"DEAD ZONES AND THE LAUNCH CABLE"</p>
            <p>
                "A bright reflection briefly blinds the OTDR's receiver, the way a camera flash "
                "leaves you seeing spots. The distance it takes to recover is a dead zone, and "
                "there are two kinds:"
            </p>
            <ul class="feature-list">
                <li>
                    <strong>"Attenuation dead zone"</strong>
                    " — the distance after a reflective event before the trace settles back onto "
                    "the backscatter line well enough to measure the loss of the next event. "
                    "Always the longer of the two."
                </li>
                <li>
                    <strong>"Event dead zone"</strong>
                    " — the shortest distance after a reflective event at which a second "
                    "reflective event shows up as a separate event at all."
                </li>
            </ul>
            <p>
                "Both grow with pulse width: a wider pulse carries more energy and sees farther, "
                "but blurs more of the trace. That's the basic OTDR trade — range versus "
                "resolution."
            </p>
            <p>
                "The worst flash of all is the instrument's own front-panel connector, which "
                "means the first connector of the link you actually care about sits inside a dead "
                "zone. A launch cable (also called a pulse suppressor) — a known length of fiber "
                "between the OTDR and the link — moves that first connector out where it can be "
                "measured. A receive (tail) cable at the far end does the same for the last "
                "connector, which otherwise has no backscatter after it to measure against. How "
                "long? It depends on pulse width and the instrument; for outside plant, a few "
                "hundred meters to a kilometer or more is common. That's a rule of thumb, not a "
                "standard — check your OTDR's documentation."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"SLOPE"</p>
            <p>
                "The straight stretches between events slope down at the fiber's attenuation "
                "rate, in dB/km. Read it as the drop between two points on a clean stretch "
                "divided by the distance between them. Typical values for cabled singlemode "
                "outside-plant fiber:"
            </p>
            <div class="bench-scroll">
                <table class="bench-table">
                    <thead>
                        <tr>
                            <th>"Wavelength"</th>
                            <th>"Typical attenuation"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"1310 nm"</td>
                            <td>"about 0.33–0.35 dB/km"</td>
                        </tr>
                        <tr>
                            <td>"1550 nm"</td>
                            <td>"about 0.20–0.25 dB/km"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <p class="bench-note">
                "Typical, not guaranteed: the cable's datasheet is the authority for your cable. "
                "Design limits in the TIA-568 family are looser than these typical figures. A "
                "stretch that slopes noticeably steeper than its neighbors is a clue — a "
                "stressed section, or a different fiber type spliced in."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"THE GAINER"</p>
            <p>
                "Sometimes a splice reads as a gain — the trace steps up. Glass doesn't amplify "
                "light; this is an artifact. The OTDR assumes both sides of the splice scatter "
                "light back equally well. If the downstream fiber has a different mode field "
                "diameter or backscatter coefficient, it returns more light per meter, and the "
                "trace jumps up even though light was lost."
            </p>
            <p>
                "Shoot the same splice from the other end and the error flips sign: it reads as "
                "extra loss. Average the two directions and the backscatter difference cancels, "
                "leaving the true splice loss. That's why bidirectional testing is the reference "
                "method for splice loss: a one-way reading of any splice between dissimilar "
                "fibers is biased, and a gainer is just the bias made obvious."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"WORKED EXAMPLE"</p>
            <p>
                "A described trace at 1310 nm, shot from end A through a 500 m launch cable. "
                "The numbers are illustrative, chosen to show each kind of event:"
            </p>
            <ul class="feature-list">
                <li>
                    <strong>"0 m:"</strong>
                    " big spike — the front panel. Ignore it; that's what the launch cable is for."
                </li>
                <li>
                    <strong>"500 m:"</strong>
                    " spike, then a 0.30 dB step. Reflective, at the end of the launch cable: the "
                    "first connector of the link, measured cleanly. 0.30 dB is inside the 0.75 dB "
                    "TIA allowance for a mated pair."
                </li>
                <li>
                    <strong>"500 m to 4,500 m:"</strong>
                    " straight line falling from −3.10 dB to −4.42 dB on the vertical scale. "
                    "Slope = 1.32 dB ÷ 4.0 km = 0.33 dB/km — right where typical singlemode sits "
                    "at 1310 nm."
                </li>
                <li>
                    <strong>"4,500 m:"</strong>
                    " the trace steps up 0.08 dB with no spike. Non-reflective, so a fusion "
                    "splice — and the step up says gainer. Shot from end B, the same splice reads "
                    "0.20 dB of loss. True loss = (−0.08 + 0.20) ÷ 2 = 0.06 dB. A good splice "
                    "that only looked strange."
                </li>
                <li>
                    <strong>"7,200 m:"</strong>
                    " a 0.15 dB step, no spike — at 1310 nm. The 1550 nm shot shows 0.60 dB at "
                    "the same spot. A splice wouldn't care about wavelength that much; a bend "
                    "would. Go look at that location for a tight slack loop or a crushed "
                    "section."
                </li>
                <li>
                    <strong>"9,000 m:"</strong>
                    " a spike, a step, then a short stretch of backscatter and a final spike "
                    "into noise: the far-end connector, then the receive cable and its open end. "
                    "Link length: 9,000 − 500 = 8,500 m."
                </li>
            </ul>
            <p>
                "Verdict: the link is sound except one location. The gainer is fine once "
                "averaged; the event at 7,200 m is the call to make."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"SOURCES AND LIMITS"</p>
            <p>
                "Mechanisms are general OTDR physics. The mated-pair allowance is from the "
                "TIA-568 family; attenuation figures are typical values, not limits. The worked "
                "trace is illustrative. Your instrument's manual and your cable's datasheet "
                "outrank this page."
            </p>
            {note_foot()}
        </section>
    }
}

#[component]
fn LightLossBudgets() -> impl IntoView {
    view! {
        {note_head("Light-loss budgets", "How much light you start with, how much you need, and where it all goes.")}
        <section class="study-section">
            <p class="band-label">"THE SHORT VERSION"</p>
            <p>
                "A transmitter puts a certain amount of light into the fiber. A receiver needs a "
                "certain minimum to read the signal. Everything in between — glass, connectors, "
                "splices, splitters — takes a cut. The budget is the difference between what you "
                "start with and what you need; the link works if everything's cuts added up fit "
                "inside it, with some left over for a bad day."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"DB FROM ZERO"</p>
            <p>
                "Light power spans a huge range, so the trade counts it on a logarithmic scale. "
                "Two units do all the work, and mixing them up is the classic mistake:"
            </p>
            <ul class="feature-list">
                <li>
                    <strong>"dB"</strong>
                    " is a ratio — how much bigger or smaller one power is than another: "
                    "dB = 10 × log₁₀(P₂ ÷ P₁). Losses and gains are in dB."
                </li>
                <li>
                    <strong>"dBm"</strong>
                    " is an absolute power, measured against 1 milliwatt: 0 dBm = 1 mW. "
                    "Transmitter output and receiver readings are in dBm."
                </li>
            </ul>
            <p>
                "Landmarks worth memorizing: every −3 dB halves the power, every −10 dB divides "
                "it by ten. So −10 dBm = 0.1 mW, −20 dBm = 0.01 mW, and −3 dBm is about 0.5 mW."
            </p>
            <p>
                "Why losses simply add: a fiber passes some fraction of light, a connector passes "
                "a fraction of that, and so on — real power multiplies through the fractions. "
                "Logarithms turn multiplication into addition, so in dB you just add the losses "
                "up. The unit arithmetic follows: dBm − dB = dBm (a power minus a loss is a "
                "power), and dBm − dBm = dB (the gap between two powers is a ratio)."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"THE BUDGET EQUATION"</p>
            <pre class="fs-calc">
                <code>
                    "available budget = transmitter power (dBm) − receiver sensitivity (dBm)\n\n"
                    "link loss = (fiber km × dB/km)\n"
                    "          + (connector pairs × dB each)\n"
                    "          + (splices × dB each)\n"
                    "          + splitter loss\n\n"
                    "margin = available budget − link loss"
                </code>
            </pre>
            <p>
                "Use the transmitter's minimum rated output and the receiver's sensitivity from "
                "the datasheets — worst case on both ends, so the answer holds for any unit off "
                "the shelf. The margin is your allowance for aging, repairs, temperature, and "
                "the splice someone adds in ten years. How much to keep is a design choice; "
                "about 3 dB is a common rule of thumb, not a standard."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"TYPICAL REFERENCE VALUES"</p>
            <div class="bench-scroll">
                <table class="bench-table">
                    <thead>
                        <tr>
                            <th>"Item"</th>
                            <th>"Value"</th>
                            <th>"What kind of number"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Fusion splice"</td>
                            <td>"~0.1 dB"</td>
                            <td>"Typical planning value; good splices often measure less"</td>
                        </tr>
                        <tr>
                            <td>"Mated connector pair"</td>
                            <td>"≤ 0.75 dB"</td>
                            <td>"Maximum allowance in the TIA-568 family"</td>
                        </tr>
                        <tr>
                            <td>"Singlemode fiber, 1310 nm"</td>
                            <td>"~0.35 dB/km"</td>
                            <td>"Typical planning value; datasheet governs"</td>
                        </tr>
                        <tr>
                            <td>"Splitter, 1×32"</td>
                            <td>"≈ 17 dB"</td>
                            <td>"Typical datasheet figure, including excess loss"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <p>
                "Splitters have a floor set by plain arithmetic: split the light N ways and each "
                "leg gets 1/N of it, a loss of 10 × log₁₀(N) dB. Real splitters lose a bit more "
                "(excess loss, plus unevenness between legs), so always budget the datasheet "
                "value, never the ideal:"
            </p>
            <div class="bench-scroll">
                <table class="bench-table">
                    <thead>
                        <tr>
                            <th>"Split"</th>
                            <th>"Ideal loss (math floor)"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"1×2"</td>
                            <td>"3.0 dB"</td>
                        </tr>
                        <tr>
                            <td>"1×4"</td>
                            <td>"6.0 dB"</td>
                        </tr>
                        <tr>
                            <td>"1×8"</td>
                            <td>"9.0 dB"</td>
                        </tr>
                        <tr>
                            <td>"1×16"</td>
                            <td>"12.0 dB"</td>
                        </tr>
                        <tr>
                            <td>"1×32"</td>
                            <td>"15.1 dB"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <p class="bench-note">
                "The ~2 dB gap between the 1×32 ideal (15.1 dB) and the typical datasheet "
                "figure (≈ 17 dB) is real-world excess loss — exactly the kind of detail a "
                "budget built on ideal math gets wrong."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"WORKED EXAMPLE — A PON LINK, END TO END"</p>
            <p>
                "A passive optical network: one OLT port, one 1×32 splitter in the field, and a "
                "customer's ONT. The optics values are illustrative — in the range of common "
                "GPON optics, but use your own datasheets in real life:"
            </p>
            <ul class="feature-list">
                <li>"Fiber: 12 km, budgeted at 0.35 dB/km (the 1310 nm planning value used for every wavelength — a conservative simplification)"</li>
                <li>"Fusion splices: 6, at 0.1 dB each (typical)"</li>
                <li>"Mated connector pairs: 4, at the 0.75 dB TIA allowance each (worst case)"</li>
                <li>"OLT transmitter, minimum output: +1.5 dBm (illustrative)"</li>
                <li>"ONT receiver sensitivity: −27.0 dBm (illustrative)"</li>
                <li>"Splitter: 1×32, at 17.0 dB (typical datasheet figure)"</li>
            </ul>
            <pre class="fs-calc">
                <code>
                    "available budget = +1.5 − (−27.0)          = 28.5 dB\n\n"
                    "fiber            = 12 km × 0.35 dB/km      =  4.20 dB\n"
                    "connector pairs  = 4 × 0.75 dB             =  3.00 dB\n"
                    "fusion splices   = 6 × 0.1 dB              =  0.60 dB\n"
                    "splitter (1×32)                            = 17.00 dB\n"
                    "                                             --------\n"
                    "link loss                                  = 24.80 dB\n\n"
                    "margin           = 28.5 − 24.80            =  3.70 dB\n"
                    "expected at ONT  = +1.5 dBm − 24.80 dB     = −23.30 dBm"
                </code>
            </pre>
            <p>
                "The link closes with 3.7 dB to spare, and a meter at the ONT should read near "
                "−23.3 dBm on a worst-case transmitter. Look at where the budget went: the "
                "splitter alone is more than two-thirds of the loss. And the connectors, budgeted "
                "at their allowance, cost five times what the splices do — which is why clean, "
                "good connections matter so much."
            </p>
            <p>
                "One more check worth knowing: receivers also have a maximum (overload) input. "
                "A very short link with no splitter can deliver too much light, and the cure is "
                "an attenuator, not more fiber."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"WHAT EATS BUDGETS IN THE REAL WORLD"</p>
            <ul class="feature-list">
                <li>
                    <strong>"Dirty endfaces."</strong>
                    " The core of a singlemode fiber is only about 8 to 9 microns across; a speck of dust "
                    "you can't see covers a lot of it. One contaminated connector can cost more "
                    "than every splice in the example combined, and mating a dirty connector can "
                    "transfer the dirt and scratch both faces. Inspect, clean, inspect again — "
                    "every time, both sides."
                </li>
                <li>
                    <strong>"Macrobends."</strong>
                    " A fiber pulled too tight around a corner, a clip, or a slack loop leaks "
                    "light. Bends hurt longer wavelengths more, so a link can pass at one "
                    "wavelength and fail at another — the OTDR note shows how to catch it."
                </li>
                <li>
                    <strong>"One bad splice."</strong>
                    " Budgeted at 0.1 dB, a poor one can measure many times that. A budget with "
                    "3.7 dB of margin can absorb one surprise, not a handful."
                </li>
            </ul>
        </section>
        <section class="study-section">
            <p class="band-label">"SOURCES AND LIMITS"</p>
            <p>
                "The connector allowance is from the TIA-568 family. Splice, fiber, and splitter "
                "figures are typical values; the optics in the example are illustrative. "
                "Datasheets for the actual parts outrank every number here. To practice this "
                "with real numbers, "
                <a href="/light-show">"Light Show"</a>
                " is built around exactly this math."
            </p>
            {note_foot()}
        </section>
    }
}

#[component]
fn ColorCode() -> impl IntoView {
    view! {
        {note_head("TIA-598 color code", "Twelve colors that tell you which glass is which.")}
        <section class="study-section">
            <p class="band-label">"THE SHORT VERSION"</p>
            <p>
                "A fiber cable can hold hundreds of strands that all look identical except for "
                "their color. TIA-598 is the standard that says what color each position gets, "
                "so a technician anywhere can open a cable, count colors, and know exactly "
                "which fiber is which — the same way, every time."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"THE TWELVE"</p>
            <ol class="ft-built fs-sequence">
                {(0..FIBERS.len())
                    .map(|position| {
                        view! {
                            <li class="ft-built-item">
                                <span class=swatch_class(position) aria-hidden="true"></span>
                                <span>{format!("{} {}", position + 1, fiber_name(position))}</span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
            <p>
                "Blue, Orange, Green, Brown, Slate, White, Red, Black, Yellow, Violet, Rose, "
                "Aqua. The first five follow the same order telephone copper has long used for "
                "its pair colors, so a lot of people already half-know it. Learn the order cold: "
                "everything else on this page is built out of it."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"FIBERS AND UNITS"</p>
            <p>
                "The same twelve-color sequence works at two levels. Inside a buffer tube (or "
                "other unit), it numbers the fibers: the blue fiber is fiber 1, orange is fiber "
                "2, and so on. And the tubes themselves are colored in the same sequence: the "
                "blue tube is tube 1, the orange tube is tube 2."
            </p>
            <p>
                "So a fiber has two colors: its tube's and its own. \"Green tube, red fiber\" "
                "is a complete address. In a cable with twelve fibers per tube, it converts to "
                "a fiber count number like this:"
            </p>
            <pre class="fs-calc">
                <code>
                    "fiber number  = (tube position − 1) × 12 + fiber position\n"
                    "tube position = fiber number ÷ 12, rounded up\n"
                    "fiber position = what's left over (a remainder of 0 means 12)"
                </code>
            </pre>
            <p>
                "Fiber color doesn't depend on the tube: fiber 7 is red in the blue tube and "
                "red in the aqua tube. The tube color is what tells them apart."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"PAST TWELVE: TRACERS AND REPEATS"</p>
            <p>
                "Twelve colors run out fast. When a unit holds more than twelve fibers, or a "
                "cable holds more than twelve tubes, the sequence repeats, and a marking tells "
                "the second pass from the first — commonly a stripe or dashes (a tracer) on the "
                "repeated color. In the common convention for positions 13–24, the tracer is "
                "black, and the black fiber takes a contrasting tracer instead so it stays "
                "readable."
            </p>
            <p>
                "High-count cables add another level: groups of tubes are bundled into units "
                "and identified by colored binder threads or tapes, which follow the same "
                "sequence again. Tube 1 in binder 2 is a different tube from tube 1 in binder 1, "
                "even though both are blue."
            </p>
            <p>
                "Past the base twelve, conventions vary between cable designs. The cable "
                "manufacturer's datasheet or color chart for the exact cable on your reel is "
                "the authority — read it before you open the sheath, not after."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"WHY IT MATTERS AT THE SPLICE TRAY"</p>
            <p>
                "Splicing is matching: this fiber on one side to that fiber on the other, and "
                "the splice records and drawings name each one by its colors. Get one pair "
                "crossed and the splice can be optically perfect and the link still dead — the "
                "light is going to the wrong place. Color errors are invisible on a loss meter "
                "until someone's service doesn't come up."
            </p>
            <p>
                "Good habits: confirm tube and fiber colors against the record before you cut, "
                "keep tubes and fibers in sequence in the tray so a wrong pair looks wrong, and "
                "check colors in good light — slate, white, and aqua can blur together under a "
                "dim lamp or after years in the field."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"WORKED EXAMPLE"</p>
            <p>"The record says fiber 31, in a cable with twelve fibers per tube. Which glass?"</p>
            <pre class="fs-calc">
                <code>
                    "tube position  = 31 ÷ 12 = 2.58 → rounded up = 3  → Green tube\n"
                    "fiber position = 31 − (2 × 12) = 7                → Red fiber\n\n"
                    "check: (3 − 1) × 12 + 7 = 31  ✓"
                </code>
            </pre>
            <p>"Fiber 31 is the red fiber in the green tube."</p>
        </section>
        <section class="study-section">
            <p class="band-label">"DRILL IT"</p>
            <p>
                "Knowing the sequence and recalling it instantly at the tray are two different "
                "skills. The "
                <a href="/field-notes/trainer">"TIA-598 trainer"</a>
                " drills color to position, position to color, and the whole order, with a "
                "score per round."
            </p>
        </section>
        <section class="study-section">
            <p class="band-label">"SOURCES AND LIMITS"</p>
            <p>
                "The twelve-color sequence and the fiber/unit scheme come from TIA-598. Tracer "
                "and binder conventions past the first twelve vary by cable design; the "
                "manufacturer's documentation for your cable governs."
            </p>
            {note_foot()}
        </section>
    }
}
