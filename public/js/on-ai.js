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
