// deno-lint-ignore-file
// Browser script (project study pages). Deno lint rules target the Deno runtime and do not apply here.
// Qompass project pages: client-side KaTeX rendering + the interactive-slot
// registry. KaTeX is vendored locally (no CDN): /vendor/katex/ (v0.16.11).
// Delimiters are bracket/paren ONLY — never $ — so currency in copy can
// never false-positive as math.

// — KaTeX ————————————————————————————————————————————————————————————————
// Leptos hydration can swap DOM nodes after this script first runs, so
// rendering is attempted at eval, on DOMContentLoaded, on window load, and
// from a short-lived MutationObserver. renderMathInElement is idempotent:
// rendered math no longer contains raw delimiters in its text.
function katexArticle() {
  return document.querySelector(".project-article");
}
function mathPending(article) {
  return article && article.textContent.includes("\\[");
}
function renderMath() {
  const article = katexArticle();
  if (!article || typeof renderMathInElement !== "function") return false;
  if (!mathPending(article)) return true;
  renderMathInElement(article, {
    delimiters: [
      { left: "\\[", right: "\\]", display: true },
      { left: "\\(", right: "\\)", display: false },
    ],
    throwOnError: false,
  });
  return !mathPending(article);
}
// No eval-time render: at script-eval the SSR DOM is parsed but Leptos
// hydration has not run yet, and mutating text nodes before hydration
// claims them is a hydration mismatch. First attempt at DOMContentLoaded;
// the math spans are inner_html (opaque to hydration), and the observer
// below keeps retrying until the delimiters are gone.
document.addEventListener("DOMContentLoaded", renderMath);
window.addEventListener("load", renderMath);
{
  const article = katexArticle();
  if (article && !renderMath()) {
    let timer = 0;
    const observer = new MutationObserver(() => {
      window.clearTimeout(timer);
      timer = window.setTimeout(() => {
        if (renderMath()) observer.disconnect();
      }, 120);
    });
    observer.observe(article, { childList: true, subtree: true, characterData: true });
    window.setTimeout(() => observer.disconnect(), 4000);
  }
}

// — Interactives —————————————————————————————————————————————————————————
// Registry keyed by the `interactive` id in content/projects/<slug>.toml.
// Each mount receives the slot element and builds its own DOM inside it.
const INTERACTIVES = {
  // Phlow stale-frame explorer: cross-play success vs the probability that
  // an agent's snapshot is one workspace-change stale. Exactly the three
  // measured points from the implementation report — no interpolation and
  // no connecting line, because nothing between the points was measured.
  "phlow-stale-explorer": (slot) => {
    const POINTS = [
      {
        p: "0.00",
        success: "1.000",
        sem: "0.000",
        rounds: "0.55",
        y: 1.0,
        err: 0,
        note: "Fresh frames throughout: full agreement at the ceiling — deference over the shared ranking converges in at most one exchange.",
      },
      {
        p: "0.10",
        success: "0.905",
        sem: "0.004",
        rounds: "0.49",
        y: 0.905,
        err: 0.004,
        note: "One frame in ten is stale: success still holds above 0.9.",
      },
      {
        p: "0.30",
        success: "0.774",
        sem: "0.006",
        rounds: "0.41",
        y: 0.774,
        err: 0.006,
        note: "At 30% stale frames the protocol still beats every baseline, but 23 points of success are spent on refusals. Frame freshness is the operational lever.",
      },
    ];
    const NOISE_NOTE =
      "Perception noise σ = 0.15 (measured at p = 0 only): success stays 1.000, but rounds-to-agree rise 0.55 → 0.73. Robust to noise, taxed by staleness.";
    // Plot geometry (viewBox 560x250): margins L46 R14 T16 B32.
    const yOf = (v) => 16 + (1 - v) * 202;
    const XS = [129, 296, 463];
    const dots = POINTS.map((pt, i) => {
      const cy = yOf(pt.y);
      const ey = Math.max(pt.err * 202, pt.err > 0 ? 1.5 : 0);
      const bar = pt.err > 0
        ? `<line class="ix-err" x1="${XS[i]}" y1="${cy - ey}" x2="${XS[i]}" y2="${cy + ey}"/>`
        : "";
      return `${bar}<circle class="ix-point" data-ix-point="${i}" cx="${XS[i]}" cy="${cy}" r="6"/>` +
        `<text class="ix-plot-text" x="${XS[i]}" y="238" text-anchor="middle">p = ${pt.p}</text>`;
    }).join("");
    slot.innerHTML =
      `<div class="ix-controls">` +
      `<div class="ix-slider-row">` +
      `<span class="ix-p-label" data-ix="plabel">p = 0.00</span>` +
      `<input class="ix-slider" data-ix="slider" type="range" min="0" max="2" step="1" value="0" ` +
      `aria-label="Stale-frame probability (measured points)"/>` +
      `</div>` +
      `<div class="ix-marks"><span>fresh frames</span><span>stale frames →</span></div>` +
      `<label class="ix-noise"><input data-ix="noise" type="checkbox"/> ` +
      `perception noise σ = 0.15 (measured at p = 0 only)</label>` +
      `</div>` +
      `<svg class="ix-plot" viewBox="0 0 560 250" role="img" aria-label="Cross-play success at the three measured stale-frame probabilities">` +
      `<rect class="ix-band" x="46" y="${yOf(0.17)}" width="500" height="${yOf(0.12) - yOf(0.17)}"/>` +
      `<line class="ix-chance" x1="46" y1="${yOf(0.2)}" x2="546" y2="${yOf(0.2)}"/>` +
      `<line class="ix-axis" x1="46" y1="16" x2="46" y2="218"/>` +
      `<line class="ix-axis" x1="46" y1="218" x2="546" y2="218"/>` +
      `<text class="ix-plot-text" x="40" y="${yOf(1) + 3}" text-anchor="end">1.0</text>` +
      `<text class="ix-plot-text" x="40" y="${yOf(0.5) + 3}" text-anchor="end">0.5</text>` +
      `<text class="ix-plot-text" x="40" y="${yOf(0) + 3}" text-anchor="end">0.0</text>` +
      `<text class="ix-plot-text" x="540" y="${yOf(0.2) - 6}" text-anchor="end">chance 0.2</text>` +
      `<text class="ix-plot-text" x="540" y="${yOf(0.145) + 3}" text-anchor="end">convention 0.145 ± 0.025</text>` +
      dots +
      `</svg>` +
      `<div class="ix-readout">` +
      `<div><span class="ix-stat-label">Cross-play success</span>` +
      `<span class="ix-stat-value" data-ix="success">1.000 ± 0.000</span></div>` +
      `<div><span class="ix-stat-label">Rounds to agree</span>` +
      `<span class="ix-stat-value" data-ix="rounds">0.55</span></div>` +
      `</div>` +
      `<p class="ix-note" data-ix="note"></p>`;
    const slider = slot.querySelector('[data-ix="slider"]');
    const noise = slot.querySelector('[data-ix="noise"]');
    const plabel = slot.querySelector('[data-ix="plabel"]');
    const success = slot.querySelector('[data-ix="success"]');
    const rounds = slot.querySelector('[data-ix="rounds"]');
    const note = slot.querySelector('[data-ix="note"]');
    const circles = slot.querySelectorAll("[data-ix-point]");
    const update = () => {
      const i = Number(slider.value);
      const pt = POINTS[i];
      plabel.textContent = `p = ${pt.p}`;
      success.textContent = `${pt.success} ± ${pt.sem}`;
      noise.disabled = i !== 0;
      if (i === 0 && noise.checked) {
        rounds.textContent = "0.73";
        note.textContent = NOISE_NOTE;
      } else {
        rounds.textContent = pt.rounds;
        note.textContent = pt.note;
      }
      circles.forEach((c, j) => {
        c.setAttribute("class", j === i ? "ix-point ix-point-active" : "ix-point");
      });
    };
    slider.addEventListener("input", update);
    noise.addEventListener("change", update);
    update();
  },
};

function mountInteractives() {
  document.querySelectorAll("[data-interactive]").forEach((slot) => {
    const mount = INTERACTIVES[slot.getAttribute("data-interactive")];
    // Content-idempotent: skip when the interactive is already built.
    // (No eval-time mount — replacing slot content before Leptos
    // hydration settles would fight the framework for the same nodes;
    // the slot's fallback is inner_html, opaque to hydration, and the
    // load-time pass below mounts after hydration has claimed the page.)
    if (mount && !slot.querySelector(".ix-controls")) {
      mount(slot);
    }
  });
}
document.addEventListener("DOMContentLoaded", mountInteractives);
window.addEventListener("load", mountInteractives);
