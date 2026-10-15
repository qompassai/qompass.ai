// Render the Field Notes glossary equations to static SVG, once, at
// authoring time — the same offline-generator pattern as
// scripts/generate-transformer-explorer.py. The output is committed;
// nothing here runs in the site build or in the browser.
//
// Input:  content/field-notes/glossary.json (equation TeX transcribed
//         verbatim from the source repo, plus per-term symbol legends)
// Output: content/field-notes/equations.json — one shared <defs> glyph
//         cache for the page, one <use>-based SVG per equation, and the
//         legend entries resolved for each equation.
//
// The legend gate: every equation is parsed to MathML and every
// identifier, text run, and non-elementary operator in it must be
// covered by that equation's legend. One uncovered symbol fails the
// run and nothing is written.
//
// Requires mathjax-full 3.2.2 (Apache-2.0), installed outside the repo:
//   npm install --prefix /tmp/mj --ignore-scripts mathjax-full@3.2.2
//   node scripts/generate-field-notes-equations.cjs /tmp/mj/node_modules
'use strict';
const fs = require('node:fs');
const path = require('node:path');

const modules = process.argv[2];
if (!modules) {
  console.error('usage: generate-field-notes-equations.cjs <node_modules dir holding mathjax-full>');
  process.exit(2);
}
const mj = (p) => require(path.join(modules, 'mathjax-full', 'js', p));
const { mathjax } = mj('mathjax.js');
const { TeX } = mj('input/tex.js');
const { SVG } = mj('output/svg.js');
const { liteAdaptor } = mj('adaptors/liteAdaptor.js');
const { RegisterHTMLHandler } = mj('handlers/html.js');
const { AllPackages } = mj('input/tex/AllPackages.js');
const { STATE } = mj('core/MathItem.js');

const root = path.join(__dirname, '..');
const glossaryPath = path.join(root, 'content', 'field-notes', 'glossary.json');
const outPath = path.join(root, 'content', 'field-notes', 'equations.json');

const adaptor = liteAdaptor();
RegisterHTMLHandler(adaptor);
const tex = new TeX({ packages: AllPackages });
const svg = new SVG({ fontCache: 'global' });
const doc = mathjax.document('', { InputJax: tex, OutputJax: svg });
svg.fontCache.clearCache();

// Operators every reader is assumed to know; anything else needs a legend.
const ELEMENTARY = new Set([
  '=', '+', '−', '-', '×', '⋅', '·', '(', ')', '[', ']', '{', '}', ',', '.', '/', '!',
  '≈', '≠', '⏟', '⁡', '⁢', '⁣', ':',
]);

/** Concatenated text of every token under a node. */
function text(node) {
  if (node.isToken) return node.getText().replace(/ /g, ' ').trim();
  return (node.childNodes || []).filter(Boolean).map(text).join('');
}

/**
 * The symbol tokens of a TeX string: identifiers, text runs, and
 * non-elementary operators. A subscripted name is one symbol (n_1 and
 * n_2 are different quantities), so base and subscript form a single
 * token; a superscript is an exponent and is read through.
 */
function tokens(source) {
  const found = new Set();
  const visit = (n) => {
    if (!n) return;
    if (n.kind === 'msub' || n.kind === 'msubsup') {
      const [base, sub, sup] = n.childNodes;
      found.add(`sub:${text(base)}_${text(sub)}`);
      if (n.kind === 'msubsup') visit(sup);
      return;
    }
    if (n.isToken) {
      const t = text(n);
      if (t === '') return;
      if (n.kind === 'mi') {
        found.add(`mi:${t}:${n.attributes.get('mathvariant') || ''}`);
      } else if (n.kind === 'mtext') {
        found.add(`mtext:${t}`);
      } else if (n.kind === 'mo' && !ELEMENTARY.has(t)) {
        found.add(`mo:${t}`);
      }
      return;
    }
    for (const child of n.childNodes || []) visit(child);
  };
  visit(doc.convert(source, { display: true, end: STATE.CONVERT }));
  return found;
}

function renderSvg(source) {
  const container = doc.convert(source, { display: true });
  let out = adaptor.innerHTML(container);
  out = out
    .replace(/ style="[^"]*"/g, '')
    .replace(/ data-mml-node="[^"]*"/g, '')
    .replace(/ data-c="[^"]*"/g, '')
    .replace(/ data-mjx-texclass="[^"]*"/g, '');
  if (/<style|<script|on\w+=/.test(out)) {
    throw new Error(`unexpected active content in SVG for ${source}`);
  }
  const label = source.replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;');
  return out.replace('<svg ', `<svg class="eq-svg" aria-label="${label}" `);
}

const glossary = JSON.parse(fs.readFileSync(glossaryPath, 'utf8'));
const failures = [];
const equations = {};
let count = 0;
for (const term of glossary.terms) {
  const symbolTokens = term.symbols.map((s) => tokens(s.tex));
  term.symbols.forEach((s, i) => {
    if (!s.meaning || !s.label) failures.push(`${term.term}: symbol ${s.tex} lacks a label or meaning`);
    if (symbolTokens[i].size === 0) failures.push(`${term.term}: symbol ${s.tex} names no symbol`);
  });
  for (const e of term.equations) {
    count += 1;
    const need = tokens(e.tex);
    let legend;
    if (e.legend) {
      legend = e.legend.map((key) => term.symbols.findIndex((s) => s.tex === key));
    } else {
      legend = term.symbols
        .map((_, i) => i)
        .filter((i) => [...symbolTokens[i]].every((t) => need.has(t)));
    }
    const covered = new Set(legend.flatMap((i) => [...symbolTokens[i]]));
    const uncovered = [...need].filter((t) => !covered.has(t));
    if (uncovered.length > 0) failures.push(`${e.id} (${term.term}) ${e.tex}\n    undefined: ${uncovered.join('  ')}`);
    if (need.size > 0 && legend.length === 0) failures.push(`${e.id} (${term.term}) has symbols but an empty legend`);
    if (!e.plain) failures.push(`${e.id} (${term.term}) lacks a plain-language line`);
    equations[e.id] = { svg: renderSvg(e.tex), legend };
  }
}

if (failures.length > 0) {
  console.error(`legend gate FAILED — ${failures.length} problem(s); nothing written:`);
  for (const f of failures) console.error(`  ${f}`);
  process.exit(1);
}

const defs = adaptor.outerHTML(svg.fontCache.getCache())
  .replace(/ data-c="[^"]*"/g, '');
const output = {
  generator: 'scripts/generate-field-notes-equations.cjs with mathjax-full 3.2.2 (TeX in, SVG out, global glyph cache)',
  count,
  defs: `<svg class="eq-defs" aria-hidden="true" focusable="false" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">${defs}</svg>`,
  equations,
};
fs.writeFileSync(outPath, `${JSON.stringify(output, null, 1)}\n`);
console.log(`legend gate passed: ${count} equations, every symbol defined; wrote ${path.relative(root, outPath)}`);
