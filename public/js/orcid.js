// Live ORCID refresh: the page ships with a build-time snapshot of the works
// lists (real HTML, good for no-JS and crawlers); this island then re-fetches
// the public record and re-renders the lists if ORCID has anything newer.
// Works are split the same way the snapshot is: publications in #orcid-works,
// software (self-published, DOI'd) in #orcid-software.
// Source: ORCID public API v3.0 — no key, CORS-open for public data.
const ORCID_ID = "0000-0002-0302-4812";
const list = document.getElementById("orcid-works");
const softwareList = document.getElementById("orcid-software");
const status = document.getElementById("orcid-status");

function extractDoi(w) {
  for (const ext of w["external-ids"]?.["external-id"] || []) {
    if (ext["external-id-type"] === "doi") {
      return (ext["external-id-value"] || "").replace(/^https?:\/\/doi\.org\//, "");
    }
  }
  return "";
}

function workItem(w, withDoi) {
  const li = document.createElement("li");
  li.className = "record-item";
  const y = document.createElement("span");
  y.className = "record-year";
  y.textContent = w.year;
  const t = document.createElement("span");
  t.className = "record-title";
  const a = document.createElement("a");
  a.href = w.url;
  a.textContent = w.title;
  t.append(a);
  li.append(y, t);
  if (withDoi) {
    if (w.doi) {
      const d = document.createElement("span");
      d.className = "record-doi";
      d.append("DOI: ");
      const da = document.createElement("a");
      da.href = "https://doi.org/" + w.doi;
      da.textContent = w.doi;
      d.append(da);
      li.append(d);
    }
  } else {
    const v = document.createElement("span");
    v.className = "record-venue";
    v.textContent = w.venue;
    li.append(v);
  }
  return li;
}

if (list) {
  fetch(`https://pub.orcid.org/v3.0/${ORCID_ID}/works`, {
    headers: { Accept: "application/json" },
  })
    .then((r) => {
      if (!r.ok) throw new Error(`ORCID ${r.status}`);
      return r.json();
    })
    .then((data) => {
      const works = [];
      for (const group of data.group || []) {
        const w = (group["work-summary"] || [])[0];
        if (!w) continue;
        const title = w.title?.title?.value?.trim();
        if (!title) continue;
        const venue = w["journal-title"]?.value || w.type || "";
        const type = w.type || "";
        const year = w["publication-date"]?.year?.value || "";
        let doi = extractDoi(w);
        let url = w.url?.value || "";
        if (!url && doi) {
          url = "https://doi.org/" + doi;
        }
        if (!doi && url) {
          const m = url.match(/^https?:\/\/doi\.org\/(.+)$/);
          if (m) doi = m[1];
        }
        works.push({ title, venue, year, type, doi, url: url || `https://orcid.org/${ORCID_ID}` });
      }
      works.sort((a, b) => (b.year || "").localeCompare(a.year || ""));
      list.replaceChildren(
        ...works.filter((w) => w.type !== "software").map((w) => workItem(w, false))
      );
      if (softwareList) {
        softwareList.replaceChildren(
          ...works.filter((w) => w.type === "software").map((w) => workItem(w, true))
        );
      }
      if (status) {
        status.textContent = `Live from ORCID · ${works.length} works · synced ${new Date().toLocaleDateString()}`;
      }
    })
    .catch(() => {
      if (status) status.textContent = "Showing the build-time snapshot (live refresh unavailable).";
    });
}
