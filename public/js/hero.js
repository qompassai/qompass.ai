// deno-lint-ignore-file
// Browser module (Three.js hero scene). Deno lint rules target the Deno runtime and do not apply here.
// Qompass hero: a superposition qubit. One particle cloud holds a project
// mark, bursts outward on a swirl into a Bloch-sphere cloud, and
// reconverges into the next mark — the twelve marks, one at a time.
// Reduced motion: the first mark, settled, drawn once; nothing moves.
// Three.js is vendored locally (no CDN): /js/vendor/three.module.min.js
import * as THREE from "/js/vendor/three.module.min.js";

const canvas = document.getElementById("qompass-hero");
if (canvas) {
  const BRASS = new THREE.Color(0xc9a227);
  const CHERENKOV = new THREE.Color(0x5b8cff);
  const VIOLET = new THREE.Color(0x7c5cff);

  const MARKS = [
    "bunker", "diver", "fathom", "ghost", "light-show", "mindthegap",
    "odyssey", "periscope", "phlow", "tko", "volta", "vongola",
  ].map((name) => `/img/projects/${name}.webp`);

  const COUNT = 1800; // particles
  const HALF = 1.45; // half-width of a held mark, world units
  const CLOUD = 2.35; // radius of the superposition cloud
  const GRID = 128; // marks are sampled on a GRID x GRID raster
  const INSET = 8; // raster cells skipped at each edge (rounded corners)
  const BRIGHT = 110; // a cell is part of the mark above this max channel
  const HOLD = 2.6; // seconds a mark is held
  const BURST = 1.1; // seconds outward
  const CONVERGE = 1.5; // seconds inward

  const renderer = new THREE.WebGLRenderer({ canvas, alpha: true, antialias: true });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(42, 2, 0.1, 100);
  camera.position.set(0, 0.3, 7.2);
  camera.lookAt(0, 0, 0);

  const qubit = new THREE.Group();
  qubit.position.x = 1.9; // leave the left of the hero for the headline
  scene.add(qubit);

  function ring(radius, color, opacity) {
    const pts = [];
    for (let i = 0; i <= 128; i++) {
      const a = (i / 128) * Math.PI * 2;
      pts.push(new THREE.Vector3(Math.cos(a) * radius, 0, Math.sin(a) * radius));
    }
    const geo = new THREE.BufferGeometry().setFromPoints(pts);
    const mat = new THREE.LineBasicMaterial({ color, transparent: true, opacity });
    return new THREE.Line(geo, mat);
  }

  // The Bloch sphere the cloud lives on: a brass equator, a violet meridian.
  const equator = ring(CLOUD, BRASS, 0.32);
  equator.rotation.x = 0.28;
  const meridian = ring(CLOUD, VIOLET, 0.22);
  meridian.rotation.set(Math.PI / 2, 0, 0.35);
  qubit.add(equator, meridian);

  // A soft round dot, drawn once, so particles read as light, not squares.
  const dotCanvas = document.createElement("canvas");
  dotCanvas.width = dotCanvas.height = 32;
  const dctx = dotCanvas.getContext("2d");
  const glow = dctx.createRadialGradient(16, 16, 0, 16, 16, 16);
  glow.addColorStop(0, "rgba(255,255,255,1)");
  glow.addColorStop(0.45, "rgba(255,255,255,0.55)");
  glow.addColorStop(1, "rgba(255,255,255,0)");
  dctx.fillStyle = glow;
  dctx.fillRect(0, 0, 32, 32);

  const positions = new Float32Array(COUNT * 3);
  const colors = new Float32Array(COUNT * 3);
  const geo = new THREE.BufferGeometry();
  geo.setAttribute("position", new THREE.BufferAttribute(positions, 3));
  geo.setAttribute("color", new THREE.BufferAttribute(colors, 3));
  const points = new THREE.Points(
    geo,
    new THREE.PointsMaterial({
      size: 0.075,
      map: new THREE.CanvasTexture(dotCanvas),
      vertexColors: true,
      transparent: true,
      depthWrite: false,
      blending: THREE.AdditiveBlending,
    }),
  );
  qubit.add(points);

  // Luminance onto the palette: dim violet, mid cherenkov, bright brass.
  const tint = new THREE.Color();
  function paletteAt(l) {
    if (l < 0.5) return tint.copy(VIOLET).lerp(CHERENKOV, l / 0.5);
    return tint.copy(CHERENKOV).lerp(BRASS, (l - 0.5) / 0.5);
  }

  // One mark as COUNT particle targets: positions on the bright cells of
  // the raster (sampled with replacement, jittered within a cell) and a
  // palette color per particle. Rejects if the mark has no bright cells.
  const raster = document.createElement("canvas");
  raster.width = raster.height = GRID;
  const rctx = raster.getContext("2d", { willReadFrequently: true });
  async function sample(src) {
    const img = new Image();
    img.src = src;
    await img.decode();
    rctx.clearRect(0, 0, GRID, GRID);
    rctx.drawImage(img, 0, 0, GRID, GRID);
    const data = rctx.getImageData(0, 0, GRID, GRID).data;
    const cells = [];
    for (let y = INSET; y < GRID - INSET; y++) {
      for (let x = INSET; x < GRID - INSET; x++) {
        const i = (y * GRID + x) * 4;
        const peak = Math.max(data[i], data[i + 1], data[i + 2]);
        if (peak > BRIGHT) cells.push(x, y, peak);
      }
    }
    if (cells.length === 0) throw new Error(`hero: no bright cells in ${src}`);
    const pos = new Float32Array(COUNT * 3);
    const col = new Float32Array(COUNT * 3);
    const cell = (2 * HALF) / GRID;
    for (let p = 0; p < COUNT; p++) {
      const c = Math.floor(Math.random() * (cells.length / 3)) * 3;
      pos[p * 3] = (cells[c] / GRID - 0.5) * 2 * HALF + Math.random() * cell;
      pos[p * 3 + 1] = (0.5 - cells[c + 1] / GRID) * 2 * HALF - Math.random() * cell;
      pos[p * 3 + 2] = (Math.random() - 0.5) * 0.12;
      const l = (cells[c + 2] - BRIGHT) / (255 - BRIGHT);
      paletteAt(l).toArray(col, p * 3);
    }
    return { pos, col };
  }

  // The superposition cloud: a fresh point in a shell of the Bloch
  // sphere per particle, and a whole number of turns (1 or 2) so every
  // swirl ends unrotated, exactly on the next mark.
  const cloud = new Float32Array(COUNT * 3);
  const turns = new Float32Array(COUNT);
  function scatter() {
    for (let p = 0; p < COUNT; p++) {
      const u = Math.random() * 2 - 1;
      const a = Math.random() * Math.PI * 2;
      const r = CLOUD * (0.75 + 0.25 * Math.random());
      const s = Math.sqrt(1 - u * u);
      cloud[p * 3] = r * s * Math.cos(a);
      cloud[p * 3 + 1] = r * u;
      cloud[p * 3 + 2] = r * s * Math.sin(a);
      turns[p] = Math.random() < 0.7 ? 1 : 2;
    }
  }

  const easeOut = (t) => 1 - Math.pow(1 - t, 3);
  const easeInOut = (t) => (t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2);

  // Place every particle between `from` and `to` at fraction e, turned
  // about the view axis by `phase` of its turns (phase 1 = full turns).
  function place(from, to, e, phase, fromCol, toCol, ce) {
    for (let p = 0; p < COUNT; p++) {
      const i = p * 3;
      const x = from[i] + (to[i] - from[i]) * e;
      const y = from[i + 1] + (to[i + 1] - from[i + 1]) * e;
      const th = phase * turns[p] * Math.PI * 2;
      const c = Math.cos(th);
      const s = Math.sin(th);
      positions[i] = x * c - y * s;
      positions[i + 1] = x * s + y * c;
      positions[i + 2] = from[i + 2] + (to[i + 2] - from[i + 2]) * e;
      for (let k = 0; k < 3; k++) {
        colors[i + k] = fromCol[i + k] + (toCol[i + k] - fromCol[i + k]) * ce;
      }
    }
    geo.attributes.position.needsUpdate = true;
    geo.attributes.color.needsUpdate = true;
  }

  function resize() {
    const w = canvas.clientWidth || window.innerWidth;
    const h = canvas.clientHeight || 480;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    qubit.position.x = w < 760 ? 0 : 1.9;
    qubit.position.y = w < 760 ? 1.15 : 0;
  }
  window.addEventListener("resize", resize);
  resize();

  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  if (reduced) {
    // Static mark: the first mark settled in place, redrawn only on resize.
    sample(MARKS[0]).then(
      ({ pos, col }) => {
        place(pos, pos, 0, 0, col, col, 0);
        renderer.render(scene, camera);
        window.addEventListener("resize", () => renderer.render(scene, camera));
      },
      () => renderer.render(scene, camera),
    );
  } else {
    // State: the mark held (`held`), the mark coming (`ready` once its
    // load, started at the top of each hold, resolves), the phase clock.
    let index = 0;
    let held = null;
    let ready = null;
    let phase = "converge";
    let phaseStart = 0;
    let from = null; // positions the current converge starts from
    let fromCol = null;

    // A mark that fails to load is skipped; a full lap of failures
    // stops trying and leaves the cloud turning where it is.
    let misses = 0;
    const fetchNext = () => {
      const i = index;
      ready = null;
      sample(MARKS[i]).then(
        (t) => { misses = 0; ready = t; },
        () => {
          misses += 1;
          if (misses >= MARKS.length) return;
          index = (i + 1) % MARKS.length;
          fetchNext();
        },
      );
    };

    // Entry: the cloud converges onto the first mark.
    scatter();
    from = cloud;
    fromCol = new Float32Array(COUNT * 3).fill(0.35);
    place(cloud, cloud, 0, 0.5, fromCol, fromCol, 0);
    fetchNext();

    const clock = new THREE.Clock();
    function tick() {
      const t = clock.getElapsedTime();
      const dt = t - phaseStart;
      qubit.rotation.y = Math.sin(t * 0.25) * 0.28;
      equator.rotation.y = t * 0.12;
      meridian.rotation.z = 0.35 + t * 0.05;

      if (phase === "converge") {
        if (!ready) {
          phaseStart = t; // still loading: hold the cloud where it is
        } else {
          const p = Math.min(dt / CONVERGE, 1);
          const e = easeInOut(p);
          place(from, ready.pos, e, 0.5 + 0.5 * e, fromCol, ready.col, e);
          if (p >= 1) {
            held = ready;
            index = (index + 1) % MARKS.length;
            fetchNext();
            phase = "hold";
            phaseStart = t;
          }
        }
      } else if (phase === "hold") {
        if (dt >= HOLD && ready) {
          scatter();
          phase = "burst";
          phaseStart = t;
        }
      } else if (phase === "burst") {
        const p = Math.min(dt / BURST, 1);
        const e = easeOut(p);
        place(held.pos, cloud, e, 0.5 * e, held.col, held.col, 0);
        if (p >= 1) {
          from = cloud;
          fromCol = held.col;
          phase = "converge";
          phaseStart = t;
        }
      }

      renderer.render(scene, camera);
      requestAnimationFrame(tick);
    }
    tick();
  }
}
