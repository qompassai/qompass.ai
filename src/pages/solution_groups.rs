//! Class index pages — `/solutions/apps`, `/solutions/mcp-servers`,
//! `/solutions/packages`.
//!
//! The three classes the main nav's Solutions dropdown differentiates.
//! Each page lists its class alphabetically, in the /solutions entry
//! idiom: mark, kind line, name, blurb, the version linked to its
//! release, and the way through to the solution's own page and source.
use crate::pages::compass_services::{solutions_of_class, version_link, Class, Solution};
use leptos::prelude::*;
use leptos_meta::Title;

/// One class entry, in the fleet's article shape.
#[component]
fn ClassEntry(solution: &'static Solution) -> impl IntoView {
    let source_label = if solution.href.starts_with("https://luarocks.org/") {
        "Module on LuaRocks"
    } else if solution.href.starts_with("/solutions/") {
        "On this site"
    } else {
        "Source on GitHub"
    };
    view! {
        <article class="entry">
            {solution
                .mark()
                .map(|mark| view! { <img class="project-mark" src=mark alt=solution.name /> })}
            <p class="entry-kind">{solution.class.label().to_uppercase()}</p>
            <h2>
                <a href=format!("/solutions/{}", solution.slug)>{solution.name}</a>
            </h2>
            {(!solution.blurb.is_empty()).then(|| view! { <p>{solution.blurb}</p> })}
            <p class="study-meta">
                {solution
                    .version
                    .is_some()
                    .then(|| view! { <span>"Version " {version_link(solution)}</span> })}
                <a href=solution.href>{source_label}</a>
            </p>
        </article>
    }
}

/// One class index page.
#[component]
fn ClassPage(class: Class, bearing: &'static str, lede: &'static str) -> impl IntoView {
    view! {
        <Title text=format!("{} - Qompass AI", class.label()) />
        <section class="page">
            <p class="page-bearing">{bearing}</p>
            <h1>{class.label()}</h1>
            <p class="page-lede">{lede}</p>
            <div class="entry-list">
                {solutions_of_class(class)
                    .into_iter()
                    .map(|s| view! { <ClassEntry solution=s /> })
                    .collect_view()}
            </div>
        </section>
    }
}

/// `/solutions/apps` — the applications.
#[component]
pub fn AppsPage() -> impl IntoView {
    view! {
        <ClassPage
            class=Class::App
            bearing="CHART — THE FLEET, BY CLASS"
            lede="The applications: servers, tools, and programs you run. Each one links to its own page — how it works, its configuration, and its controls."
        />
    }
}

/// `/solutions/mcp-servers` — the MCP servers.
#[component]
pub fn McpServersPage() -> impl IntoView {
    view! {
        <ClassPage
            class=Class::Mcp
            bearing="CHART — THE FLEET, BY CLASS"
            lede="The MCP servers among the solutions: Model Context Protocol servers that put Qompass tools in front of agents — reverse engineering, voice, media conversion, and the agent runtime itself. The full estate roster — twenty servers, with standings — is at /solutions/mcp. The fleet migrated to rmcp, modern-era only, in October 2026."
        />
    }
}

/// `/solutions/packages` — the packages (LuaRocks).
#[component]
pub fn PackagesPage() -> impl IntoView {
    view! {
        <ClassPage
            class=Class::Package
            bearing="CHART — THE FLEET, BY CLASS"
            lede="The packages: LuaRocks modules published under phaedrusflow — the Diver ecosystem in installable pieces, plus the blaze.nvim module. Every version below is the published rock."
        />
    }
}
