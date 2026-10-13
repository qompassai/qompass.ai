// deno-lint-ignore-file
// Browser script (On AI page). Deno lint rules target the Deno runtime and do not apply here.
// Timeline track filters. The timeline is fully server-rendered — this
// script is an enhancement only: without it, every entry stays visible.
document.addEventListener("DOMContentLoaded", () => {
  const buttons = document.querySelectorAll(".tl-filter");
  const entries = document.querySelectorAll(".tl-entry");
  const years = document.querySelectorAll(".tl-year");
  if (!buttons.length || !entries.length) return;
  buttons.forEach((button) => {
    button.addEventListener("click", () => {
      const filter = button.getAttribute("data-filter");
      buttons.forEach((b) => b.setAttribute("aria-pressed", b === button ? "true" : "false"));
      entries.forEach((entry) => {
        const show = filter === "all" || entry.getAttribute("data-track") === filter;
        entry.style.display = show ? "" : "none";
      });
      years.forEach((year) => {
        let visible = false;
        let node = year.nextElementSibling;
        while (node && !node.classList.contains("tl-year")) {
          if (node.classList.contains("tl-entry") && node.style.display !== "none") {
            visible = true;
            break;
          }
          node = node.nextElementSibling;
        }
        year.style.display = visible ? "" : "none";
      });
    });
  });
});

// — Transformer explorer ————————————————————————————————————————
// Enhancement over the server-rendered shell in the "Inside a
// Transformer" section. On first open it fetches the recorded GPT-2
// (small) activations from /data/transformer-explorer.json (generated
// by scripts/generate-transformer-explorer.py) and builds the
// widgets. All arithmetic is local; nothing is sent anywhere.
document.addEventListener("DOMContentLoaded", () => {
  const openBtn = document.getElementById("tx-open");
  const root = document.getElementById("transformer-explorer");
  if (!openBtn || !root) return;

  const COL = {
    abyss: "#04070F",
    brass: "#C9A227",
    chart: "#0A1220",
    cherenkov: "#5B8CFF",
    fog: "#8CA3C3",
    line: "#1B2A44",
    raise: "#0E1930",
    soft: "#9DBCFF",
    starlight: "#E8EEF7",
    violet: "#7C5CFF",
  };
  const state = {
    cell: null,
    data: null,
    head: 5,
    layer: 1,
    s: 0,
    temp: 1,
    tok: null,
  };
  const $ = (id) => document.getElementById(id);
  const sent = () => state.data.sentences[state.s];
  const tokText = (t) => (t.t.trim() === "" ? t.t : t.t.trim());

  function ctx2d(canvas, cssW, cssH) {
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.round(cssW * dpr);
    canvas.height = Math.round(cssH * dpr);
    canvas.style.height = cssH + "px";
    const ctx = canvas.getContext("2d");
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    return ctx;
  }

  function attnMatrix() {
    const s = sent();
    const layer = s.attn[state.layer - 1];
    if (state.head !== "mean") return layer[state.head];
    const t = s.tokens.length;
    const out = Array.from({ length: t }, () => new Array(t).fill(0));
    for (const head of layer) {
      for (let i = 0; i < t; i++) {
        for (let j = 0; j < t; j++) out[i][j] += head[i][j] / 12;
      }
    }
    return out;
  }

  function renderPicker() {
    const picker = $("tx-picker");
    picker.innerHTML = "";
    state.data.sentences.forEach((s, i) => {
      const b = document.createElement("button");
      b.type = "button";
      b.className = "tx-chip";
      b.textContent = s.label;
      b.setAttribute("aria-pressed", i === state.s ? "true" : "false");
      b.setAttribute("aria-label", "Show the sentence: " + s.text);
      b.addEventListener("click", () => {
        state.s = i;
        state.cell = null;
        state.tok = null;
        renderAll();
      });
      picker.appendChild(b);
    });
    $("tx-sentence-note").textContent = sent().note;
  }

  function renderTokens() {
    const box = $("tx-tokens");
    box.innerHTML = "";
    sent().tokens.forEach((t, i) => {
      const b = document.createElement("button");
      b.type = "button";
      b.className = "tx-token" + (state.tok === i ? "" : "");
      b.textContent = t.t;
      b.setAttribute("aria-pressed", state.tok === i ? "true" : "false");
      b.setAttribute(
        "aria-label",
        "Token " + (i + 1) + ": " + tokText(t) + " (id " + t.id + ")",
      );
      b.addEventListener("click", () => {
        state.tok = state.tok === i ? null : i;
        renderTokens();
        renderEmb();
        renderAttn();
      });
      box.appendChild(b);
    });
  }

  function renderEmb() {
    const s = sent();
    const t = s.tokens.length;
    // PCA scatter.
    const pca = $("tx-pca");
    const w = pca.clientWidth || 340;
    const ctx = ctx2d(pca, w, 260);
    ctx.clearRect(0, 0, w, 260);
    const pad = 34;
    const xs = s.emb_pca.map((p) => p[0]);
    const ys = s.emb_pca.map((p) => p[1]);
    const [x0, x1] = [Math.min(...xs), Math.max(...xs)];
    const [y0, y1] = [Math.min(...ys), Math.max(...ys)];
    const px = (v) => pad + ((v - x0) / (x1 - x0 || 1)) * (w - 2 * pad);
    const py = (v) => 260 - pad - ((v - y0) / (y1 - y0 || 1)) * (260 - 2 * pad);
    ctx.strokeStyle = COL.line;
    ctx.strokeRect(pad, pad, w - 2 * pad, 260 - 2 * pad);
    ctx.font = "10px 'JetBrains Mono', monospace";
    s.emb_pca.forEach((p, i) => {
      const cx = px(p[0]);
      const cy = py(p[1]);
      const sel = state.tok === i;
      ctx.beginPath();
      ctx.arc(cx, cy, sel ? 5 : 3.5, 0, Math.PI * 2);
      ctx.fillStyle = sel ? COL.brass : COL.cherenkov;
      ctx.fill();
      ctx.fillStyle = sel ? COL.starlight : COL.fog;
      ctx.fillText(tokText(s.tokens[i]), cx + 7, cy + 3);
    });
    // Embedding slice strip: T rows x 48 dims, diverging color.
    const slice = $("tx-slice");
    const sw = slice.clientWidth || 340;
    const rowH = Math.max(14, Math.floor(240 / t));
    const sh = rowH * t + 20;
    const sctx = ctx2d(slice, sw, sh);
    sctx.clearRect(0, 0, sw, sh);
    let maxAbs = 0.001;
    for (const row of s.emb_slice) {
      for (const v of row) maxAbs = Math.max(maxAbs, Math.abs(v));
    }
    const labelW = 64;
    const cellW = (sw - labelW) / 48;
    sctx.font = "10px 'JetBrains Mono', monospace";
    s.emb_slice.forEach((row, i) => {
      const y = 10 + i * rowH;
      row.forEach((v, d) => {
        const a = Math.min(1, Math.abs(v) / maxAbs);
        sctx.fillStyle =
          v >= 0
            ? "rgba(91,140,255," + a.toFixed(2) + ")"
            : "rgba(124,92,255," + a.toFixed(2) + ")";
        sctx.fillRect(labelW + d * cellW, y, Math.ceil(cellW), rowH - 1);
      });
      sctx.fillStyle = state.tok === i ? COL.brass : COL.fog;
      sctx.fillText(tokText(s.tokens[i]), 4, y + rowH - 5);
      if (state.tok === i) {
        sctx.strokeStyle = COL.brass;
        sctx.strokeRect(1, y, sw - 2, rowH - 1);
      }
    });
  }

  function renderAttn() {
    const note = $("tx-attn-note");
    const canvas = $("tx-attn");
    $("tx-layer-label").textContent =
      state.layer === 0
        ? "Layer 0 — embedding"
        : "Layer " + state.layer + " of 12";
    $("tx-layer-range").value = state.layer;
    note.hidden = state.layer !== 0;
    canvas.style.display = state.layer === 0 ? "none" : "";
    if (state.layer === 0) {
      $("tx-attn-readout").textContent =
        "Attention starts at layer 1 — step forward to see the first matrix.";
      return;
    }
    const s = sent();
    const t = s.tokens.length;
    const mat = attnMatrix();
    const size = Math.min(canvas.clientWidth || 340, 560);
    const labelW = 62;
    const topH = 26;
    const plot = size - labelW;
    const cell = plot / t;
    const ctx = ctx2d(canvas, size, topH + plot);
    ctx.clearRect(0, 0, size, topH + plot);
    ctx.font = "10px 'JetBrains Mono', monospace";
    for (let i = 0; i < t; i++) {
      for (let j = 0; j < t; j++) {
        const x = labelW + j * cell;
        const y = topH + i * cell;
        if (j > i) {
          ctx.fillStyle = COL.abyss;
          ctx.fillRect(x, y, cell, cell);
          ctx.strokeStyle = COL.line;
          ctx.beginPath();
          ctx.moveTo(x, y + cell);
          ctx.lineTo(x + cell, y);
          ctx.stroke();
        } else {
          const wgt = mat[i][j];
          ctx.fillStyle =
            "rgba(91,140,255," + Math.sqrt(Math.max(0, wgt)).toFixed(3) + ")";
          ctx.fillRect(x, y, cell, cell);
        }
      }
    }
    // Hover/pin cross-highlight.
    if (state.cell) {
      const { q, k } = state.cell;
      ctx.fillStyle = "rgba(201,162,39,0.13)";
      ctx.fillRect(labelW, topH + q * cell, plot, cell);
      ctx.fillRect(labelW + k * cell, topH, cell, plot);
      ctx.strokeStyle = COL.brass;
      ctx.lineWidth = 2;
      ctx.strokeRect(labelW + k * cell, topH + q * cell, cell, cell);
      ctx.lineWidth = 1;
    }
    if (state.tok !== null) {
      ctx.strokeStyle = COL.soft;
      ctx.strokeRect(labelW, topH + state.tok * cell, plot, cell);
    }
    // Axis labels.
    ctx.fillStyle = COL.fog;
    for (let i = 0; i < t; i++) {
      ctx.textAlign = "right";
      ctx.fillStyle = state.tok === i ? COL.brass : COL.fog;
      ctx.fillText(tokText(s.tokens[i]), labelW - 5, topH + i * cell + cell / 2 + 3);
      ctx.save();
      ctx.translate(labelW + i * cell + cell / 2 + 3, topH - 6);
      ctx.rotate(-0.6);
      ctx.textAlign = "left";
      ctx.fillText(tokText(s.tokens[i]), 0, 0);
      ctx.restore();
    }
    ctx.textAlign = "left";
    renderAttnReadout();
  }

  function renderAttnReadout() {
    const out = $("tx-attn-readout");
    if (state.layer === 0) return;
    const headLabel =
      state.head === "mean" ? "mean of 12 heads" : "head " + (state.head + 1);
    if (!state.cell) {
      out.textContent =
        "Layer " + state.layer + ", " + headLabel +
        ". Touch a cell: rows listen, columns are listened to.";
      return;
    }
    const s = sent();
    const { q, k } = state.cell;
    const wgt = attnMatrix()[q][k];
    out.textContent =
      "Token " + (q + 1) + " \u201C" + tokText(s.tokens[q]) + "\u201D attends to token " +
      (k + 1) + " \u201C" + tokText(s.tokens[k]) + "\u201D with weight " +
      wgt.toFixed(3) + " (layer " + state.layer + ", " + headLabel + ").";
  }

  function renderLens() {
    const s = sent();
    const box = $("tx-lens");
    box.innerHTML = "";
    const greedyId = s.top50[0].id;
    s.logit_lens.forEach((row, l) => {
      const div = document.createElement("div");
      div.className = "tx-lens-row";
      const lab = document.createElement("span");
      lab.className = "tx-lens-layer";
      lab.textContent = "Layer " + l;
      div.appendChild(lab);
      const top = document.createElement("span");
      top.className =
        "tx-lens-top" + (row[0].id === greedyId ? " tx-converged" : "");
      top.textContent = row[0].t;
      div.appendChild(top);
      const rest = document.createElement("span");
      rest.className = "tx-lens-rest";
      rest.textContent = row[1].t + " · " + row[2].t;
      div.appendChild(rest);
      if (l === 12) {
        const tag = document.createElement("span");
        tag.className = "tx-lens-tag";
        tag.textContent = "the model's actual output";
        div.appendChild(tag);
      }
      box.appendChild(div);
    });
    // Residual-stream norms heat strip: 13 layers x T tokens.
    const canvas = $("tx-norms");
    const s2 = sent();
    const t = s2.tokens.length;
    const w = Math.min(canvas.clientWidth || 340, 560);
    const labelW = 62;
    const rowH = 10;
    const h = 13 * rowH + 8;
    const ctx = ctx2d(canvas, w, h);
    ctx.clearRect(0, 0, w, h);
    let maxV = 0.001;
    for (const row of s2.res_norms) {
      for (const v of row) maxV = Math.max(maxV, v);
    }
    const cellW = (w - labelW) / t;
    ctx.font = "9px 'JetBrains Mono', monospace";
    s2.res_norms.forEach((row, l) => {
      row.forEach((v, i) => {
        const a = Math.min(1, v / maxV);
        ctx.fillStyle = "rgba(201,162,39," + (0.08 + 0.92 * a).toFixed(2) + ")";
        ctx.fillRect(labelW + i * cellW, 4 + l * rowH, Math.ceil(cellW) - 1, rowH - 2);
      });
      ctx.fillStyle = COL.fog;
      ctx.textAlign = "right";
      ctx.fillText("L" + l, labelW - 5, 4 + l * rowH + rowH - 2);
    });
    ctx.textAlign = "left";
  }

  function renderOutput() {
    const s = sent();
    const entries = s.top50.map((e) => ({
      label: e.t,
      logit: e.logit,
      tail: false,
    }));
    entries.push({ label: "all other tokens", logit: s.tail_logit, tail: true });
    const scaled = entries.map((e) => e.logit / state.temp);
    const m = Math.max(...scaled);
    const exps = scaled.map((v) => Math.exp(v - m));
    const z = exps.reduce((a, b) => a + b, 0);
    const probs = entries.map((e, i) => ({ ...e, p: exps[i] / z }));
    $("tx-temp-label").textContent = state.temp.toFixed(2);
    const fmt = (p) =>
      p >= 0.001 ? (p * 100).toFixed(1) + "%" : (p * 100).toFixed(2) + "%";
    const box = $("tx-bars");
    box.innerHTML = "";
    probs.slice(0, 10).forEach((e, rank) => {
      const row = document.createElement("div");
      row.className = "tx-bar-row";
      const tok = document.createElement("span");
      tok.className = "tx-bar-tok" + (rank === 0 ? " tx-greedy" : "");
      tok.textContent = e.tail ? "all other tokens" : e.label;
      row.appendChild(tok);
      const track = document.createElement("div");
      track.className = "tx-bar-track";
      const fill = document.createElement("div");
      fill.className =
        "tx-bar-fill" + (rank === 0 ? " tx-greedy" : "") + (e.tail ? " tx-tail" : "");
      fill.style.width = Math.max(0.4, e.p * 100) + "%";
      track.appendChild(fill);
      row.appendChild(track);
      const pct = document.createElement("span");
      pct.className = "tx-bar-pct";
      pct.textContent = fmt(e.p);
      row.appendChild(pct);
      box.appendChild(row);
    });
    let cum = 0;
    let n = 0;
    for (const e of probs) {
      cum += e.p;
      n++;
      if (cum >= 0.9) break;
    }
    $("tx-topp").textContent =
      "Top token takes " + fmt(probs[0].p) + " at temperature " +
      state.temp.toFixed(2) + "; the smallest set of entries covering 90% " +
      "of the probability spans " + n + " of 51 (50 tokens + the combined bucket).";
  }

  function renderAll() {
    renderPicker();
    renderTokens();
    renderEmb();
    renderAttn();
    renderLens();
    renderOutput();
  }

  // Controls (wired once; they act on state).
  $("tx-layer-prev").addEventListener("click", () => {
    state.layer = Math.max(0, state.layer - 1);
    renderAttn();
  });
  $("tx-layer-next").addEventListener("click", () => {
    state.layer = Math.min(12, state.layer + 1);
    renderAttn();
  });
  $("tx-layer-range").addEventListener("input", (ev) => {
    state.layer = Number(ev.target.value);
    renderAttn();
  });
  const headSel = $("tx-head");
  for (let h = 0; h < 12; h++) {
    const o = document.createElement("option");
    o.value = h;
    o.textContent = "Head " + (h + 1);
    headSel.appendChild(o);
  }
  const meanOpt = document.createElement("option");
  meanOpt.value = "mean";
  meanOpt.textContent = "Mean of heads";
  headSel.appendChild(meanOpt);
  headSel.value = state.head;
  headSel.setAttribute("aria-label", "Attention head");
  headSel.addEventListener("change", () => {
    state.head = headSel.value === "mean" ? "mean" : Number(headSel.value);
    renderAttn();
  });
  $("tx-temp").addEventListener("input", (ev) => {
    state.temp = Number(ev.target.value);
    renderOutput();
  });

  const attnCanvas = $("tx-attn");
  function cellFromEvent(ev) {
    if (state.layer === 0) return;
    const rect = attnCanvas.getBoundingClientRect();
    const s = sent();
    const t = s.tokens.length;
    const labelW = 62;
    const topH = 26;
    const plot = rect.width - labelW;
    const cell = plot / t;
    const x = ev.clientX - rect.left - labelW;
    const y = ev.clientY - rect.top - topH;
    const k = Math.floor(x / cell);
    const q = Math.floor(y / cell);
    if (q >= 0 && q < t && k >= 0 && k <= q) {
      state.cell = { q, k };
      renderAttn();
    }
  }
  attnCanvas.addEventListener("pointermove", cellFromEvent);
  attnCanvas.addEventListener("pointerdown", cellFromEvent);
  attnCanvas.setAttribute("tabindex", "0");
  attnCanvas.setAttribute(
    "aria-label",
    "Attention matrix. Use arrow keys to move between cells; the readout below reports the exact weight.",
  );
  attnCanvas.addEventListener("keydown", (ev) => {
    if (state.layer === 0) return;
    const t = sent().tokens.length;
    const cur = state.cell || { q: 0, k: 0 };
    const next = { ...cur };
    if (ev.key === "ArrowDown") next.q = Math.min(t - 1, cur.q + 1);
    else if (ev.key === "ArrowUp") next.q = Math.max(0, cur.q - 1);
    else if (ev.key === "ArrowRight") next.k = Math.min(cur.q, cur.k + 1);
    else if (ev.key === "ArrowLeft") next.k = Math.max(0, cur.k - 1);
    else return;
    ev.preventDefault();
    state.cell = next;
    renderAttn();
  });
  let resizeTimer = null;
  window.addEventListener("resize", () => {
    if (!state.data) return;
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
      renderEmb();
      renderAttn();
      renderLens();
    }, 150);
  });

  openBtn.addEventListener("click", () => {
    const status = $("tx-status");
    openBtn.disabled = true;
    status.textContent = "Loading the recorded activations…";
    fetch("/data/transformer-explorer.json")
      .then((r) => {
        if (!r.ok) throw new Error("HTTP " + r.status);
        return r.json();
      })
      .then((data) => {
        state.data = data;
        openBtn.hidden = true;
        $("tx-body").hidden = false;
        status.textContent =
          "Loaded: real GPT-2 (small) activations for " +
          data.sentences.length + " sentences.";
        const meta = data.meta;
        $("tx-prov").textContent =
          "Recorded from " + meta.model + " (revision " +
          meta.revision.slice(0, 7) + "), " + meta.layers + " layers · " +
          meta.heads + " heads · " + meta.d_model +
          " dimensions, by " + meta.generated_by +
          " in this site's repository.";
        renderAll();
      })
      .catch(() => {
        openBtn.disabled = false;
        status.textContent =
          "The explorer data could not be loaded — the rest of the page is unaffected.";
      });
  });
});
