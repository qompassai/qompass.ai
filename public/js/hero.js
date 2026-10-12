// Qompass hero: a compass rose drawn as a Bloch sphere — orbital rings at
// quantum angles, rhumb-line bearings, a celestial/particle starfield, and
// light pulses travelling the chart like photons in fiber.
// Three.js is vendored locally (no CDN): /js/vendor/three.module.min.js
import * as THREE from "/js/vendor/three.module.min.js";

const canvas = document.getElementById("qompass-hero");
if (canvas) {
  const CHERENKOV = 0x5b8cff;
  const VIOLET = 0x7c5cff;
  const BRASS = 0xc9a227;
  const STARLIGHT = 0xe8eef7;

  const renderer = new THREE.WebGLRenderer({ canvas, alpha: true, antialias: true });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(42, 2, 0.1, 100);
  camera.position.set(0, 0.6, 7.2);
  camera.lookAt(0, 0, 0);

  const chart = new THREE.Group();
  chart.position.x = 1.9; // leave the left of the hero for the headline
  scene.add(chart);

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

  // Bloch-sphere orbits: three rings, three tilts.
  const orbitA = ring(2.05, CHERENKOV, 0.85);
  const orbitB = ring(2.05, VIOLET, 0.55);
  orbitB.rotation.set(Math.PI / 3, 0, Math.PI / 5);
  const orbitC = ring(2.05, CHERENKOV, 0.4);
  orbitC.rotation.set(-Math.PI / 4, Math.PI / 6, 0);
  chart.add(orbitA, orbitB, orbitC);

  // The flat chart ring (brass) + rhumb lines at 16 bearings.
  const chartRing = ring(2.6, BRASS, 0.5);
  chartRing.rotation.x = Math.PI / 2.35;
  chart.add(chartRing);
  const rhumbMat = new THREE.LineBasicMaterial({ color: BRASS, transparent: true, opacity: 0.16 });
  const rhumbPts = [];
  for (let i = 0; i < 16; i++) {
    const a = (i / 16) * Math.PI * 2;
    rhumbPts.push(new THREE.Vector3(0, 0, 0));
    rhumbPts.push(new THREE.Vector3(Math.cos(a) * 2.6, 0, Math.sin(a) * 2.6));
  }
  const rhumbs = new THREE.LineSegments(
    new THREE.BufferGeometry().setFromPoints(rhumbPts),
    rhumbMat
  );
  rhumbs.rotation.x = Math.PI / 2.35 - Math.PI / 2;
  chart.add(rhumbs);

  // Core: small wireframe sphere — the state vector's home.
  const core = new THREE.LineSegments(
    new THREE.WireframeGeometry(new THREE.SphereGeometry(0.55, 18, 12)),
    new THREE.LineBasicMaterial({ color: STARLIGHT, transparent: true, opacity: 0.55 })
  );
  chart.add(core);

  // Light pulses travelling orbit A.
  const pulseGeo = new THREE.SphereGeometry(0.045, 8, 8);
  const pulseMat = new THREE.MeshBasicMaterial({ color: 0xbfd4ff });
  const pulses = [];
  for (let i = 0; i < 3; i++) {
    const p = new THREE.Mesh(pulseGeo, pulseMat);
    pulses.push({ mesh: p, phase: (i / 3) * Math.PI * 2 });
    chart.add(p);
  }

  // Starfield: celestial fixes and particles in one.
  const starCount = 650;
  const starPos = new Float32Array(starCount * 3);
  for (let i = 0; i < starCount; i++) {
    starPos[i * 3] = (Math.random() - 0.5) * 26;
    starPos[i * 3 + 1] = (Math.random() - 0.5) * 14;
    starPos[i * 3 + 2] = -4 - Math.random() * 7;
  }
  const starGeo = new THREE.BufferGeometry();
  starGeo.setAttribute("position", new THREE.BufferAttribute(starPos, 3));
  const stars = new THREE.Points(
    starGeo,
    new THREE.PointsMaterial({ color: STARLIGHT, size: 0.035, transparent: true, opacity: 0.75 })
  );
  scene.add(stars);

  function resize() {
    const w = canvas.clientWidth || window.innerWidth;
    const h = canvas.clientHeight || 480;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    chart.position.x = w < 760 ? 0 : 1.9;
    chart.position.y = w < 760 ? 1.15 : 0;
  }
  window.addEventListener("resize", resize);
  resize();

  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const clock = new THREE.Clock();
  function tick() {
    const t = clock.getElapsedTime();
    chart.rotation.y = t * 0.07;
    core.rotation.y = -t * 0.22;
    core.rotation.x = t * 0.06;
    stars.rotation.z = t * 0.004;
    for (const p of pulses) {
      const a = t * 0.55 + p.phase;
      p.mesh.position.set(Math.cos(a) * 2.05, 0, Math.sin(a) * 2.05);
    }
    renderer.render(scene, camera);
    if (!reduced) requestAnimationFrame(tick);
  }
  tick();
}
