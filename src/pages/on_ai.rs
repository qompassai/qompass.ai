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

/// The agent stack as a true diagram: five layers on a foundation slab,
/// a request path flowing down the left, and agent nodes talking
/// sideways on the A2A layer.
#[component]
fn StackDiagram() -> impl IntoView {
    view! {
        <figure class="fig fig-slim">
            <svg viewBox="0 0 412 408" role="img" aria-label="Diagram of the agent stack: Models, Tools over MCP, Agent-to-agent over A2A, Payments over AP2, all on a Governance foundation; a request path flows from the user down through the model to its tools">
                <title>"The agent stack"</title>
                <desc>"Five horizontal layers — Models, Tools (MCP), Agent-to-agent (A2A), Payments (AP2) — on a Governance foundation slab. A vertical arrow at the left carries a request from the user through the model to its tools; two agent nodes exchange a double-headed arrow on the A2A layer."</desc>
                <defs>
                    <marker id="sd-arr" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto-start-reverse">
                        <path d="M0 0 L7 3 L0 6 z" class="mk-arr"></path>
                    </marker>
                </defs>
                <text x="2" y="20" class="f-mono f-brass" font-size="13">"USER"</text>
                <circle cx="24" cy="36" r="5" class="dot-cher"></circle>
                <path d="M24 44 V148" class="ln-arrow ln-flow" marker-end="url(#sd-arr)"></path>
                <circle cx="24" cy="62" r="2.5" class="dot-cher"></circle>
                <rect x="56" y="30" width="340" height="64" rx="5" class="box"></rect>
                <rect x="56" y="30" width="4" height="64" class="acc"></rect>
                <text x="72" y="58" class="f-name" font-size="16">"Models"</text>
                <text x="72" y="81" class="f-dim" font-size="13">"Frontier models that plan, write, and decide."</text>
                <rect x="56" y="102" width="340" height="64" rx="5" class="box"></rect>
                <rect x="56" y="102" width="4" height="64" class="acc"></rect>
                <text x="72" y="130" class="f-name" font-size="16">"Tools — MCP"</text>
                <text x="72" y="153" class="f-dim" font-size="13">"One standard way to reach tools and data."</text>
                <rect x="56" y="174" width="340" height="88" rx="5" class="box"></rect>
                <rect x="56" y="174" width="4" height="88" class="acc-soft"></rect>
                <text x="72" y="202" class="f-name" font-size="16">"Agent-to-agent — A2A"</text>
                <text x="72" y="225" class="f-dim" font-size="13">"Agents discover each other — and delegate work."</text>
                <rect x="72" y="234" width="86" height="24" rx="4" class="box-raise"></rect>
                <text x="115" y="250" class="f-mid" font-size="13">"Agent A"</text>
                <line x1="162" y1="246" x2="196" y2="246" class="ln-arrow ln-flow" marker-start="url(#sd-arr)" marker-end="url(#sd-arr)"></line>
                <rect x="200" y="234" width="86" height="24" rx="4" class="box-raise"></rect>
                <text x="243" y="250" class="f-mid" font-size="13">"Agent B"</text>
                <rect x="56" y="270" width="340" height="64" rx="5" class="box"></rect>
                <rect x="56" y="270" width="4" height="64" class="acc-violet"></rect>
                <text x="72" y="298" class="f-name" font-size="16">"Payments — AP2"</text>
                <text x="72" y="321" class="f-dim" font-size="13">"Signed mandates: agents transact, not just talk."</text>
                <rect x="40" y="342" width="364" height="58" rx="5" class="box-raise slab"></rect>
                <text x="68" y="369" class="f-name" font-size="16">"Governance"</text>
                <text x="68" y="390" class="f-dim" font-size="13">"Stewarded by the Linux Foundation's AAIF."</text>
            </svg>
            <figcaption>
                "One request path, five layers: a user's request flows down through the "
                "model to its tools (left), agents coordinate sideways over A2A, payments "
                "settle over AP2 — and the whole stack sits on foundation governance, the "
                "Linux Foundation's Agentic AI Foundation."
            </figcaption>
        </figure>
    }
}

/// MCP vs A2A: complements, not rivals — horizontal agent-to-agent
/// traffic over A2A, vertical agent-to-tools spokes over MCP.
#[component]
fn McpA2aDiagram() -> impl IntoView {
    view! {
        <figure class="fig fig-slim">
            <svg viewBox="0 0 412 252" role="img" aria-label="Diagram: two agents linked horizontally by A2A, each with a vertical MCP spoke down to a shared tools and data box">
                <title>"MCP and A2A are complements"</title>
                <desc>"Two agent boxes face each other across a double-headed horizontal arrow labelled A2A — agents to agents. From each agent a vertical arrow labelled MCP runs down to one wide box labelled Tools and data."</desc>
                <defs>
                    <marker id="md-arr" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto-start-reverse">
                        <path d="M0 0 L7 3 L0 6 z" class="mk-arr"></path>
                    </marker>
                </defs>
                <text x="206" y="22" class="f-mid f-cher f-name" font-size="14">"A2A — agents to agents"</text>
                <rect x="30" y="34" width="140" height="54" rx="5" class="box agent"></rect>
                <text x="100" y="67" class="f-mid f-name" font-size="15.5">"Agent A"</text>
                <rect x="242" y="34" width="140" height="54" rx="5" class="box agent"></rect>
                <text x="312" y="67" class="f-mid f-name" font-size="15.5">"Agent B"</text>
                <line x1="176" y1="61" x2="236" y2="61" class="ln-arrow ln-flow" marker-start="url(#md-arr)" marker-end="url(#md-arr)"></line>
                <path d="M100 88 V166" class="ln-arrow ln-flow" marker-end="url(#md-arr)"></path>
                <path d="M312 88 V166" class="ln-arrow ln-flow" marker-end="url(#md-arr)"></path>
                <text x="110" y="132" class="f-mono f-brass" font-size="13">"MCP"</text>
                <text x="322" y="132" class="f-mono f-brass" font-size="13">"MCP"</text>
                <rect x="30" y="168" width="352" height="56" rx="5" class="box"></rect>
                <text x="206" y="193" class="f-mid f-name" font-size="15">"Tools & data"</text>
                <text x="206" y="213" class="f-mid f-dim" font-size="13">"Files, APIs, databases, services"</text>
            </svg>
            <figcaption>
                "A2A for agent-to-agent across boundaries, MCP for agent-to-tools-and-data — "
                "the complementarity the Linux Foundation restates. Neither replaces the "
                "other; a working agent uses both at once."
            </figcaption>
        </figure>
    }
}

/// Four eras on one arrow, 2022 to 2026+. Wide horizontal variant for
/// desktop, vertical variant for phones — same facts, both labelled.
/// The interactive Transformer explorer: a server-rendered shell of
/// original copy; `/js/on-ai.js` lazily fetches the recorded GPT-2
/// activations (`/data/transformer-explorer.json`, generated by
/// `scripts/generate-transformer-explorer.py`) on first interaction
/// and builds the widgets. Without JS the copy and the honesty note
/// still stand.
#[component]
fn TransformerExplorer() -> impl IntoView {
    view! {
        <div class="tx" id="transformer-explorer">
            <p class="on-ai-note">
                "Everything else on this page is about what AI systems do. "
                "This section opens one up. A transformer reads text as "
                "tokens, turns each token into a vector, and passes those "
                "vectors through twelve layers of attention and arithmetic "
                "before it bets on the next token. The explorer below walks "
                "that pipeline on seven short sentences, stage by stage."
            </p>
            <button
                type="button"
                class="tx-open"
                id="tx-open"
                aria-label="Open the interactive Transformer explorer"
            >
                "Open the explorer"
            </button>
            <p class="tx-status" id="tx-status" role="status"></p>
            <div class="tx-body" id="tx-body" hidden>
                <h3 class="tx-stage">"1 · Tokens and embeddings"</h3>
                <p class="tx-copy">
                    "Tokenization cuts text into pieces from a fixed "
                    "vocabulary of 50,257 — common words whole, rare words "
                    "in fragments. Each piece is looked up in a table of "
                    "768-number vectors, its embedding; a second table adds "
                    "a vector for the piece's position. What travels through "
                    "the model is the sum: meaning plus place. Pick a "
                    "sentence, then a token. The scatter is a PCA projection "
                    "of those 768 numbers down to two dimensions — "
                    "distances are indicative, not exact — and the strip "
                    "shows the first 48 numbers of each token's vector, "
                    "exactly as recorded."
                </p>
                <div
                    class="tx-picker"
                    id="tx-picker"
                    role="group"
                    aria-label="Choose a sentence"
                ></div>
                <p class="tx-note-line" id="tx-sentence-note"></p>
                <div
                    class="tx-tokens"
                    id="tx-tokens"
                    role="group"
                    aria-label="Tokens in the chosen sentence"
                ></div>
                <div class="tx-emb">
                    <figure class="tx-fig">
                        <canvas id="tx-pca" height="260"></canvas>
                        <figcaption>
                            "Embedding scatter — PCA projection to 2-D"
                        </figcaption>
                    </figure>
                    <figure class="tx-fig">
                        <canvas id="tx-slice" height="260"></canvas>
                        <figcaption>
                            "First 48 of 768 embedding values per token"
                        </figcaption>
                    </figure>
                </div>

                <h3 class="tx-stage">"2 · Attention"</h3>
                <p class="tx-copy">
                    "Attention is how tokens talk to each other. Every token "
                    "emits a query, a key, and a value; its next "
                    "representation is a weighted blend of the values of "
                    "the tokens it attends to, and the weights are scaled "
                    "dot products of queries against keys, normalized to "
                    "sum to one. GPT-2 runs twelve heads of this in "
                    "parallel, twelve layers deep. Rows are the listening "
                    "token, columns the listened-to token. The empty upper "
                    "triangle is the causal mask: the model is never "
                    "allowed to peek at the future it is trying to predict. "
                    "Touch a cell to read the exact weight."
                </p>
                <div class="tx-controls">
                    <div class="tx-ctl">
                        <button
                            type="button"
                            class="tx-btn"
                            id="tx-layer-prev"
                            aria-label="Previous layer"
                        >
                            "−"
                        </button>
                        <span class="tx-ctl-label" id="tx-layer-label">
                            "Layer 1 of 12"
                        </span>
                        <button
                            type="button"
                            class="tx-btn"
                            id="tx-layer-next"
                            aria-label="Next layer"
                        >
                            "+"
                        </button>
                        <input
                            type="range"
                            id="tx-layer-range"
                            min="0"
                            max="12"
                            step="1"
                            value="1"
                            aria-label="Layer, 0 is the embedding"
                        />
                    </div>
                    <div class="tx-ctl">
                        <label class="tx-ctl-label" for="tx-head">
                            "Head"
                        </label>
                        <select id="tx-head" class="tx-select"></select>
                    </div>
                </div>
                <p class="tx-note-line" id="tx-attn-note" hidden>
                    "Layer 0 is the embedding itself — attention starts "
                    "at layer 1."
                </p>
                <canvas id="tx-attn" height="320"></canvas>
                <p
                    class="tx-readout"
                    id="tx-attn-readout"
                    role="status"
                ></p>

                <h3 class="tx-stage">
                    "3 · What each layer thinks comes next"
                </h3>
                <p class="tx-copy">
                    "Apply the model's output head to every layer's "
                    "representation of the final token — a probe called "
                    "the logit lens — and a draft answer forms early, "
                    "then sharpens. The probe is not a component; the "
                    "model itself only ever reads the last layer. Below "
                    "the strip, the heat row shows each token's "
                    "residual-stream norm per layer: watch the stream "
                    "grow as layer after layer writes into it."
                </p>
                <div class="tx-lens" id="tx-lens"></div>
                <canvas id="tx-norms" height="150"></canvas>

                <h3 class="tx-stage">"4 · The output, and a dial"</h3>
                <p class="tx-copy">
                    "At the end, the final representation is scored "
                    "against all 50,257 token embeddings and a softmax "
                    "turns scores into probabilities. Temperature rescales "
                    "the scores first: below 1 the favourite pulls away, "
                    "above 1 the field flattens. These bars are recomputed "
                    "in your browser from the model's recorded top-50 "
                    "scores; the other 50,207 tokens share one combined "
                    "bucket, so the probabilities stay honest."
                </p>
                <div class="tx-controls">
                    <div class="tx-ctl">
                        <label class="tx-ctl-label" for="tx-temp">
                            "Temperature"
                        </label>
                        <input
                            type="range"
                            id="tx-temp"
                            min="0.3"
                            max="2"
                            step="0.05"
                            value="1"
                        />
                        <span class="tx-ctl-label" id="tx-temp-label">
                            "1.00"
                        </span>
                    </div>
                    <p class="tx-readout" id="tx-topp"></p>
                </div>
                <div class="tx-bars" id="tx-bars"></div>

                <p class="tx-honesty">
                    "Honesty note: every number in this section is a real "
                    "output of GPT-2 (small), recorded by Qompass for "
                    "these seven sentences — attention weights, "
                    "embeddings, and logits alike. Nothing is simulated, "
                    "and the model does not run in your browser: the data "
                    "file is static, the arithmetic happens on your "
                    "device, and nothing is sent anywhere."
                </p>
                <p class="tx-prov" id="tx-prov"></p>
                <p class="tx-credit">
                    "The idea of an explorable transformer — the pipeline "
                    "laid out stage by stage, attention as a matrix you "
                    "can interrogate — owes a debt to Georgia Tech's "
                    <a href="https://poloclub.github.io/transformer-explainer/">
                        "Transformer Explainer"
                    </a>
                    ", which runs a live model in the browser. This is "
                    "an independent implementation with recorded "
                    "activations, built for this page."
                </p>
            </div>
        </div>
    }
}

#[component]
fn EraDiagram() -> impl IntoView {
    view! {
        <figure class="fig">
            <svg class="fig-wide" viewBox="0 0 860 216" role="img" aria-label="Timeline arrow 2022 to 2026 plus with four eras: Chatbots, Copilots and multimodal, Reasoners, Agents and protocols, each with two anchor events">
                <title>"Four eras, 2022 to 2026+"</title>
                <desc>"A horizontal arrow from 2022 to 2026 and beyond carries four era nodes: Chatbots (ChatGPT, GPT-4), Copilots and multimodal (Claude 3, GPT-4o), Reasoners (o1-preview, DeepSeek R1), Agents and protocols (A2A, AP2)."</desc>
                <defs>
                    <marker id="ed-arr" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto-start-reverse">
                        <path d="M0 0 L7 3 L0 6 z" class="mk-fog"></path>
                    </marker>
                </defs>
                <path d="M16 108 H836" class="ln-axis" marker-end="url(#ed-arr)"></path>
                <line x1="36" y1="100" x2="36" y2="116" class="grid"></line>
                <line x1="232" y1="100" x2="232" y2="116" class="grid"></line>
                <line x1="428" y1="100" x2="428" y2="116" class="grid"></line>
                <line x1="624" y1="100" x2="624" y2="116" class="grid"></line>
                <line x1="812" y1="100" x2="812" y2="116" class="grid"></line>
                <text x="36" y="200" class="f-mid f-mono f-dim" font-size="12.5">"2022"</text>
                <text x="232" y="200" class="f-mid f-mono f-dim" font-size="12.5">"2023"</text>
                <text x="428" y="200" class="f-mid f-mono f-dim" font-size="12.5">"2024"</text>
                <text x="624" y="200" class="f-mid f-mono f-dim" font-size="12.5">"2025"</text>
                <text x="812" y="200" class="f-mid f-mono f-dim" font-size="12.5">"2026+"</text>
                <text x="134" y="50" class="f-mid f-name" font-size="16.5">"Chatbots"</text>
                <text x="134" y="72" class="f-mid f-mono f-dim" font-size="12">"2022–23"</text>
                <circle cx="134" cy="108" r="8" class="node"></circle>
                <text x="134" y="150" class="f-mid f-dim" font-size="13">"ChatGPT · Nov '22"</text>
                <text x="134" y="172" class="f-mid f-dim" font-size="13">"GPT-4 · Mar '23"</text>
                <text x="330" y="50" class="f-mid f-name" font-size="16.5">"Copilots & multimodal"</text>
                <text x="330" y="72" class="f-mid f-mono f-dim" font-size="12">"2023–24"</text>
                <circle cx="330" cy="108" r="8" class="node"></circle>
                <text x="330" y="150" class="f-mid f-dim" font-size="13">"Claude 3 · Mar '24"</text>
                <text x="330" y="172" class="f-mid f-dim" font-size="13">"GPT-4o · May '24"</text>
                <text x="526" y="50" class="f-mid f-name" font-size="16.5">"Reasoners"</text>
                <text x="526" y="72" class="f-mid f-mono f-dim" font-size="12">"2024–25"</text>
                <circle cx="526" cy="108" r="8" class="node"></circle>
                <text x="526" y="150" class="f-mid f-dim" font-size="13">"o1-preview · Sep '24"</text>
                <text x="526" y="172" class="f-mid f-dim" font-size="13">"DeepSeek R1 · Jan '25"</text>
                <text x="722" y="50" class="f-mid f-name" font-size="16.5">"Agents & protocols"</text>
                <text x="722" y="72" class="f-mid f-mono f-dim" font-size="12">"2025– "</text>
                <circle cx="722" cy="108" r="8" class="node-now fig-pulse"></circle>
                <text x="722" y="150" class="f-mid f-dim" font-size="13">"A2A · Apr '25"</text>
                <text x="722" y="172" class="f-mid f-dim" font-size="13">"AP2 · Sep '25"</text>
            </svg>
            <svg class="fig-narrow" viewBox="0 0 360 452" role="img" aria-label="Vertical timeline 2022 to 2026 plus with four eras: Chatbots, Copilots and multimodal, Reasoners, Agents and protocols, each with two anchor events">
                <title>"Four eras, 2022 to 2026+ (vertical)"</title>
                <desc>"The same four eras on a vertical arrow for narrow screens: Chatbots, Copilots and multimodal, Reasoners, Agents and protocols, each annotated with two anchor events from the timeline."</desc>
                <defs>
                    <marker id="edn-arr" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto-start-reverse">
                        <path d="M0 0 L7 3 L0 6 z" class="mk-fog"></path>
                    </marker>
                </defs>
                <path d="M40 14 V430" class="ln-axis" marker-end="url(#edn-arr)"></path>
                <text x="2" y="26" class="f-mono f-dim" font-size="12">"2022"</text>
                <text x="2" y="442" class="f-mono f-dim" font-size="12">"2026+"</text>
                <circle cx="40" cy="64" r="7" class="node"></circle>
                <text x="62" y="54" class="f-name" font-size="15.5">"Chatbots"</text>
                <text x="352" y="54" class="f-end f-mono f-dim" font-size="12">"2022–23"</text>
                <text x="62" y="80" class="f-dim" font-size="12.5">"ChatGPT · Nov '22"</text>
                <text x="62" y="102" class="f-dim" font-size="12.5">"GPT-4 · Mar '23"</text>
                <circle cx="40" cy="172" r="7" class="node"></circle>
                <text x="62" y="162" class="f-name" font-size="15.5">"Copilots & multimodal"</text>
                <text x="352" y="162" class="f-end f-mono f-dim" font-size="12">"2023–24"</text>
                <text x="62" y="188" class="f-dim" font-size="12.5">"Claude 3 · Mar '24"</text>
                <text x="62" y="210" class="f-dim" font-size="12.5">"GPT-4o · May '24"</text>
                <circle cx="40" cy="280" r="7" class="node"></circle>
                <text x="62" y="270" class="f-name" font-size="15.5">"Reasoners"</text>
                <text x="352" y="270" class="f-end f-mono f-dim" font-size="12">"2024–25"</text>
                <text x="62" y="296" class="f-dim" font-size="12.5">"o1-preview · Sep '24"</text>
                <text x="62" y="318" class="f-dim" font-size="12.5">"DeepSeek R1 · Jan '25"</text>
                <circle cx="40" cy="388" r="7" class="node-now fig-pulse"></circle>
                <text x="62" y="378" class="f-name" font-size="15.5">"Agents & protocols"</text>
                <text x="352" y="378" class="f-end f-mono f-dim" font-size="12">"2025– "</text>
                <text x="62" y="404" class="f-dim" font-size="12.5">"A2A · Apr '25"</text>
                <text x="62" y="426" class="f-dim" font-size="12.5">"AP2 · Sep '25"</text>
            </svg>
            <figcaption>
                "Four eras in five years — the anchor releases for each era are entries "
                "on the timeline below."
            </figcaption>
        </figure>
    }
}

/// The law on one axis: a Gantt over 2023–2027 with EU, US federal, and
/// US states swimlanes. Every date matches a verified timeline entry.
#[component]
fn LawDiagram() -> impl IntoView {
    view! {
        <figure class="fig">
            <svg class="fig-wide" viewBox="0 0 860 322" role="img" aria-label="Gantt chart of AI law 2023 to 2027: EU AI Act phases, US federal executive orders with the 2025 revocation, and US state laws signed 2024 to 2025 and effective 2026">
                <title>"The law on one axis, 2023–2027"</title>
                <desc>"Three swimlanes on a 2023 to 2027 axis. EU AI Act: in force August 2024, GPAI obligations August 2025, most obligations August 2 2026, high-risk phase 2027. US federal: Executive Order 14110 of October 2023, revoked January 2025, EO 14179 the same month, the AI Action Plan and accompanying orders July 2025. US states: Colorado signed May 2024, Texas June 2025, California SB 53 September 2025, the state laws taking effect through 2026."</desc>
                <line x1="128" y1="30" x2="128" y2="302" class="grid"></line>
                <line x1="297" y1="30" x2="297" y2="302" class="grid"></line>
                <line x1="466" y1="30" x2="466" y2="302" class="grid"></line>
                <line x1="634" y1="30" x2="634" y2="302" class="grid"></line>
                <line x1="802" y1="30" x2="802" y2="302" class="grid"></line>
                <text x="128" y="20" class="f-mid f-mono f-dim" font-size="12.5">"2023"</text>
                <text x="297" y="20" class="f-mid f-mono f-dim" font-size="12.5">"2024"</text>
                <text x="466" y="20" class="f-mid f-mono f-dim" font-size="12.5">"2025"</text>
                <text x="634" y="20" class="f-mid f-mono f-dim" font-size="12.5">"2026"</text>
                <text x="802" y="20" class="f-mid f-mono f-dim" font-size="12.5">"2027"</text>
                <line x1="767" y1="28" x2="767" y2="302" class="ln-today"></line>
                <text x="767" y="314" class="f-mid f-brass" font-size="11.5">"Oct '26 — today"</text>
                <text x="2" y="80" class="f-mono f-brass" font-size="12.5">"EU AI ACT"</text>
                <rect x="396" y="60" width="167" height="28" class="seg seg-a"></rect>
                <rect x="563" y="60" width="170" height="28" class="seg seg-b"></rect>
                <rect x="733" y="60" width="69" height="28" class="seg seg-c"></rect>
                <rect x="802" y="60" width="42" height="28" class="seg seg-d"></rect>
                <text x="856" y="50" class="f-end" font-size="12">"High-risk phase · 2027"</text>
                <text x="396" y="104" class="f-dim" font-size="12">"In force · Aug '24"</text>
                <text x="563" y="104" class="f-dim" font-size="12">"GPAI · Aug '25"</text>
                <text x="856" y="104" class="f-end f-dim" font-size="12">"Most obligations · Aug 2, '26"</text>
                <text x="2" y="152" class="f-mono f-brass" font-size="12.5">"US — FEDERAL"</text>
                <text x="261" y="116" font-size="12">"EO 14110 · Oct '23"</text>
                <rect x="261" y="124" width="212" height="9" class="seg seg-e"></rect>
                <path d="M467 120 L479 136 M479 120 L467 136" class="ln-x"></path>
                <text x="487" y="133" class="f-violet" font-size="12">"revoked Jan '25"</text>
                <circle cx="473" cy="156" r="4.5" class="dot-cher"></circle>
                <text x="483" y="160" font-size="12">"EO 14179 · Jan '25"</text>
                <circle cx="556" cy="182" r="4.5" class="dot-cher"></circle>
                <text x="566" y="186" font-size="12">"AI Action Plan + EOs · Jul '25"</text>
                <text x="2" y="264" class="f-mono f-brass" font-size="12.5">"US — STATES"</text>
                <text x="359" y="222" font-size="12">"Colorado AI Act · signed May '24"</text>
                <rect x="359" y="228" width="358" height="8" class="seg seg-f"></rect>
                <text x="723" y="235" class="f-dim" font-size="11.5">"effective '26"</text>
                <text x="543" y="258" font-size="12">"Texas TRAIGA · signed Jun '25"</text>
                <rect x="543" y="264" width="91" height="8" class="seg seg-g"></rect>
                <text x="640" y="271" class="f-dim" font-size="11.5">"eff. Jan '26"</text>
                <text x="585" y="288" font-size="12">"California SB 53 · Sep '25"</text>
                <rect x="585" y="294" width="49" height="8" class="seg seg-g"></rect>
            </svg>
            <svg class="fig-narrow" viewBox="0 0 360 420" role="img" aria-label="The law on one axis, stacked for narrow screens: EU AI Act phases, US federal executive orders with the 2025 revocation, and US state laws effective 2026">
                <title>"The law on one axis (stacked)"</title>
                <desc>"The same Gantt as three stacked bands with their own 2023 to 2027 axes: the EU AI Act phase-in, US federal executive action including the January 2025 revocation of EO 14110, and US state statutes signed 2024 to 2025, effective 2026."</desc>
                <text x="8" y="16" class="f-mono f-brass" font-size="12.5">"EU AI ACT"</text>
                <line x1="8" y1="30" x2="352" y2="30" class="grid"></line>
                <text x="8" y="44" class="f-mono f-dim" font-size="11.5">"2023"</text>
                <text x="89" y="44" class="f-mid f-mono f-dim" font-size="11.5">"2024"</text>
                <text x="170" y="44" class="f-mid f-mono f-dim" font-size="11.5">"2025"</text>
                <text x="251" y="44" class="f-mid f-mono f-dim" font-size="11.5">"2026"</text>
                <text x="352" y="44" class="f-end f-mono f-dim" font-size="11.5">"2027"</text>
                <rect x="137" y="54" width="83" height="20" class="seg seg-a"></rect>
                <rect x="220" y="54" width="81" height="20" class="seg seg-b"></rect>
                <rect x="301" y="54" width="31" height="20" class="seg seg-c"></rect>
                <rect x="332" y="54" width="20" height="20" class="seg seg-d"></rect>
                <text x="178" y="68" class="f-mid" font-size="11.5">"In force"</text>
                <text x="260" y="68" class="f-mid" font-size="11.5">"GPAI"</text>
                <text x="356" y="92" class="f-end f-dim" font-size="11.5">"Most obligations · Aug 2, '26"</text>
                <text x="356" y="108" class="f-end f-dim" font-size="11.5">"High-risk phase · 2027"</text>
                <text x="8" y="140" class="f-mono f-brass" font-size="12.5">"US — FEDERAL"</text>
                <line x1="8" y1="154" x2="352" y2="154" class="grid"></line>
                <text x="8" y="168" class="f-mono f-dim" font-size="11.5">"2023"</text>
                <text x="89" y="168" class="f-mid f-mono f-dim" font-size="11.5">"2024"</text>
                <text x="170" y="168" class="f-mid f-mono f-dim" font-size="11.5">"2025"</text>
                <text x="251" y="168" class="f-mid f-mono f-dim" font-size="11.5">"2026"</text>
                <text x="352" y="168" class="f-end f-mono f-dim" font-size="11.5">"2027"</text>
                <rect x="72" y="178" width="102" height="8" class="seg seg-e"></rect>
                <path d="M169 175 L179 189 M179 175 L169 189" class="ln-x"></path>
                <text x="8" y="204" class="f-dim" font-size="11.5">"EO 14110 · Oct '23 — revoked Jan '25"</text>
                <circle cx="174" cy="222" r="4" class="dot-cher"></circle>
                <text x="182" y="226" font-size="11.5">"EO 14179 · Jan '25"</text>
                <circle cx="222" cy="244" r="4" class="dot-cher"></circle>
                <text x="8" y="266" font-size="11.5">"AI Action Plan + EOs · Jul '25"</text>
                <text x="8" y="280" class="f-mono f-brass" font-size="12.5">"US — STATES"</text>
                <line x1="8" y1="294" x2="352" y2="294" class="grid"></line>
                <text x="8" y="308" class="f-mono f-dim" font-size="11.5">"2023"</text>
                <text x="89" y="308" class="f-mid f-mono f-dim" font-size="11.5">"2024"</text>
                <text x="170" y="308" class="f-mid f-mono f-dim" font-size="11.5">"2025"</text>
                <text x="251" y="308" class="f-mid f-mono f-dim" font-size="11.5">"2026"</text>
                <text x="352" y="308" class="f-end f-mono f-dim" font-size="11.5">"2027"</text>
                <text x="8" y="330" class="f-dim" font-size="11.5">"Colorado — signed May '24 · effective '26"</text>
                <rect x="119" y="336" width="169" height="7" class="seg seg-f"></rect>
                <text x="8" y="362" class="f-dim" font-size="11.5">"Texas — signed Jun '25 · effective Jan '26"</text>
                <rect x="208" y="368" width="43" height="7" class="seg seg-g"></rect>
                <text x="8" y="394" class="f-dim" font-size="11.5">"California — signed Sep '25 · effective Jan '26"</text>
                <rect x="235" y="400" width="16" height="7" class="seg seg-g"></rect>
            </svg>
            <figcaption>
                "The law on one axis: the EU phases in by date certain; US federal policy "
                "turns on executive action — one order revoked, its replacement and an "
                "action plan inside 2025 — and the states sign in 2024–25 and switch on "
                "through 2026."
            </figcaption>
        </figure>
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

            <EraDiagram />

            <h2 class="band-label">"THE AGENT STACK"</h2>
            <StackDiagram />
            <McpA2aDiagram />

            <h2 class="band-label">"INSIDE A TRANSFORMER"</h2>
            <TransformerExplorer />

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
            <LawDiagram />

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
