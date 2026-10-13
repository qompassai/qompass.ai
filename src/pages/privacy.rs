use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Privacy() -> impl IntoView {
    view! {
        <Title text="Privacy - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — PRIVACY"</p>
            <h1>"Privacy Policy"</h1>
            <div class="prose">
                <p>
                    "This policy covers Light Show (" <span class="mono-inline">"ai.qompass.lightshow"</span>
                    "), a game published by Qompass AI. It is short because there is little to say: "
                    "the game collects nothing."
                </p>
                <h2>"What the game collects"</h2>
                <p>
                    "Nothing. Light Show collects no personal data and shares no data with "
                    "anyone. It contains no analytics or advertising software, works fully "
                    "offline, and requests no device permissions. No account is required to play."
                </p>
                <h2>"Where your progress lives"</h2>
                <p>
                    "Game progress is stored only on your own device. Nothing about your play "
                    "is transmitted to Qompass AI or to any third party."
                </p>
                <h2>"Changes"</h2>
                <p>
                    "If this policy ever changes materially, the updated policy will be posted "
                    "on this page."
                </p>
                <h2>"Contact"</h2>
                <p>
                    "Questions about this policy: "
                    <a href="mailto:map@qompass.ai">"map@qompass.ai"</a>
                    "."
                </p>
                <p>"Effective October 12, 2026."</p>
            </div>
        </section>
    }
}
