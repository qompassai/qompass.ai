use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Title text="Contact - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — OPEN A CHANNEL"</p>
            <h1>"Contact"</h1>
            <p class="page-lede">
                "One mailbox, answered by the founder. Encrypt it if the matter warrants — "
                "the key is published on our own key server."
            </p>
            <div class="entry-list">
                <article class="entry">
                    <p class="entry-kind">"EMAIL"</p>
                    <h2>
                        <a href="mailto:map@qompass.ai">"map@qompass.ai"</a>
                    </h2>
                    <p>"Research, consulting, and collaboration inquiries."</p>
                </article>
                <article class="entry">
                    <p class="entry-kind">"OPENPGP — WEB KEY DIRECTORY"</p>
                    <h2 class="mono-block">"3410 59A8 D566 DE30 B4B6 7FC5 FC89 A70A C651 68FD"</h2>
                    <p>
                        "Fetch it the modern way: " <span class="mono-inline">"gpg --locate-keys map@qompass.ai"</span>
                        " — served by volta at keys.qompass.ai — or verify the fingerprint "
                        "above against any copy you already hold."
                    </p>
                </article>
                <article class="entry">
                    <p class="entry-kind">"IDENTITY HUB"</p>
                    <h2>"Everywhere else"</h2>
                    <p>
                        <a href="https://orcid.org/0000-0002-0302-4812">"ORCID"</a>
                        " · "
                        <a href="https://github.com/qompassai">"GitHub"</a>
                        " · "
                        <a href="https://huggingface.co/qompass">"Hugging Face"</a>
                        " · "
                        <a href="https://ovou.me/qompass">"Ovou"</a>
                    </p>
                </article>
            </div>
        </section>
    }
}
