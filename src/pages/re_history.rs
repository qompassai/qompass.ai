//! “The RE toolchain: Ghidra, IDA, Hopper” — an animated timeline
//! of the three reverse-engineering engines fathom's documentation
//! tracks, linked from fathom's MCP page.
//!
//! Every dated claim is cited inline to its source: NSA's own
//! publications and the NSA's GitHub organization, Hex-Rays' site
//! and documentation, Hopper's site, and contemporaneous reporting
//! where the primary record is thin (marked as such). Researched
//! 2026-10-20. Motion lives entirely in CSS under
//! `prefers-reduced-motion: no-preference`; with reduced motion the
//! page is a plain, complete document.

use leptos::prelude::*;
use leptos_meta::Title;

/// One dated entry on one tool's timeline.
struct Entry {
    year: &'static str,
    title: &'static str,
    body: &'static str,
    cite: &'static str,
    href: &'static str,
}

const fn entry(
    year: &'static str,
    title: &'static str,
    body: &'static str,
    cite: &'static str,
    href: &'static str,
) -> Entry {
    Entry { year, title, body, cite, href }
}

const GHIDRA: &[Entry] = &[
    entry(
        "Before 2017",
        "Built inside the NSA",
        "Ghidra is a software reverse engineering framework created by the National \
         Security Agency's Research Directorate. NSA built it to solve scaling and \
         teaming problems on complex analysis efforts and applied it to analyzing \
         malicious code and hunting vulnerabilities in networks and systems — an \
         internal instrument, built for the agency's own missions.",
        "NationalSecurityAgency/ghidra, README",
        "https://github.com/NationalSecurityAgency/ghidra",
    ),
    entry(
        "March 2017",
        "The name leaks",
        "The first public trace of the tool is a leak: the CIA's Vault 7 documents, \
         published by WikiLeaks in March 2017, referenced an NSA reverse-engineering \
         framework called GHIDRA. The reporting at the time treated it as an \
         unannounced counterpart to IDA Pro.",
        "BleepingComputer, March 2019 (reporting the Vault 7 references)",
        "https://www.bleepingcomputer.com/news/security/nsa-releasing-the-ghidra-reverse-engineering-tool-at-rsaconference/",
    ),
    entry(
        "5 March 2019",
        "Released at RSA, in the open",
        "NSA releases Ghidra at the 2019 RSA Conference. The agency's own release \
         story describes what it shipped: the Sleigh processor language, which \
         specifies how machine code is disassembled and lifted into the intermediate \
         representation called P-code; a C++ decompiler; undo/redo; a multi-user \
         collaboration repository; and scripting — free, for anyone.",
        "NSA, “Ghidra … is Here!”, 5 March 2019",
        "https://www.nsa.gov/DesktopModules/ArticleCS/Print.aspx?PortalId=70&ModuleId=10970&Article=1775584",
    ),
    entry(
        "April 2019",
        "The source follows",
        "The binaries came first, at RSA; the complete source code was published on \
         GitHub about a month later, under the Apache License 2.0. The repository \
         itself was created on GitHub on 1 March 2019, days before the announcement.",
        "Wikipedia, “Ghidra”; repository record, GitHub",
        "https://en.wikipedia.org/wiki/Ghidra",
    ),
];

const IDA: &[Entry] = &[
    entry(
        "1990–1991",
        "One programmer, in Moscow",
        "Ilfak Guilfanov begins writing the Interactive Disassembler in late 1990 \
         and releases version 0.1 on 21 May 1991. From 1994 it spreads the way \
         software spread then: as shareware, over FidoNet and dial-up bulletin \
         boards.",
        "Profile: Ilfak Guilfanov, Threat Picture",
        "https://threatpicture.com/people/ilfak-guilfanov/",
    ),
    entry(
        "1996",
        "DataRescue takes it commercial",
        "A Belgian company, DataRescue of Liège, begins selling the tool as IDA Pro; \
         Guilfanov moves to Belgium and works there as a senior developer. The \
         interactive disassembler becomes the profession's default instrument.",
        "Profile: Ilfak Guilfanov, Threat Picture",
        "https://threatpicture.com/people/ilfak-guilfanov/",
    ),
    entry(
        "2005",
        "Hex-Rays is founded",
        "Guilfanov co-founds Hex-Rays in 2005 to turn the work into a company. The \
         same year ends with his other famous act: on 31 December 2005 he publishes \
         an unofficial hotfix for the Windows Metafile flaw, and the security \
         industry recommends his patch ahead of Microsoft's own.",
        "Hex-Rays, “About us”",
        "http://hex-rays.com/about-us",
    ),
    entry(
        "17 September 2007",
        "The decompiler",
        "Hex-Rays Decompiler 1.0 is released. Disassembly becomes pseudocode: IDA \
         stops merely showing the analyst machine code and starts reconstructing \
         something close to the source. It is the feature the next two decades of \
         reverse engineering are built on.",
        "Hex-Rays, “IDA: celebrating 30 years”",
        "https://hex-rays.com/blog/ida-celebrating-30-years-of-binary-analysis-innovation",
    ),
    entry(
        "1 January 2008",
        "Hex-Rays takes over IDA",
        "Development of IDA itself moves from DataRescue to Hex-Rays SA. The \
         timeline that follows is steady, versioned progress: a Qt cross-platform \
         GUI in 6.0 (2010), a native 64-bit IDA in 7.0 (2017), the Lumina function \
         database in 7.2 (2018).",
        "Hex-Rays, “IDA: celebrating 30 years”",
        "https://hex-rays.com/blog/ida-celebrating-30-years-of-binary-analysis-innovation",
    ),
    entry(
        "End of 2022",
        "Investors join",
        "A consortium led by Smartfin, with the Belgian sovereign fund SFPIM and \
         Wallonie Entreprendre, joins Hex-Rays; Smartsec SA becomes the parent \
         company. Guilfanov and the employees remain the largest shareholders, and \
         he stays on as founder and CTO.",
        "Hex-Rays, “About us”",
        "http://hex-rays.com/about-us",
    ),
    entry(
        "30 September 2024",
        "IDA 9.0, and subscriptions",
        "IDA 9.0 (build 240925) ships with idalib — headless processing that lets \
         programs drive IDA's engine directly — and from 1 October 2024 every \
         Hex-Rays product moves to yearly subscription licensing, ending the \
         perpetual era.",
        "Hex-Rays documentation, IDA 9.0 release notes; Hex-Rays licensing FAQ",
        "https://docs.hex-rays.com/9.1/release-notes/9_0",
    ),
];

const HOPPER: &[Entry] = &[
    entry(
        "By 2012",
        "A Mac-native disassembler",
        "Hopper, written by Vincent Benony and published by his company Crypto \
         Software, is already a going concern: a January 2012 review walks through \
         its annotation, its control-flow handling, and its decompiler producing \
         C-like pseudocode — on Mac OS X, as a true Mac application, at a price an \
         individual could expense.",
        "Mike Ash, “Friday Q&A”, 6 January 2012; hopperapp.com",
        "http://www.mikeash.com/pyblog/friday-qa-2012-01-06-the-hopper-disassembler.html",
    ),
    entry(
        "Design center",
        "The Mac is the point",
        "Where IDA is a cross-platform workbench and Ghidra a framework, Hopper's \
         identity is platform craft: a Cocoa interface on macOS (Qt on Linux), \
         deep Objective-C and Swift support — mangled-name decoding, selectors, \
         message sends — and an inspector that adapts to context. It was built for \
         the Apple ecosystem's analysts first.",
        "hopperapp.com",
        "https://www.hopperapp.com/",
    ),
    entry(
        "Today",
        "Hopper v6",
        "The current line keeps the formula and extends it: control-flow-graph \
         editing, LLDB and GDB debugging, Python scripting, an extensible SDK — \
         and, closing a loop with the rest of this site, an integrated MCP server \
         so an analyst's language model can work inside the tool.",
        "hopperapp.com",
        "https://www.hopperapp.com/",
    ),
];

/// One lane of the timeline.
#[component]
fn Lane(name: &'static str, tagline: &'static str, entries: &'static [Entry]) -> impl IntoView {
    view! {
        <h2 class="re-lane-title">{name}</h2>
        <p class="re-lane-tag">{tagline}</p>
        <ol class="re-timeline">
            {entries
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    let delay = format!("--re-delay: {}ms", i * 140);
                    view! {
                        <li class="re-entry" style=delay>
                            <span class="re-year">{e.year}</span>
                            <h3>{e.title}</h3>
                            <p>
                                {e.body} " "
                                <a class="re-cite" href=e.href>
                                    {format!("[{}]", e.cite)}
                                </a>
                            </p>
                        </li>
                    }
                })
                .collect_view()}
        </ol>
    }
}

/// The animated RE-toolchain history page.
#[component]
pub fn ReHistory() -> impl IntoView {
    view! {
        <Title text="The RE Toolchain: Ghidra, IDA, Hopper - Qompass AI" />
        <section class="page re-page">
            <p class="page-bearing">"CHART — THE FLEET / MCP / FATHOM"</p>
            <h1>"The RE toolchain: Ghidra, IDA, Hopper"</h1>
            <p class="page-lede">
                "Three engines, three ownership models: a state framework given away, a \
                 commercial cathedral built by one programmer, and an indie instrument \
                 crafted for one platform. Fathom works across all three — Ghidra headless \
                 today, IDA and Hopper on its roadmap — so their histories are part of its \
                 documentation."
            </p>
            <svg
                class="re-sounding"
                viewBox="0 0 600 60"
                preserveAspectRatio="none"
                aria-hidden="true"
            >
                <polyline
                    points="0,8 60,8 75,30 90,8 180,8 195,44 210,8 300,8 315,24 330,8 420,8 435,52 450,8 600,8"
                    fill="none"
                />
            </svg>
            <Lane
                name="Ghidra"
                tagline="The state's framework, given away — NSA, open source, Apache-2.0."
                entries=GHIDRA
            />
            <aside class="re-panel">
                <h2>"How involved is the NSA today?"</h2>
                <p>
                    "Directly — Ghidra is an NSA program that happens to be open source, not a \
                     donation the agency walked away from. The evidence, checked 2026-10-20:"
                </p>
                <ul>
                    <li>
                        "The canonical repository lives in the NSA's own GitHub organization, \
                         and its README opens by calling Ghidra a framework “created and \
                         maintained by the National Security Agency Research Directorate.”"
                    </li>
                    <li>
                        "The repository is active, not archived: 56 releases and more than \
                         18,800 commits, with the current line at 12.1 (12.1.4, August 2026)."
                    </li>
                    <li>
                        "NSA presents the work as its own, in its own voice: its Cybersecurity \
                         Speaker Series features a member of Ghidra's development team \
                         discussing the release decision and how the software “continues to \
                         evolve for the Ghidra team.”"
                    </li>
                    <li>
                        "The contribution model is the public one — issues, pull requests, a \
                         Contributor's Guide — with the agency as upstream maintainer. The \
                         README still recruits for the mission directly: U.S. citizens are \
                         invited to apply “to develop Ghidra and other cybersecurity tools \
                         for NSA.”"
                    </li>
                </ul>
                <p class="re-cite-line">
                    "Sources: "
                    <a href="https://github.com/NationalSecurityAgency/ghidra">
                        "github.com/NationalSecurityAgency/ghidra"
                    </a>
                    " · "
                    <a href="https://www.nsa.gov/Press-Room/News-Highlights/Article/Article/2958453/cybersecurity-speaker-series-ghidra-beyond-the-code/">
                        "NSA Cybersecurity Speaker Series: Ghidra — Beyond the Code"
                    </a>
                    " · "
                    <a href="https://en.wikipedia.org/wiki/Ghidra">"Wikipedia, “Ghidra”"</a>
                </p>
            </aside>
            <Lane
                name="IDA"
                tagline="The profession's instrument — Hex-Rays SA, Liège; commercial, subscription since 2024."
                entries=IDA
            />
            <Lane
                name="Hopper"
                tagline="The indie instrument — Vincent Benony / Crypto Software; macOS-native, commercial."
                entries=HOPPER
            />
            <p class="re-close">
                "Back to "
                <a href="/solutions/mcp/fathom">"fathom's tool documentation"</a>
                " — where these three histories become a build order."
            </p>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_entry_is_cited() {
        for e in GHIDRA.iter().chain(IDA).chain(HOPPER) {
            assert!(!e.cite.is_empty() && e.href.starts_with("http"), "{}", e.title);
            assert!(!e.year.is_empty() && !e.body.is_empty(), "{}", e.title);
        }
        assert_eq!(GHIDRA.len(), 4);
        assert_eq!(IDA.len(), 7);
        assert_eq!(HOPPER.len(), 3);
    }
}
