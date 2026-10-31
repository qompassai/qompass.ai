#!/usr/bin/env python3
"""Generate public/search-index.json — the static index for the
header search bar (Fleet 1.0, 2026-10-31).

Sources are the site's own data, nothing else: the SOLUTIONS and
MCP_SERVERS tables in src/pages/compass_services.rs, the ready app
guides in content/apps/*.toml, the ready study pages in
content/projects/*.toml, and the fixed routes scripts/prerender.sh
ships. The output is bounded by construction — one entry per source
record, summaries truncated to 240 chars — and is committed so the
prerendered site ships it as a plain static file.

Run from the repo root: python3 scripts/generate-search-index.py
"""
import glob
import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "public" / "search-index.json"
MAX_SUMMARY = 240

FIXED = [
    ("Home", "/", "Qompass AI — navigating deep tech's murky waters.", "Page"),
    ("Research", "/research", "The research program and its study pages.", "Page"),
    ("Solutions", "/solutions", "Every solution on the compass, by cardinal.", "Page"),
    ("Apps", "/solutions/apps", "The application solutions.", "Page"),
    ("MCP Servers", "/solutions/mcp-servers", "The MCP server solutions.", "Page"),
    ("Packages", "/solutions/packages", "The LuaRocks package solutions.", "Page"),
    ("MCP estate index", "/solutions/mcp", "The full MCP server roster, with standings.", "Page"),
    ("Standards", "/standards", "The Tiger Style standards and their references.", "Page"),
    ("Analysis", "/analysis", "On AI: analysis and the Transformer explorer.", "Page"),
    ("Field Notes", "/field-notes", "Field service notes and the TIA-598 trainer.", "Page"),
    ("Light Show", "/light-show", "The Light Show game.", "Page"),
    ("About", "/about", "The founder's record and identifiers.", "Page"),
    ("Privacy", "/privacy", "The site privacy policy.", "Page"),
    ("Legal", "/legal", "Privacy, accessibility, and terms of service.", "Page"),
]


def clean(text: str) -> str:
    text = text.replace('\\"', '"').replace("\\n", " ")
    return re.sub(r"\s+", " ", text).strip()[:MAX_SUMMARY]


def main() -> int:
    entries = []
    seen = set()

    def add(title, url, summary, kind):
        if url in seen:
            return
        seen.add(url)
        entries.append(
            {"title": title, "url": url, "summary": clean(summary), "kind": kind}
        )

    for title, url, summary, kind in FIXED:
        add(title, url, summary, kind)

    services = (ROOT / "src/pages/compass_services.rs").read_text()
    sol_re = re.compile(
        r'sol\("([^"]+)",\s*"([^"]+)",\s*"[^"]+",\s*[NEWS],\s*Class::(\w+),\s*(?:[^,]+),\s*"((?:[^"\\]|\\.)*)"\)'
    )
    kind_by_class = {"App": "Solution — App", "Mcp": "Solution — MCP Server", "Package": "Solution — Package"}
    for name, slug, cls, blurb in sol_re.findall(services):
        add(name, f"/solutions/{slug}", blurb, kind_by_class.get(cls, "Solution"))

    mcp_re = re.compile(
        r'mcp\(\s*"([^"]+)",\s*"((?:[^"\\]|\\.)*)",\s*"(?:[^"]+)",\s*"(?:[^"\\]|\\.)*",\s*(?:Some|None)',
        re.S,
    )
    mcp_section = services.split("MCP_SERVERS", 1)[-1]
    for name, role in mcp_re.findall(mcp_section):
        slug = name.replace(" ", "-")
        add(f"{name} (MCP server)", f"/solutions/mcp/{slug}", role, "MCP Server")

    for path in sorted(glob.glob(str(ROOT / "content/apps/*.toml"))):
        with open(path, "rb") as fh:
            guide = tomllib.load(fh)
        if guide.get("ready"):
            add(guide["name"], f"/solutions/{guide['slug']}", guide.get("summary", ""), "Guide")

    for path in sorted(glob.glob(str(ROOT / "content/projects/*.toml"))):
        with open(path, "rb") as fh:
            page = tomllib.load(fh)
        if page.get("ready"):
            add(page["name"], f"/solutions/{page['slug']}", page.get("abstract", ""), "Study")

    entries.sort(key=lambda e: (e["kind"], e["title"].lower()))
    OUT.write_text(json.dumps(entries, indent=1, ensure_ascii=False) + "\n")
    print(f"search-index: {len(entries)} entries -> {OUT} ({OUT.stat().st_size} bytes)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
