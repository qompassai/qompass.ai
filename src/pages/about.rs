use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — THE COMPANY"</p>
            <h1>"About"</h1>
            <p class="page-lede">
                "Qompass AI is the trade name of Amor Fati Labs LLC, a Washington company "
                "founded by Matthew A. Porter — West Point Class of 2011, former Army "
                "intelligence officer, and service-disabled veteran."
            </p>
            <div class="prose">
                <p>
                    "The name is the mission. A compass does not move the ship — it tells the "
                    "truth about where the ship is, in any weather. Qompass AI builds truthful "
                    "instruments for the quantum transition: benchmarks that measure "
                    "post-quantum cryptography instead of marketing it, infrastructure that "
                    "refuses deprecated algorithms by design, and AI systems held to the "
                    "oldest safety standard in medicine — first, do no harm."
                </p>
                <p>
                    "The founder's route to deep tech ran through two other professions that "
                    "punish sloppy evidence: military intelligence — a decade on active duty, "
                    "from a tactical cell on the Korean DMZ to brigade intelligence director, "
                    "decorated with the Bronze Star — and medicine, with years of study at "
                    "WSU's Elson S. Floyd College of Medicine, orthopaedic publications, and "
                    "journal peer review continuing alongside the engineering. The red "
                    "teamer's habit runs through all of it: threat-model first, publish the "
                    "negative results, and let the measurements argue. His full record is on "
                    <a href="/cv">"the CV page"</a>
                    "."
                </p>
                <p>
                    "Everything we ship is open source under Apache-2.0, archived with DOIs, "
                    "and — as of this site — served by our own stack: vongola at the edge, "
                    "volta for keys, bunker for packages."
                </p>
            </div>
        </section>
    }
}
