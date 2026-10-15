//! The US fiber-optics timeline on `/field-notes`: sixteen milestones
//! in five eras, from Corning's low-loss glass (1970) to the federal
//! buildout programs of the 2020s.
//!
//! Every date and figure was checked against the source each milestone
//! links; the prose is original. Each era carries one true inline-SVG
//! diagram, drawn at a 360-unit width so its labels stay legible on a
//! phone without a second, narrow variant.
//!
//! The server renders every era in full, so the page reads completely
//! without JavaScript. Once hydrated, the era buttons, the previous /
//! next buttons, and the arrow keys show one era at a time: the panels
//! only gain `hidden` after the first client-side effect runs, so the
//! server render and the hydrated render match.
use leptos::ev::KeyboardEvent;
use leptos::prelude::*;

struct Milestone {
    /// ISO date (or year) for the `<time>` element.
    date: &'static str,
    display: &'static str,
    title: &'static str,
    body: &'static str,
    sources: &'static [(&'static str, &'static str)],
}

struct Era {
    key: &'static str,
    title: &'static str,
    span: &'static str,
    intro: &'static str,
    milestones: &'static [Milestone],
    diagram: fn() -> AnyView,
}

const WIKI_FIBER: (&str, &str) = ("Wikipedia — Optical fiber", "https://en.wikipedia.org/wiki/Optical_fiber");
const WIKI_FOC: (&str, &str) =
    ("Wikipedia — Fiber-optic communication", "https://en.wikipedia.org/wiki/Fiber-optic_communication");
const ETHW: (&str, &str) = ("IEEE ETHW — Fiber Optics", "https://ethw.org/Fiber_Optics");
const WIKI_PON: (&str, &str) =
    ("Wikipedia — Passive optical network", "https://en.wikipedia.org/wiki/Passive_optical_network");

const ERAS: &[Era] = &[
    Era {
        key: "glass",
        title: "Glass that carries light",
        span: "1970–1980",
        intro: "Fiber starts as a materials problem: glass pure enough that light survives a kilometer of it. \
                Once that is solved, the phone companies put real calls on it within seven years.",
        milestones: &[
            Milestone {
                date: "1970",
                display: "1970",
                title: "Corning makes glass clear enough to signal through",
                body: "A Corning Glass Works team — Robert Maurer, Donald Keck, and Peter Schultz among them — \
                       drew a titanium-doped silica fiber that lost about 17 dB per kilometer, under the roughly \
                       20 dB/km threshold that made fiber practical for communication.",
                sources: &[WIKI_FIBER, ETHW],
            },
            Milestone {
                date: "1977-04-22",
                display: "April 22, 1977",
                title: "First live telephone traffic over fiber",
                body: "GTE carried live telephone calls over fiber in Long Beach, California, at 6 Mb/s. AT&T \
                       followed quickly with a fiber system in Chicago.",
                sources: &[WIKI_FOC, ETHW],
            },
            Milestone {
                date: "1980",
                display: "1980",
                title: "Fiber carries the Lake Placid Olympics",
                body: "AT&T installed a public demonstration system in Lake Placid, New York, in 1979, and it \
                       carried multiple television signals during the 1980 Winter Olympics.",
                sources: &[ETHW],
            },
        ],
        diagram: tir_diagram,
    },
    Era {
        key: "long-haul",
        title: "The long haul",
        span: "1983–1988",
        intro: "Fiber leaves the demonstration city and starts tying cities, then continents, together. \
                The links still turn light back into electricity every few dozen kilometers — and the \
                device that ends that habit is invented in the same years.",
        milestones: &[
            Milestone {
                date: "1983",
                display: "1983",
                title: "MCI lights New York to Washington",
                body: "MCI, working with Corning, opened a commercial fiber-optic system between New York and \
                       Washington at a wavelength of 1.3 micrometers. AT&T soon answered with a competing line.",
                sources: &[ETHW],
            },
            Milestone {
                date: "1987",
                display: "1986–87",
                title: "The erbium-doped fiber amplifier",
                body: "Groups at the University of Southampton and AT&T Bell Laboratories demonstrated gain in \
                       erbium-doped fiber. An amplifier made of fiber boosts the light as light, so a long link \
                       no longer has to convert to electricity and back at every repeater.",
                sources: &[
                    ("Wikipedia — Optical amplifier", "https://en.wikipedia.org/wiki/Optical_amplifier"),
                    WIKI_FIBER,
                ],
            },
            Milestone {
                date: "1988",
                display: "1988",
                title: "TAT-8 crosses the Atlantic in glass",
                body: "The first transatlantic fiber-optic cable, built by a consortium led by AT&T, France \
                       Télécom, and British Telecom, linked the United States, the United Kingdom, and France. \
                       Each fiber carried 280 Mb/s of traffic — about 40,000 telephone circuits for the \
                       system — through regenerators spaced roughly 40 km apart.",
                sources: &[("Wikipedia — TAT-8", "https://en.wikipedia.org/wiki/TAT-8"), ETHW],
            },
        ],
        diagram: repeater_diagram,
    },
    Era {
        key: "many-colors",
        title: "Many colors on one strand",
        span: "1996",
        intro: "Amplifiers and wavelength-division multiplexing arrive together: one fiber carries many \
                colors of light, each its own channel, and the amplifiers boost them without unpacking them. \
                Capacity stops growing by the decade and starts growing by the year.",
        milestones: &[
            Milestone {
                date: "1996",
                display: "1996",
                title: "Amplifiers go under the Atlantic",
                body: "TAT-12/13 entered operation carrying 2 × 5 Gb/s, the first cables on the transatlantic \
                       route to use erbium-doped fiber amplifiers.",
                sources: &[("Wikipedia — TAT-12/13", "https://en.wikipedia.org/wiki/TAT-12/13")],
            },
            Milestone {
                date: "1996",
                display: "1996",
                title: "Dense WDM goes commercial",
                body: "Ciena delivered the first commercial dense wavelength-division multiplexing (DWDM) \
                       system on Sprint's network: many wavelengths, each an independent channel, sharing a \
                       single fiber.",
                sources: &[
                    (
                        "Wikipedia — Wavelength-division multiplexing",
                        "https://en.wikipedia.org/wiki/Wavelength-division_multiplexing",
                    ),
                    ("Wikipedia — Ciena", "https://en.wikipedia.org/wiki/Ciena"),
                ],
            },
        ],
        diagram: capacity_diagram,
    },
    Era {
        key: "to-the-home",
        title: "Fiber to the home",
        span: "1998–2016",
        intro: "The standards bodies work out how to share one fiber among many homes without any powered \
                equipment in the field — the passive optical network — and carriers start running glass all \
                the way to the house.",
        milestones: &[
            Milestone {
                date: "1998-10",
                display: "October 1998",
                title: "BPON: a passive optical network standard",
                body: "ITU-T approved G.983.1, broadband PON, building on fiber-to-the-home work the Full \
                       Service Access Network (FSAN) group began in 1995: one fiber from the office, split \
                       passively among many homes.",
                sources: &[("ITU-T — G.983.1", "https://www.itu.int/rec/T-REC-G.983.1/en"), WIKI_PON],
            },
            Milestone {
                date: "2003-03",
                display: "2003–2004",
                title: "GPON and EPON",
                body: "ITU-T G.984.1 defined gigabit-capable PON in March 2003, and the IEEE ratified Ethernet \
                       PON (802.3ah) in 2004 — two standards families built on the same passive split.",
                sources: &[("ITU-T — G.984.1", "https://www.itu.int/rec/T-REC-G.984.1/en"), WIKI_PON],
            },
            Milestone {
                date: "2004",
                display: "2004–2005",
                title: "FiOS lights Keller, Texas",
                body: "Verizon launched FiOS fiber internet service in Keller, Texas, in 2004, and added FiOS TV \
                       there in September 2005.",
                sources: &[("Wikipedia — Verizon Fios", "https://en.wikipedia.org/wiki/Verizon_Fios")],
            },
            Milestone {
                date: "2012",
                display: "2012",
                title: "Google Fiber comes to Kansas City",
                body: "Google Fiber introduced service in the Kansas City metropolitan area in 2012 and grew to \
                       twenty area suburbs within three years.",
                sources: &[("Wikipedia — Google Fiber", "https://en.wikipedia.org/wiki/Google_Fiber")],
            },
            Milestone {
                date: "2016-06",
                display: "June 2016",
                title: "XGS-PON: ten gigabits both ways",
                body: "ITU-T approved G.9807.1, XGS-PON: a passive optical network at up to 10 Gb/s, \
                       symmetric — the same rate upstream and downstream.",
                sources: &[("ITU-T — G.9807.1", "https://www.itu.int/rec/T-REC-G.9807.1/en"), WIKI_PON],
            },
        ],
        diagram: pon_diagram,
    },
    Era {
        key: "buildout",
        title: "Capacity, and public money",
        span: "2018–2023",
        intro: "At the top of the network a single cable now carries terabits per fiber pair. At the edge, \
                the open question is who still has no line at all — and federal programs put billions behind \
                reaching them.",
        milestones: &[
            Milestone {
                date: "2018-02",
                display: "February 2018",
                title: "MAREA: 160 Tb/s from Virginia Beach",
                body: "MAREA, finished in September 2017 between Virginia Beach and Sopelana, Spain, began \
                       operating in February 2018. Its eight fiber pairs were designed for 160 Tb/s in total — \
                       20 Tb/s per pair.",
                sources: &[("Wikipedia — MAREA", "https://en.wikipedia.org/wiki/MAREA")],
            },
            Milestone {
                date: "2020-11-25",
                display: "November 25, 2020",
                title: "RDOF Phase I awards $9.2 billion",
                body: "The FCC's Rural Digital Opportunity Fund Phase I auction (Auction 904) ended, awarding \
                       $9.2 billion in support, paid out over ten years, to 180 winning bidders. More than 85 \
                       percent of the locations are in areas where the winner committed to gigabit service.",
                sources: &[(
                    "USAC — Rural Digital Opportunity Fund",
                    "https://www.usac.org/high-cost/funds/rural-digital-opportunity-fund/",
                )],
            },
            Milestone {
                date: "2021-11-15",
                display: "November 15, 2021",
                title: "BEAD: $42.45 billion for broadband",
                body: "The Infrastructure Investment and Jobs Act, signed November 15, 2021, funded NTIA's \
                       Broadband Equity, Access, and Deployment program at $42.45 billion. In June 2023, NTIA \
                       announced allocations for all 56 states and territories.",
                sources: &[
                    (
                        "Wikipedia — Infrastructure Investment and Jobs Act",
                        "https://en.wikipedia.org/wiki/Infrastructure_Investment_and_Jobs_Act",
                    ),
                    (
                        "NTIA — BEAD Program",
                        "https://broadbandusa.ntia.gov/funding-programs/broadband-equity-access-and-deployment-bead-program",
                    ),
                ],
            },
        ],
        diagram: funding_diagram,
    },
];

/// Total cross-section: cladding over core over cladding, a ray
/// zig-zagging down the core, the angle at one bounce, and an end-on
/// view of core, cladding, and coating.
fn tir_diagram() -> AnyView {
    view! {
        <figure class="fig fig-slim fn-fig">
            <svg viewBox="0 0 360 264" role="img" aria-label="Diagram of total internal reflection: light zig-zags down a fiber core, reflecting off the core-cladding boundary each time it strikes at more than the critical angle; an end-on view shows the core inside the cladding inside the coating">
                <title>"Total internal reflection in an optical fiber"</title>
                <desc>"A side view of a fiber: a core band with a higher refractive index n1 between two cladding bands with a lower index n2. A ray of light enters at the left and zig-zags to the right, reflecting at each core-cladding boundary. At one reflection the angle theta between the ray and the boundary's normal is marked; because theta is greater than the critical angle, the light reflects and stays in the core. Below, an end-on view shows a small core inside the cladding inside a protective coating; a single-mode core is about 9 micrometers across."</desc>
                <defs>
                    <marker id="tir-arr" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto-start-reverse">
                        <path d="M0 0 L7 3 L0 6 z" class="mk-arr"></path>
                    </marker>
                </defs>
                <rect x="84" y="20" width="272" height="44" class="box"></rect>
                <rect x="84" y="64" width="272" height="56" class="box-raise"></rect>
                <rect x="84" y="120" width="272" height="44" class="box"></rect>
                <text x="2" y="38" class="f-mono f-dim" font-size="12">"CLADDING"</text>
                <text x="2" y="55" class="f-dim" font-size="13">"index n₂"</text>
                <text x="2" y="88" class="f-mono f-brass" font-size="12">"CORE"</text>
                <text x="2" y="106" class="f-dim" font-size="13">"n₁ > n₂"</text>
                <text x="2" y="138" class="f-mono f-dim" font-size="12">"CLADDING"</text>
                <text x="2" y="155" class="f-dim" font-size="13">"index n₂"</text>
                <path d="M84 100 L124 64 L188 120 L252 64 L316 120 L356 85" class="ln-arrow ln-flow" marker-end="url(#tir-arr)"></path>
                <line x1="188" y1="120" x2="188" y2="84" class="grid fn-dash"></line>
                <path d="M188 104 A16 16 0 0 0 176 109.5" class="ln-axis"></path>
                <text x="164" y="100" class="f-brass" font-size="14">"θ"</text>
                <text x="84" y="186" font-size="13">"θ greater than the critical angle θc:"</text>
                <text x="84" y="204" class="f-dim" font-size="13">"the light reflects and stays in the core."</text>
                <circle cx="40" cy="238" r="20" class="box"></circle>
                <circle cx="40" cy="238" r="14" class="box-raise"></circle>
                <circle cx="40" cy="238" r="4" class="acc"></circle>
                <text x="72" y="232" font-size="13">"End-on: core in cladding in coating"</text>
                <text x="72" y="250" class="f-dim" font-size="13">"single-mode core ≈ 9 µm (not to scale)"</text>
            </svg>
            <figcaption>
                "Why the light stays in the glass: the core's refractive index is higher than the "
                "cladding's, so light meeting the boundary at more than the critical angle reflects "
                "back in, over and over, for kilometers. The critical angle is in the glossary below."
            </figcaption>
        </figure>
    }
    .into_any()
}

/// Two link styles on one page: TAT-8's optical-electrical-optical
/// regenerators, and the amplified link that replaced them.
fn repeater_diagram() -> AnyView {
    view! {
        <figure class="fig fig-slim fn-fig">
            <svg viewBox="0 0 360 208" role="img" aria-label="Diagram comparing two long links: a 1988 regenerated link whose repeaters convert light to electricity and back, and an amplified link whose erbium-doped fiber amplifiers boost the light without conversion">
                <title>"Regenerated versus amplified links"</title>
                <desc>"Top row, 1988, TAT-8: a transmitter, fiber, two regenerator boxes marked O to E to O, more fiber, and a receiver; every 40 kilometers or so the light is converted to electricity and back. Bottom row, the amplified link of the 1990s: a transmitter, fiber, two triangular amplifier symbols marked EDFA, and a receiver; the light is boosted as light, with no conversion."</desc>
                <text x="2" y="18" class="f-mono f-brass" font-size="12">"1988 · TAT-8 · REGENERATED"</text>
                <rect x="2" y="32" width="38" height="32" rx="4" class="box"></rect>
                <text x="21" y="53" class="f-mid f-mono" font-size="12">"TX"</text>
                <line x1="40" y1="48" x2="92" y2="48" class="ln-arrow"></line>
                <rect x="92" y="32" width="72" height="32" rx="4" class="box-raise slab"></rect>
                <text x="128" y="53" class="f-mid f-mono" font-size="12">"O→E→O"</text>
                <line x1="164" y1="48" x2="200" y2="48" class="ln-arrow"></line>
                <rect x="200" y="32" width="72" height="32" rx="4" class="box-raise slab"></rect>
                <text x="236" y="53" class="f-mid f-mono" font-size="12">"O→E→O"</text>
                <line x1="272" y1="48" x2="318" y2="48" class="ln-arrow"></line>
                <rect x="318" y="32" width="38" height="32" rx="4" class="box"></rect>
                <text x="337" y="53" class="f-mid f-mono" font-size="12">"RX"</text>
                <text x="2" y="86" class="f-dim" font-size="13">"Every ~40 km: light → electricity → light"</text>
                <text x="2" y="122" class="f-mono f-brass" font-size="12">"1990s · AMPLIFIED (EDFA)"</text>
                <rect x="2" y="136" width="38" height="32" rx="4" class="box"></rect>
                <text x="21" y="157" class="f-mid f-mono" font-size="12">"TX"</text>
                <line x1="40" y1="152" x2="112" y2="152" class="ln-arrow"></line>
                <polygon points="112,134 112,170 148,152" class="box-raise agent"></polygon>
                <line x1="148" y1="152" x2="220" y2="152" class="ln-arrow"></line>
                <polygon points="220,134 220,170 256,152" class="box-raise agent"></polygon>
                <line x1="256" y1="152" x2="318" y2="152" class="ln-arrow"></line>
                <rect x="318" y="136" width="38" height="32" rx="4" class="box"></rect>
                <text x="337" y="157" class="f-mid f-mono" font-size="12">"RX"</text>
                <text x="130" y="186" class="f-mid f-mono f-cher" font-size="12">"EDFA"</text>
                <text x="238" y="186" class="f-mid f-mono f-cher" font-size="12">"EDFA"</text>
                <text x="2" y="204" class="f-dim" font-size="13">"Light boosted as light — no conversion"</text>
            </svg>
            <figcaption>
                "TAT-8 still regenerated its signal electronically about every 40 km. The erbium-doped "
                "fiber amplifier, demonstrated in 1986–87, boosts the light inside a length of fiber "
                "instead — the change that let the next generation of cables carry many wavelengths at "
                "once."
            </figcaption>
        </figure>
    }
    .into_any()
}

/// Capacity over time on a log axis: four verified systems from 1977 to
/// 2018, each on the timeline.
fn capacity_diagram() -> AnyView {
    view! {
        <figure class="fig fig-slim fn-fig">
            <svg viewBox="0 0 360 264" role="img" aria-label="Log-scale chart of fiber capacity from 1977 to 2018: 6 megabits per second in 1977, 280 megabits per second per fiber on TAT-8 in 1988, 5 gigabits per second per fiber pair on TAT-12/13 in 1996, and 20 terabits per second per fiber pair on MAREA in 2018">
                <title>"Capacity over time, log scale"</title>
                <desc>"A line chart with a logarithmic vertical axis from 1 megabit per second to 100 terabits per second and a horizontal axis from 1975 to 2020. Four points rise steeply: 1977, GTE's first live system, 6 Mb/s; 1988, TAT-8, 280 Mb/s per fiber; 1996, TAT-12/13, 5 Gb/s per fiber pair; 2018, MAREA, 20 Tb/s per fiber pair by design. Each gridline is a factor of ten."</desc>
                <text x="60" y="14" class="f-dim" font-size="12">"Capacity per fiber pair · log scale"</text>
                <line x1="60" y1="28" x2="350" y2="28" class="grid"></line>
                <line x1="60" y1="54" x2="350" y2="54" class="grid fn-faint"></line>
                <line x1="60" y1="80" x2="350" y2="80" class="grid"></line>
                <line x1="60" y1="106" x2="350" y2="106" class="grid fn-faint"></line>
                <line x1="60" y1="132" x2="350" y2="132" class="grid fn-faint"></line>
                <line x1="60" y1="158" x2="350" y2="158" class="grid"></line>
                <line x1="60" y1="184" x2="350" y2="184" class="grid fn-faint"></line>
                <line x1="60" y1="210" x2="350" y2="210" class="grid fn-faint"></line>
                <line x1="60" y1="236" x2="350" y2="236" class="ln-axis"></line>
                <text x="0" y="32" class="f-mono f-dim" font-size="12">"100 Tb/s"</text>
                <text x="0" y="84" class="f-mono f-dim" font-size="12">"1 Tb/s"</text>
                <text x="0" y="162" class="f-mono f-dim" font-size="12">"1 Gb/s"</text>
                <text x="0" y="240" class="f-mono f-dim" font-size="12">"1 Mb/s"</text>
                <text x="92" y="254" class="f-mid f-mono f-dim" font-size="12">"1980"</text>
                <text x="157" y="254" class="f-mid f-mono f-dim" font-size="12">"1990"</text>
                <text x="221" y="254" class="f-mid f-mono f-dim" font-size="12">"2000"</text>
                <text x="286" y="254" class="f-mid f-mono f-dim" font-size="12">"2010"</text>
                <text x="350" y="254" class="f-end f-mono f-dim" font-size="12">"2020"</text>
                <polyline points="73,216 144,172 195,140 337,46" class="ln-arrow"></polyline>
                <circle cx="73" cy="216" r="4.5" class="node"></circle>
                <circle cx="144" cy="172" r="4.5" class="node"></circle>
                <circle cx="195" cy="140" r="4.5" class="node"></circle>
                <circle cx="337" cy="46" r="5" class="node-now"></circle>
                <text x="80" y="230" font-size="12.5">"1977 GTE · 6 Mb/s"</text>
                <text x="150" y="190" font-size="12.5">"1988 TAT-8 · 280 Mb/s"</text>
                <text x="201" y="158" font-size="12.5">"1996 TAT-12/13 · 5 Gb/s"</text>
                <text x="328" y="40" class="f-end" font-size="12.5">"2018 MAREA · 20 Tb/s"</text>
            </svg>
            <figcaption>
                "Each gridline is a factor of ten. From GTE's 6 Mb/s link in 1977 to MAREA's design "
                "capacity of 20 Tb/s per fiber pair is a factor of more than three million in about "
                "forty years. The 1977 figure is the whole link; the others are per fiber or fiber pair, "
                "as their sources state."
            </figcaption>
        </figure>
    }
    .into_any()
}

/// The PON split tree: OLT, feeder, passive splitter, drops, ONTs.
fn pon_diagram() -> AnyView {
    view! {
        <figure class="fig fig-slim fn-fig">
            <svg viewBox="0 0 360 290" role="img" aria-label="Diagram of a passive optical network: an OLT in the central office sends light down one feeder fiber to an unpowered 1 to N splitter, which feeds drop fibers to an ONT at each home">
                <title>"A passive optical network split tree"</title>
                <desc>"At the left, an OLT box in the central office. One feeder fiber runs right to a passive splitter marked 1:N, which needs no power. From the splitter, drop fibers fan out to four ONT boxes, one per home, with more homes implied. Notes: on GPON, downstream light is at 1490 nanometers and upstream at 1310; a 1:32 split has an ideal loss of 10 log10 of 32, about 15.1 dB."</desc>
                <text x="2" y="18" class="f-cher" font-size="12.5">"Down 1490 nm →"</text>
                <text x="2" y="36" class="f-cher" font-size="12.5">"← Up 1310 nm (GPON)"</text>
                <rect x="2" y="120" width="66" height="48" rx="4" class="box agent"></rect>
                <text x="35" y="142" class="f-mid f-name" font-size="15">"OLT"</text>
                <text x="35" y="159" class="f-mid f-dim" font-size="12">"office"</text>
                <line x1="68" y1="144" x2="150" y2="144" class="ln-arrow"></line>
                <text x="109" y="134" class="f-mid f-dim" font-size="12.5">"feeder"</text>
                <rect x="150" y="124" width="40" height="40" rx="4" class="box-raise slab"></rect>
                <text x="170" y="149" class="f-mid f-mono" font-size="13">"1:N"</text>
                <text x="170" y="184" class="f-mid f-dim" font-size="12.5">"passive"</text>
                <text x="170" y="200" class="f-mid f-dim" font-size="12.5">"splitter"</text>
                <line x1="190" y1="144" x2="276" y2="36" class="ln-arrow"></line>
                <line x1="190" y1="144" x2="276" y2="98" class="ln-arrow"></line>
                <line x1="190" y1="144" x2="276" y2="190" class="ln-arrow"></line>
                <line x1="190" y1="144" x2="276" y2="252" class="ln-arrow"></line>
                <rect x="276" y="19" width="80" height="34" rx="4" class="box"></rect>
                <text x="316" y="41" class="f-mid" font-size="13">"ONT · home"</text>
                <rect x="276" y="81" width="80" height="34" rx="4" class="box"></rect>
                <text x="316" y="103" class="f-mid" font-size="13">"ONT · home"</text>
                <text x="312" y="150" class="f-mid f-dim" font-size="13">"+ more homes"</text>
                <rect x="276" y="173" width="80" height="34" rx="4" class="box"></rect>
                <text x="316" y="195" class="f-mid" font-size="13">"ONT · home"</text>
                <rect x="276" y="235" width="80" height="34" rx="4" class="box"></rect>
                <text x="316" y="257" class="f-mid" font-size="13">"ONT · home"</text>
                <text x="2" y="252" class="f-dim" font-size="12.5">"1:32 split, ideal loss:"</text>
                <text x="2" y="270" class="f-brass" font-size="12.5">"10·log₁₀(32) ≈ 15.1 dB"</text>
            </svg>
            <figcaption>
                "One fiber from the office serves many homes. The splitter is just glass — no power, "
                "nothing to fail in a storm — and every split costs light: dividing it 32 ways costs "
                "about 15.1 dB before any real-world excess loss, which is why the splitter dominates a "
                "PON light budget."
            </figcaption>
        </figure>
    }
    .into_any()
}

/// The two federal programs on one dollar axis.
fn funding_diagram() -> AnyView {
    view! {
        <figure class="fig fig-slim fn-fig">
            <svg viewBox="0 0 360 170" role="img" aria-label="Bar chart: RDOF Phase I awarded 9.2 billion dollars in 2020; the BEAD program was funded at 42.45 billion dollars in 2021">
                <title>"Federal broadband programs, in billions of dollars"</title>
                <desc>"Two horizontal bars on a 0 to 45 billion dollar axis. RDOF Phase I, 2020: 9.2 billion dollars awarded to 180 winning bidders. BEAD, created by the Infrastructure Investment and Jobs Act in 2021: 42.45 billion dollars."</desc>
                <text x="8" y="16" class="f-dim" font-size="12.5">"Billions of dollars"</text>
                <line x1="8" y1="28" x2="8" y2="134" class="ln-axis"></line>
                <line x1="84.4" y1="28" x2="84.4" y2="134" class="grid"></line>
                <line x1="160.9" y1="28" x2="160.9" y2="134" class="grid"></line>
                <line x1="237.3" y1="28" x2="237.3" y2="134" class="grid"></line>
                <line x1="313.8" y1="28" x2="313.8" y2="134" class="grid"></line>
                <rect x="8" y="40" width="70.3" height="28" class="seg seg-d"></rect>
                <text x="86" y="52" font-size="13">"RDOF Phase I · $9.2B"</text>
                <text x="86" y="68" class="f-dim" font-size="12">"awarded, Nov 2020"</text>
                <rect x="8" y="94" width="324.5" height="28" class="seg seg-g"></rect>
                <text x="16" y="113" font-size="13">"BEAD · $42.45B (IIJA, Nov 2021)"</text>
                <text x="8" y="152" class="f-mid f-mono f-dim" font-size="12">"0"</text>
                <text x="84.4" y="152" class="f-mid f-mono f-dim" font-size="12">"10"</text>
                <text x="160.9" y="152" class="f-mid f-mono f-dim" font-size="12">"20"</text>
                <text x="237.3" y="152" class="f-mid f-mono f-dim" font-size="12">"30"</text>
                <text x="313.8" y="152" class="f-mid f-mono f-dim" font-size="12">"40"</text>
            </svg>
            <figcaption>
                "Dollars committed, not dollars spent, and not all of it fiber: RDOF's $9.2 billion is "
                "paid out over ten years; BEAD's $42.45 billion was allocated to the states and "
                "territories in June 2023."
            </figcaption>
        </figure>
    }
    .into_any()
}

fn era_panel(index: usize, era: &'static Era, current: RwSignal<usize>, enhanced: RwSignal<bool>) -> impl IntoView {
    view! {
        <section
            class="fn-era"
            id=format!("era-{}", era.key)
            hidden=move || enhanced.get() && current.get() != index
        >
            <h3 class="fn-era-title">
                {era.title} " " <span class="fn-era-span">{era.span}</span>
            </h3>
            <p class="fn-era-intro">{era.intro}</p>
            {(era.diagram)()}
            <ol class="fn-milestones">
                {era
                    .milestones
                    .iter()
                    .map(|m| {
                        view! {
                            <li class="tl-entry fn-milestone">
                                <p class="tl-meta">
                                    <time class="tl-date" datetime=m.date>{m.display}</time>
                                </p>
                                <h4 class="tl-title fn-ms-title">{m.title}</h4>
                                <p class="tl-body">{m.body}</p>
                                <p class="tl-source">
                                    "Sources: "
                                    {m
                                        .sources
                                        .iter()
                                        .enumerate()
                                        .map(|(i, &(label, url))| {
                                            view! {
                                                {(i > 0).then_some(" · ")}
                                                <a href=url rel="external">{label}</a>
                                            }
                                        })
                                        .collect_view()}
                                </p>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </section>
    }
}

/// Move the selected era by `delta`, clamped to the timeline.
fn step(current: RwSignal<usize>, delta: isize) {
    current.update(|i| *i = i.saturating_add_signed(delta).min(ERAS.len() - 1));
}

#[component]
pub fn FiberTimeline() -> impl IntoView {
    let current = RwSignal::new(0_usize);
    let enhanced = RwSignal::new(false);
    Effect::new(move |_| enhanced.set(true));
    let on_key = move |ev: KeyboardEvent| match ev.key().as_str() {
        "ArrowRight" | "ArrowDown" => {
            ev.prevent_default();
            step(current, 1);
        }
        "ArrowLeft" | "ArrowUp" => {
            ev.prevent_default();
            step(current, -1);
        }
        "Home" => {
            ev.prevent_default();
            current.set(0);
        }
        "End" => {
            ev.prevent_default();
            current.set(ERAS.len() - 1);
        }
        _ => {}
    };
    let status = move || {
        let i = current.get();
        ERAS.get(i).map(|era| format!("Era {} of {}: {}, {}", i + 1, ERAS.len(), era.title, era.span))
    };
    view! {
        <section class="study-section fn-timeline" id="timeline">
            <h2 class="band-label">"FIBER IN THE UNITED STATES — A TIMELINE"</h2>
            <p>
                "Fifty-odd years from a strand of Corning glass to a federal buildout: sixteen "
                "milestones in five eras. Every date was checked against the sources each entry "
                "links."
            </p>
            <div class="fn-era-nav" hidden=move || !enhanced.get() on:keydown=on_key>
                <div class="fn-era-tabs" role="group" aria-label="Choose an era (arrow keys move between eras)">
                    {ERAS
                        .iter()
                        .enumerate()
                        .map(|(i, era)| {
                            view! {
                                <button
                                    type="button"
                                    class="tl-filter"
                                    aria-pressed=move || (current.get() == i).to_string()
                                    aria-controls=format!("era-{}", era.key)
                                    on:click=move |_| current.set(i)
                                >
                                    {era.span}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
                <div class="fn-era-step">
                    <button
                        type="button"
                        class="ft-next"
                        disabled=move || current.get() == 0
                        on:click=move |_| step(current, -1)
                    >
                        "← Earlier"
                    </button>
                    <p class="fn-era-status" aria-live="polite">{status}</p>
                    <button
                        type="button"
                        class="ft-next"
                        disabled=move || current.get() + 1 == ERAS.len()
                        on:click=move |_| step(current, 1)
                    >
                        "Later →"
                    </button>
                </div>
            </div>
            {ERAS.iter().enumerate().map(|(i, era)| era_panel(i, era, current, enhanced)).collect_view()}
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::{Era, ERAS};

    #[test]
    fn sixteen_milestones_in_chronological_order() {
        let all: Vec<&str> = ERAS.iter().flat_map(|e: &Era| e.milestones.iter().map(|m| m.date)).collect();
        assert_eq!(all.len(), 16);
        for pair in all.windows(2) {
            let [a, b] = pair else { continue };
            assert!(a <= b, "{a} after {b}");
        }
    }

    #[test]
    fn every_milestone_cites_an_https_source() {
        for era in ERAS {
            for m in era.milestones {
                assert!(!m.sources.is_empty(), "{}", m.title);
                assert!(m.sources.iter().all(|(_, url)| url.starts_with("https://")), "{}", m.title);
            }
        }
    }
}
