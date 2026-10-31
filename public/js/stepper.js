// Stepper explainer enhancement (Fleet 1.0, 2026-10-31).
//
// Enhances every [data-stepper] section the server rendered. No
// timers, no polling, no busy-waiting: state changes are driven only
// by user events (input, click, keydown), and the visual transition
// is a CSS opacity/transform animation owned by the stylesheet
// (disabled under prefers-reduced-motion by the stylesheet itself).
// Arrow keys are handled on the component element only — the handler
// fires solely when the stepper or one of its controls has focus, so
// page-level arrow keys are never hijacked.
(function () {
  "use strict";
  function initStepper(root) {
    var panels = Array.prototype.slice.call(root.querySelectorAll("[data-stepper-panel]"));
    var markers = Array.prototype.slice.call(root.querySelectorAll("[data-stepper-marker]"));
    var range = root.querySelector("[data-stepper-range]");
    var count = root.querySelector("[data-stepper-count]");
    var prev = root.querySelector("[data-stepper-prev]");
    var next = root.querySelector("[data-stepper-next]");
    if (!panels.length) return;
    var current = 0;
    function show(i) {
      current = Math.max(0, Math.min(panels.length - 1, i));
      panels.forEach(function (panel, j) {
        var active = j === current;
        panel.classList.toggle("is-active", active);
        panel.setAttribute("aria-hidden", active ? "false" : "true");
      });
      markers.forEach(function (marker, j) {
        marker.classList.toggle("is-active", j === current);
      });
      if (range) range.value = String(current);
      if (count) count.textContent = "Step " + (current + 1) + " of " + panels.length;
    }
    if (range) {
      range.addEventListener("input", function () { show(parseInt(range.value, 10) || 0); });
    }
    markers.forEach(function (marker, j) {
      marker.addEventListener("click", function () { show(j); });
    });
    if (prev) prev.addEventListener("click", function () { show(current - 1); });
    if (next) next.addEventListener("click", function () { show(current + 1); });
    root.addEventListener("keydown", function (event) {
      if (event.key === "ArrowLeft") { show(current - 1); event.preventDefault(); }
      else if (event.key === "ArrowRight") { show(current + 1); event.preventDefault(); }
    });
    show(0);
  }
  function init() {
    document.querySelectorAll("[data-stepper]").forEach(initStepper);
  }
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
