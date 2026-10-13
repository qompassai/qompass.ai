//! "On AI" — a compressed timeline from `ChatGPT` to `Agent2Agent`.
//!
//! Three tracks: Protocols (ported faithfully from Matt's Daelin
//! timeline, github.com/qompassai/daelin — categories mcp/a2a/adj map
//! to Protocols, gov entries carry a Governance tag), Models &
//! Companies, and Law & Policy (dates verified against primary
//! sources: official journals, the Federal Register, EUR-Lex, company
//! announcements). The timeline is fully server-rendered; the filter
//! buttons in `/js/on-ai.js` are an enhancement only.
use leptos::prelude::*;
use leptos_meta::Title;

#[derive(Clone, Copy, PartialEq)]
enum Track {
    Law,
    Models,
    Protocols,
}

impl Track {
    fn chip(self) -> &'static str {
        match self {
            Self::Law => "LAW & POLICY",
            Self::Models => "MODELS & COMPANIES",
            Self::Protocols => "PROTOCOLS",
        }
    }
    fn class(self) -> &'static str {
        match self {
            Self::Law => "law",
            Self::Models => "models",
            Self::Protocols => "protocols",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Self::Law => "law",
            Self::Models => "models",
            Self::Protocols => "protocols",
        }
    }
}

struct Entry {
    body: &'static str,
    date: &'static str,
    display: &'static str,
    flag: &'static str,
    gov: bool,
    source: &'static str,
    title: &'static str,
    track: Track,
    year: &'static str,
}

const fn entry(
    year: &'static str,
    date: &'static str,
    display: &'static str,
    track: Track,
    title: &'static str,
    body: &'static str,
    source: &'static str,
) -> Entry {
    Entry { body, date, display, flag: "", gov: false, source, title, track, year }
}

const fn gov_entry(
    year: &'static str,
    date: &'static str,
    display: &'static str,
    title: &'static str,
    body: &'static str,
    source: &'static str,
) -> Entry {
    Entry { body, date, display, flag: "", gov: true, source, title, track: Track::Protocols, year }
}

const fn flagged(mut e: Entry, flag: &'static str) -> Entry {
    e.flag = flag;
    e
}

/// The Daelin spine: MCP / A2A / adjacent / governance entries from
/// qompassai/daelin's timeline.html, titles, dates, and bodies ported
/// accurately; [secondary]/[disputed] honesty markers kept.
const PROTOCOLS: &[Entry] = &[
    entry(
        "2024",
        "2024-11-05",
        "Nov 5, 2024",
        Track::Protocols,
        "MCP spec first published (v2024-11-05)",
        "JSON-RPC 2.0 protocol with hosts, clients, and servers. Server features: tools, resources, prompts. Client features: roots, sampling. Transports: stdio and HTTP+SSE.",
        "https://modelcontextprotocol.io/specification/2025-03-26/changelog",
    ),
    flagged(
        entry(
            "2024",
            "2024-11-25",
            "Nov 25, 2024",
            Track::Protocols,
            "Anthropic open-sources MCP",
            "The spec plus Python/TypeScript SDKs, local server support in Claude Desktop, and an open-source server repo (Google Drive, Slack, GitHub, Git, Postgres, Puppeteer). Created by David Soria Parra and Justin Spahr-Summers. Early adopters: Block, Apollo.",
            "https://www.anthropic.com/news/model-context-protocol",
        ),
        "[secondary] date per corroborating sources",
    ),
    entry(
        "2024",
        "2024-12-19",
        "Dec 19, 2024",
        Track::Protocols,
        "Anthropic publishes \u{201c}Building effective agents\u{201d}",
        "The workflows-vs-agents taxonomy (prompt chaining, routing, parallelization, orchestrator-workers, evaluator-optimizer, autonomous agents). Core advice: simplest solution first, sandboxed testing, human feedback at checkpoints.",
        "https://www.anthropic.com/engineering/building-effective-agents",
    ),
    flagged(
        entry(
            "2025",
            "2025-01",
            "Jan 2025",
            Track::Protocols,
            "OpenAI publishes \u{201c}A Practical Guide to Building Agents\u{201d}",
            "Agent = model + tools + instructions. Single- vs multi-agent orchestration, tool design, guardrails, and the line everyone quotes: set up evals to establish a performance baseline.",
            "https://cdn.openai.com/business-guides-and-resources/a-practical-guide-to-building-agents.pdf",
        ),
        "[secondary] PDF undated; Jan 2025 per coverage",
    ),
    entry(
        "2025",
        "2025-03-26",
        "Mar 26, 2025",
        Track::Protocols,
        "MCP spec 2025-03-26",
        "OAuth 2.1 authorization framework. HTTP+SSE replaced by the new Streamable HTTP transport. JSON-RPC batching (removed again in June). Tool annotations — read-only vs destructive hints.",
        "https://modelcontextprotocol.io/specification/2025-03-26/changelog",
    ),
    entry(
        "2025",
        "2025-04-09",
        "Apr 9, 2025",
        Track::Protocols,
        "Google announces Agent2Agent (A2A)",
        "Open protocol for agents from different vendors to collaborate. 50+ launch partners. Explicitly \u{201c}complements Anthropic's Model Context Protocol.\u{201d} Core concepts: Agent Card discovery, task lifecycle, messages, UX negotiation via parts.",
        "https://developers.googleblog.com/en/a2a-a-new-era-of-agent-interoperability/",
    ),
    entry(
        "2025",
        "2025-05-27",
        "May 27, 2025",
        Track::Protocols,
        "A2A spec v0.2.1",
        "Authenticated extended cards, referenceTaskIds. The v0.2 line announced around Google I/O.",
        "https://github.com/a2aproject/a2a/blob/HEAD/CHANGELOG.md",
    ),
    entry(
        "2025",
        "2025-06-18",
        "Jun 18, 2025",
        Track::Protocols,
        "MCP spec 2025-06-18",
        "Batching removed. Structured tool output. Servers classified as OAuth Resource Servers; clients MUST implement Resource Indicators (RFC 8707) so malicious servers can't steal tokens. New Security Best Practices page. Elicitation. MCP-Protocol-Version header.",
        "https://modelcontextprotocol.io/specification/2025-06-18/changelog",
    ),
    gov_entry(
        "2025",
        "2025-06-23",
        "Jun 23, 2025",
        "Google donates A2A to the Linux Foundation",
        "The Agent2Agent project forms with AWS, Cisco, Google, Microsoft, Salesforce, SAP, ServiceNow. Google transfers the spec, SDKs, and tooling. 100+ companies supporting.",
        "https://developers.googleblog.com/en/google-cloud-donates-a2a-to-linux-foundation/",
    ),
    entry(
        "2025",
        "2025-07-30",
        "Jul 30, 2025",
        Track::Protocols,
        "A2A spec v0.3.0 (breaking)",
        "Agent Card well-known URI changes from agent.json to agent-card.json. Signed cards. mTLS added to security schemes.",
        "https://github.com/a2aproject/a2a/blob/HEAD/CHANGELOG.md",
    ),
    flagged(
        entry(
            "2025",
            "~2025-08-25",
            "\u{2248} Aug 25, 2025",
            Track::Protocols,
            "IBM's ACP merges into A2A",
            "IBM/BeeAI's Agent Communication Protocol winds down; the repo is archived and IBM Research joins the A2A Technical Steering Committee. A2A consolidates as the de-facto agent-to-agent standard.",
            "https://github.com/a2aproject/a2a",
        ),
        "[secondary]",
    ),
    entry(
        "2025",
        "2025-09-08",
        "Sep 8, 2025",
        Track::Protocols,
        "MCP Registry launches (preview)",
        "An open catalog and API for discovering publicly available MCP servers at registry.modelcontextprotocol.io — official MCP project, open source.",
        "https://blog.modelcontextprotocol.io/posts/2025-09-08-mcp-registry-preview/",
    ),
    flagged(
        entry(
            "2025",
            "2025-09-16",
            "Sep 16, 2025",
            Track::Protocols,
            "Google announces Agent Payments Protocol (AP2)",
            "Open standard for agents to transact via cryptographically signed mandates. 60+ launch partners. Built on A2A + MCP — the commerce layer above the messaging protocols.",
            "https://www.linuxfoundation.org/press/a2a-protocol-surpasses-150-organizations-lands-in-major-cloud-platforms-and-sees-enterprise-production-use-in-first-year",
        ),
        "[disputed] some sources say 09-17 or 09-29",
    ),
    entry(
        "2025",
        "2025-11-25",
        "Nov 25, 2025",
        Track::Protocols,
        "MCP spec 2025-11-25 (\u{201c}One Year of MCP\u{201d})",
        "OIDC discovery for authorization servers. Tool/resource icons. Incremental scope consent. OAuth Client ID Metadata Documents. Experimental Tasks — durable async requests.",
        "https://modelcontextprotocol.io/specification/2025-11-25/changelog",
    ),
    flagged(
        gov_entry(
            "2025",
            "2025-12-09",
            "Dec 9, 2025",
            "Anthropic donates MCP to the Agentic AI Foundation",
            "AAIF, a Linux Foundation directed fund co-founded by Anthropic, Block, and OpenAI. MCP joins Block's goose and OpenAI's AGENTS.md as founding projects. Claimed: 10,000+ public servers, 97M+/month SDK downloads.",
            "https://www.anthropic.com/news/donating-the-model-context-protocol-and-establishing-of-the-agentic-ai-foundation",
        ),
        "[secondary] date per MCP blog",
    ),
    entry(
        "2026",
        "2026-03-12",
        "Mar 12, 2026",
        Track::Protocols,
        "A2A v1.0.0 — first stable spec",
        "Breaking vs 0.x. Protocol separated from transport bindings (JSON-RPC, REST, gRPC). Modernized OAuth 2.0. Native multi-tenancy. Signed Agent Cards. tasks/list with filtering and pagination.",
        "https://github.com/a2aproject/a2a/blob/HEAD/CHANGELOG.md",
    ),
    gov_entry(
        "2026",
        "2026-04-09",
        "Apr 9, 2026",
        "Linux Foundation: A2A one-year press release",
        "150+ organizations, 22,000+ GitHub stars, SDKs in five languages, native integration in Azure AI Foundry, Copilot Studio, Amazon Bedrock AgentCore. Restates the complementarity: A2A for agent-to-agent across boundaries, MCP for agent-to-tools-and-data.",
        "https://www.linuxfoundation.org/press/a2a-protocol-surpasses-150-organizations-lands-in-major-cloud-platforms-and-sees-enterprise-production-use-in-first-year",
    ),
    entry(
        "2026",
        "2026-05-26",
        "May 26, 2026",
        Track::Protocols,
        "A2A v1.0.1 (patch)",
        "Prefer application/a2a+json in the HTTP binding; error-mapping and TaskStatus fixes. Latest 1.x as of research date.",
        "https://github.com/a2aproject/a2a/blob/HEAD/CHANGELOG.md",
    ),
    entry(
        "2026",
        "2026-07-28",
        "Jul 28, 2026",
        Track::Protocols,
        "MCP spec 2026-07-28 — \u{201c}largest revision since launch\u{201d}",
        "Stateless core: sessions and the initialize handshake removed; every request carries version + capabilities in _meta. New server/discover RPC. ping, logging/setLevel removed. Tasks moved to an official extension. MRTR replaces sampling + elicitation. SSE resumability removed.",
        "https://modelcontextprotocol.io/specification/2026-07-28/changelog",
    ),
    gov_entry(
        "2026",
        "2026-08-17",
        "Aug 17, 2026",
        "A2A joins the Agentic AI Foundation",
        "A2A accepted as a Growth Stage project at AAIF. Both protocols now share a governance home — specs remain separate, no merger.",
        "https://a2a-protocol.org/latest/blog/2026/08/27/a-new-chapter-for-a2a-joining-the-agentic-ai-foundation/",
    ),
];

/// Models & Companies track — every date verified against a company
/// announcement or primary reporting of it (verification pass,
/// 2026-10-12). Compressed to inflection points on purpose.
const MODELS: &[Entry] = &[
    entry(
        "2022",
        "2022-11-30",
        "Nov 30, 2022",
        Track::Models,
        "ChatGPT launches",
        "OpenAI releases ChatGPT as a research preview — a conversational model the general public could simply talk to. The chatbot era starts here.",
        "https://techcrunch.com/2025/11/30/chatgpt-launched-three-years-ago-today/",
    ),
    entry(
        "2023",
        "2023-03-14",
        "Mar 14, 2023",
        Track::Models,
        "GPT-4",
        "OpenAI releases GPT-4, a multimodal model accepting text and image input, first to ChatGPT Plus subscribers and an API waitlist.",
        "https://9to5google.com/2023/03/14/openai-gpt-4-announcement/",
    ),
    entry(
        "2023",
        "2023-03-14",
        "Mar 14, 2023",
        Track::Models,
        "Claude 1",
        "The same day, Anthropic introduces Claude and Claude Instant via API and chat — a frontier competitor built with Constitutional AI and positioned on safety.",
        "https://www.anthropic.com/news/introducing-claude",
    ),
    entry(
        "2023",
        "2023-07-18",
        "Jul 18, 2023",
        Track::Models,
        "Llama 2 — open weights go mainstream",
        "Meta releases the Llama 2 family (7B, 13B, 70B) with weights available for research and commercial use. A capable open-weights model is suddenly something any developer can run.",
        "https://github.com/meta-llama/llama-models/blob/HEAD/README.md",
    ),
    entry(
        "2023",
        "2023-11-17",
        "Nov 17–22, 2023",
        Track::Models,
        "The OpenAI board crisis",
        "OpenAI's nonprofit board removes CEO Sam Altman on Nov 17; five days of employee and investor pressure later, he is reinstated on Nov 22 with a reconstituted board — the governance tension inside the frontier labs, in public.",
        "https://en.wikipedia.org/wiki/Removal_of_Sam_Altman_from_OpenAI",
    ),
    entry(
        "2023",
        "2023-12-06",
        "Dec 6, 2023",
        Track::Models,
        "Gemini 1.0",
        "Google and Google DeepMind announce Gemini 1.0 — Ultra, Pro, and Nano — a natively multimodal family; Gemini Pro reaches Bard the same day.",
        "https://www.reuters.com/technology/alphabet-unveils-long-awaited-gemini-ai-model-2023-12-06/",
    ),
    entry(
        "2024",
        "2024-03-04",
        "Mar 4, 2024",
        Track::Models,
        "Claude 3",
        "Anthropic launches the Claude 3 family — Haiku, Sonnet, Opus — its first multimodal generation, available on claude.ai and the API at launch.",
        "https://www.reuters.com/technology/anthropic-releases-more-powerful-claude-3-ai-tech-race-continues-2024-03-04/",
    ),
    entry(
        "2024",
        "2024-05-13",
        "May 13, 2024",
        Track::Models,
        "GPT-4o",
        "OpenAI announces GPT-4o (\u{201c}o\u{201d} for omni): one natively multimodal flagship across text, audio, and vision, rolled out to free and paid ChatGPT users alike.",
        "https://openai.com/index/hello-gpt-4o/",
    ),
    entry(
        "2024",
        "2024-09-12",
        "Sep 12, 2024",
        Track::Models,
        "o1-preview — the reasoning era",
        "OpenAI releases o1-preview and o1-mini, the first o-series models, designed to spend more time thinking before answering. Test-time compute becomes a product.",
        "https://openai.com/index/introducing-openai-o1-preview/",
    ),
    entry(
        "2024",
        "2024-10-22",
        "Oct 22, 2024",
        Track::Models,
        "Anthropic computer use",
        "Alongside an upgraded Claude 3.5 Sonnet, Anthropic puts computer use into public beta: Claude can look at a screen, move a cursor, click, and type. The agent era gets hands.",
        "https://www.anthropic.com/news/3-5-models-and-computer-use",
    ),
    entry(
        "2025",
        "2025-01-20",
        "Jan 20, 2025",
        Track::Models,
        "DeepSeek releases R1",
        "China's DeepSeek releases R1, an open reasoning model matching the o-series pattern at a fraction of the presumed cost.",
        "https://www.reuters.com/technology/chinas-deepseek-sets-off-ai-market-rout-2025-01-27/",
    ),
    entry(
        "2025",
        "2025-01-27",
        "Jan 27, 2025",
        Track::Models,
        "The DeepSeek market shock",
        "A week after R1, investors reprice the US AI build-out: a global tech selloff in which Nvidia loses $593 billion of market value in a day — a record one-day loss (Reuters/LSEG).",
        "https://www.reuters.com/technology/chinas-deepseek-sets-off-ai-market-rout-2025-01-27/",
    ),
    entry(
        "2025",
        "2025-01-23",
        "Jan 23, 2025",
        Track::Models,
        "OpenAI Operator",
        "OpenAI previews Operator for US ChatGPT Pro users: an agent on its Computer-Using Agent model that drives its own browser to perform web tasks.",
        "https://openai.com/index/computer-using-agent/",
    ),
    entry(
        "2025",
        "2025-05-22",
        "May 22, 2025",
        Track::Models,
        "Claude 4",
        "Anthropic introduces Claude Opus 4 and Sonnet 4 — hybrid models with near-instant and extended-thinking modes, aimed at coding, reasoning, and agent workflows; Claude Code reaches general availability.",
        "https://www.anthropic.com/news/claude-4",
    ),
    entry(
        "2025",
        "2025-08-07",
        "Aug 7, 2025",
        Track::Models,
        "GPT-5",
        "OpenAI releases GPT-5 as a unified system — the GPT series' speed with the o-series' reasoning — and makes it the default model in ChatGPT for free and paid users.",
        "https://openai.com/index/introducing-gpt-5/",
    ),
];

/// Law & Policy track.
///
/// Every date verified against the Federal Register, EUR-Lex,
/// GOV.UK, the Élysée, and state legislature records (verification
/// pass, 2026-10-12). Signing and effective dates are distinguished
/// where they differ.
const LAW: &[Entry] = &[
    entry(
        "2023",
        "2023-08-15",
        "Aug 15, 2023",
        Track::Law,
        "China's generative-AI measures take effect",
        "The Interim Measures for the Management of Generative AI Services — issued jointly by seven bodies led by the Cyberspace Administration of China and promulgated Jul 13 — take effect: the first national rules for public-facing generative AI services.",
        "http://english.www.gov.cn/news/202307/13/content_WS64aff5b3c6d0868f4e8ddc01.html",
    ),
    entry(
        "2023",
        "2023-10-30",
        "Oct 30, 2023",
        Track::Law,
        "EO 14110 — the US AI executive order",
        "President Biden signs Executive Order 14110, \u{201c}Safe, Secure, and Trustworthy Development and Use of Artificial Intelligence\u{201d} — a whole-of-government order including reporting duties for powerful dual-use foundation models (88 FR 75191).",
        "https://www.federalregister.gov/documents/2023/11/01/2023-24283/safe-secure-and-trustworthy-development-and-use-of-artificial-intelligence",
    ),
    entry(
        "2023",
        "2023-11-01",
        "Nov 1–2, 2023",
        Track::Law,
        "Bletchley Park AI Safety Summit",
        "The UK hosts the first global AI Safety Summit; 28 countries plus the EU agree the Bletchley Declaration — a shared account of frontier-AI opportunity and risk, and of international safety research.",
        "https://www.gov.uk/government/news/countries-agree-to-safe-and-responsible-development-of-frontier-ai-in-landmark-bletchley-declaration",
    ),
    entry(
        "2024",
        "2024-05-17",
        "May 17, 2024",
        Track::Law,
        "Colorado AI Act — first US state AI law",
        "Gov. Jared Polis signs SB24-205, \u{201c}Consumer Protections in Interactions with Artificial Intelligence Systems\u{201d}: the first comprehensive US state AI law, imposing reasonable-care, documentation, and disclosure duties around high-risk AI in consequential decisions. (Operative date later delayed to Jun 30, 2026.)",
        "https://leg.colorado.gov/sites/default/files/2024a_205_signed.pdf",
    ),
    entry(
        "2024",
        "2024-08-01",
        "Aug 1, 2024",
        Track::Law,
        "EU AI Act enters into force",
        "Regulation (EU) 2024/1689 — published in the Official Journal Jul 12 — enters into force, laying down risk-based rules for AI systems in the Union. Most obligations apply from Aug 2, 2026; the phase-in is the story (see the bar below).",
        "https://eur-lex.europa.eu/eli/reg/2024/1689/oj/eng",
    ),
    entry(
        "2025",
        "2025-01-20",
        "Jan 20, 2025",
        Track::Law,
        "EO 14110 revoked",
        "On inauguration day, EO 14148 (\u{201c}Initial Rescissions of Harmful Executive Orders and Actions\u{201d}) revokes EO 14110 among 78 Biden-era actions. The revocation is often misattributed to EO 14179; the instruments are distinct.",
        "https://www.federalregister.gov/documents/2025/01/28/2025-01901/initial-rescissions-of-harmful-executive-orders-and-actions",
    ),
    entry(
        "2025",
        "2025-01-23",
        "Jan 23, 2025",
        Track::Law,
        "EO 14179 — \u{201c}Removing Barriers to American Leadership in AI\u{201d}",
        "Three days later, EO 14179 sets US policy to sustain and enhance global AI dominance and directs an AI Action Plan within 180 days.",
        "https://www.federalregister.gov/documents/2025/01/31/2025-02172/removing-barriers-to-american-leadership-in-artificial-intelligence",
    ),
    entry(
        "2025",
        "2025-02-10",
        "Feb 10–11, 2025",
        Track::Law,
        "Paris AI Action Summit",
        "France and India co-chair the third global summit, at the Grand Palais: public-interest AI, the future of work, trust, and governance. Its Statement on Inclusive and Sustainable AI is not signed by the US or the UK.",
        "https://www.elysee.fr/en/sommet-pour-l-action-sur-l-ia",
    ),
    entry(
        "2025",
        "2025-06-22",
        "Jun 22, 2025",
        Track::Law,
        "Texas TRAIGA signed",
        "Gov. Greg Abbott signs HB 149, the Texas Responsible Artificial Intelligence Governance Act: disclosure duties for government and healthcare AI, prohibitions on certain harmful uses, civil penalties, a regulatory sandbox, and a state AI council. Effective Jan 1, 2026.",
        "https://capitol.texas.gov/BillLookup/History.aspx?LegSess=89R&Bill=HB149",
    ),
    entry(
        "2025",
        "2025-07-23",
        "Jul 23, 2025",
        Track::Law,
        "America's AI Action Plan",
        "The White House releases \u{201c}Winning the Race: America's AI Action Plan\u{201d} — 90+ federal actions across innovation, infrastructure, and diplomacy — with three same-day orders: EO 14318 (data-center permitting), EO 14319 (\u{201c}Preventing Woke AI in the Federal Government\u{201d}), EO 14320 (exporting the American AI stack).",
        "https://www.whitehouse.gov/articles/2025/07/white-house-unveils-americas-ai-action-plan/",
    ),
    entry(
        "2025",
        "2025-09-29",
        "Sep 29, 2025",
        Track::Law,
        "California SB 53 — the frontier-AI law",
        "Gov. Gavin Newsom signs SB 53, the Transparency in Frontier Artificial Intelligence Act (Chapter 138): frontier developers must publish safety frameworks and transparency reports, report critical safety incidents to the Office of Emergency Services, and protect whistleblowers. Effective Jan 1, 2026.",
        "https://leginfo.legislature.ca.gov/faces/billTextClient.xhtml?bill_id=202520260SB53",
    ),
];

fn all_entries() -> Vec<&'static Entry> {
    let mut all: Vec<&'static Entry> = Vec::new();
    all.extend(PROTOCOLS.iter());
    all.extend(MODELS.iter());
    all.extend(LAW.iter());
    all.sort_by(|a, b| a.date.trim_start_matches('~').cmp(b.date.trim_start_matches('~')));
    all
}

#[component]
fn TimelineEntry(e: &'static Entry) -> impl IntoView {
    view! {
        <article class="tl-entry" data-track={e.track.key()}>
            <p class="tl-meta">
                <span class={format!("tl-chip tl-chip-{}", e.track.class())}>{e.track.chip()}</span>
                {(e.gov).then(|| view! { <span class="tl-chip tl-chip-gov">"GOVERNANCE"</span> })}
                <time class="tl-date" datetime={e.date}>{e.display}</time>
            </p>
            <h4 class="tl-title">{e.title}</h4>
            <p class="tl-body">{e.body}</p>
            {(!e.flag.is_empty()).then(|| view! { <p class="tl-flag">{e.flag}</p> })}
            <p class="tl-source">
                <a href={e.source} rel="external">"Source"</a>
            </p>
        </article>
    }
}

#[component]
pub fn OnAi() -> impl IntoView {
    let entries = all_entries();
    let years: Vec<&'static str> = {
        let mut ys: Vec<&'static str> = Vec::new();
        for e in &entries {
            if !ys.contains(&e.year) {
                ys.push(e.year);
            }
        }
        ys
    };
    view! {
        <Title text="On AI - Qompass AI" />
        <section class="page on-ai-page">
            <p class="page-bearing">"ON AI"</p>
            <h1>"On AI"</h1>
            <p class="page-lede">
                "A compressed timeline from ChatGPT to Agent2Agent — the models that "
                "changed what software can do, the protocols that let agents work "
                "together, and the laws racing to catch up. Three tracks, one page, "
                "no hype: what happened, when, and where it is going."
            </p>

            <div class="era-strip" role="img" aria-label="Four eras: Chatbots 2022 to 2023, Copilots and multimodal 2023 to 2024, Reasoners 2024 to 2025, Agents and protocols 2025 onward">
                <div class="era">
                    <span class="era-name">"Chatbots"</span>
                    <span class="era-range">"2022–23"</span>
                </div>
                <div class="era">
                    <span class="era-name">"Copilots & multimodal"</span>
                    <span class="era-range">"2023–24"</span>
                </div>
                <div class="era">
                    <span class="era-name">"Reasoners"</span>
                    <span class="era-range">"2024–25"</span>
                </div>
                <div class="era era-now">
                    <span class="era-name">"Agents & protocols"</span>
                    <span class="era-range">"2025– "</span>
                </div>
            </div>

            <h2 class="band-label">"THE AGENT STACK"</h2>
            <div class="stack">
                <div class="stack-layer">
                    <span class="stack-name">"Models"</span>
                    <span class="stack-gloss">"The reasoning engines — frontier models that plan, write, and decide."</span>
                </div>
                <div class="stack-layer">
                    <span class="stack-name">"Tools — MCP"</span>
                    <span class="stack-gloss">"The Model Context Protocol: one standard way for a model to reach tools and data."</span>
                </div>
                <div class="stack-layer">
                    <span class="stack-name">"Agent-to-agent — A2A"</span>
                    <span class="stack-gloss">"Agents from different vendors discover each other and delegate work."</span>
                </div>
                <div class="stack-layer">
                    <span class="stack-name">"Payments — AP2"</span>
                    <span class="stack-gloss">"Cryptographically signed mandates let agents transact, not just talk."</span>
                </div>
                <div class="stack-layer">
                    <span class="stack-name">"Governance"</span>
                    <span class="stack-gloss">"Foundations steward the specs — the Linux Foundation's Agentic AI Foundation."</span>
                </div>
            </div>

            <h2 class="band-label">"THE TIMELINE"</h2>
            <div class="tl-filters" role="group" aria-label="Filter timeline by track">
                <button type="button" class="tl-filter" data-filter="all" aria-pressed="true">"All"</button>
                <button type="button" class="tl-filter" data-filter="protocols" aria-pressed="false">"Protocols"</button>
                <button type="button" class="tl-filter" data-filter="models" aria-pressed="false">"Models & Companies"</button>
                <button type="button" class="tl-filter" data-filter="law" aria-pressed="false">"Law & Policy"</button>
            </div>
            <div class="tl">
                {years
                    .into_iter()
                    .map(|year| {
                        view! {
                            <h3 class="tl-year" data-year={year}>{year}</h3>
                            {entries
                                .iter()
                                .filter(|e| e.year == year)
                                .copied()
                                .map(|e| view! { <TimelineEntry e=e /> })
                                .collect_view()}
                        }
                    })
                    .collect_view()}
            </div>

            <h2 class="band-label">"REGULATION, PHASED IN"</h2>
            <p class="on-ai-note">
                "The EU AI Act is a phase-in, not a switch: obligations land in "
                "stages from 2025 to 2027. The US picture is executive action at "
                "the federal level and statutes in the states."
            </p>
            <div class="phase-bar" role="img" aria-label="Regulation phases: EU AI Act in force August 2024, GPAI obligations 2025, high-risk obligations 2026 to 2027; US executive action 2023 to 2025; US state laws from 2024, taking effect 2026">
                <div class="phase-row">
                    <span class="phase-label">"EU AI Act"</span>
                    <div class="phase-track">
                        <span class="phase-seg">"2024 · in force"</span>
                        <span class="phase-seg">"2025 · GPAI obligations"</span>
                        <span class="phase-seg phase-seg-long">"2026–27 · high-risk obligations"</span>
                    </div>
                </div>
                <div class="phase-row">
                    <span class="phase-label">"US — executive"</span>
                    <div class="phase-track">
                        <span class="phase-seg">"2023 · EO 14110"</span>
                        <span class="phase-seg">"2025 · revoked; EO 14179 + AI Action Plan"</span>
                    </div>
                </div>
                <div class="phase-row">
                    <span class="phase-label">"US — states"</span>
                    <div class="phase-track">
                        <span class="phase-seg">"2024 · Colorado AI Act"</span>
                        <span class="phase-seg">"2025 · Texas TRAIGA · California SB 53"</span>
                        <span class="phase-seg">"2026 · state laws take effect"</span>
                    </div>
                </div>
            </div>

            <h2 class="band-label">"WHERE IT'S GOING"</h2>
            <p class="on-ai-note">
                "Trajectory, not prediction — each item is tied to a dated fact above."
            </p>
            <ul class="outlook">
                <li>
                    <strong>"Protocols consolidate under foundations."</strong>
                    " MCP (Dec 2025) and A2A (Aug 2026) both sit at the Agentic AI "
                    "Foundation; the specs stay separate, the governance converges."
                </li>
                <li>
                    <strong>"Agents get wallets."</strong>
                    " AP2 (Sep 2025) layers signed payment mandates on A2A + MCP — "
                    "commerce follows messaging, as it did on the web."
                </li>
                <li>
                    <strong>"The EU phase-in reaches high-risk systems."</strong>
                    " GPAI obligations landed in 2025; high-risk obligations phase "
                    "in across 2026–27."
                </li>
                <li>
                    <strong>"US state laws switch on."</strong>
                    " Colorado, Texas, and California statutes take effect through "
                    "2026 — compliance becomes a state-by-state map."
                </li>
                <li>
                    <strong>"Agents move from demos to infrastructure."</strong>
                    " A public registry (Sep 2025), a stable A2A 1.0 (Mar 2026), and "
                    "a stateless MCP core (Jul 2026) are what infrastructure looks "
                    "like while it is still being built."
                </li>
            </ul>

            <h2 class="band-label">"SOURCES"</h2>
            <p class="on-ai-note">
                "Expanded from the "
                <a href="https://github.com/qompassai/daelin">"Daelin timeline"</a>
                " — the Protocols track is ported from it, honesty markers and "
                "all. Dates on the other tracks were verified against primary "
                "sources (the Federal Register, EUR-Lex, state legislature "
                "records, and company announcements); each entry links its source. "
                "Entries marked [secondary] rest on corroborating coverage rather "
                "than a primary page; [disputed] marks a date sources disagreed on."
            </p>
            <script src="/js/on-ai.js"></script>
        </section>
    }
}
