// Live ORCID refresh: the page ships with a build-time snapshot of the works
// list (real HTML, good for no-JS and crawlers); this island then re-fetches
// the public record and re-renders the list if ORCID has anything newer.
// Source: ORCID public API v3.0 — no key, CORS-open for public data.
const ORCID_ID = "0000-0002-0302-4812";
const list = document.getElementById("orcid-works");
const status = document.getElementById("orcid-status");

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
        const year = w["publication-date"]?.year?.value || "";
        let url = w.url?.value || "";
        if (!url) {
          for (const ext of w["external-ids"]?.["external-id"] || []) {
            if (ext["external-id-type"] === "doi") {
              url = "https://doi.org/" + ext["external-id-value"];
              break;
            }
          }
        }
        works.push({ title, venue, year, url: url || `https://orcid.org/${ORCID_ID}` });
      }
      works.sort((a, b) => (b.year || "").localeCompare(a.year || ""));
      list.replaceChildren(
        ...works.map((w) => {
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
          const v = document.createElement("span");
          v.className = "record-venue";
          v.textContent = w.venue;
          li.append(y, t, v);
          return li;
        })
      );
      if (status) {
        status.textContent = `Live from ORCID · ${works.length} works · synced ${new Date().toLocaleDateString()}`;
      }
    })
    .catch(() => {
      if (status) status.textContent = "Showing the build-time snapshot (live refresh unavailable).";
    });
}
