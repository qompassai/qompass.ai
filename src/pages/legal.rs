use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Legal() -> impl IntoView {
    view! {
        <Title text="Legal - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — LEGAL"</p>
            <h1>"Legal"</h1>
            <div class="prose">
                <p>
                    "This page is the legal chart for qompass.ai — three documents in one "
                    "place. Privacy covers how this website treats the people who visit it. "
                    "Accessibility states the standard the site is built to. Terms of Service "
                    "govern the site and the software downloaded through it. Effective "
                    "October 13, 2026."
                </p>
                <p>
                    "On this page: " <a href="#privacy">"Privacy"</a> " · "
                    <a href="#accessibility">"Accessibility"</a> " · "
                    <a href="#terms">"Terms of Service"</a> "."
                </p>
                <p>
                    "Related document: Light Show, the game published by Qompass AI, carries "
                    "its own " <a href="/privacy">"privacy policy"</a>
                    " — a separate, shorter document, because the game collects no personal "
                    "data at all. Nothing here changes or replaces it."
                </p>
                <h2 id="privacy">"Privacy"</h2>
                <p>
                    "The short version: this site has no user accounts, sets no tracking "
                    "cookies, and runs no analytics. There is very little about a visitor "
                    "for us to hold."
                </p>
                <p>
                    "Like every web server, ours keeps standard operational logs — requests, "
                    "timestamps, and IP addresses — to serve pages and keep the site secure. "
                    "Those logs are the only records the site itself creates about a visit, "
                    "and we do not use them to build profiles of visitors."
                </p>
                <p>
                    "If you write to us, we receive what you send, and we keep the "
                    "correspondence so we can answer it. The privacy contact of record is "
                    <a href="mailto:map@qompass.ai">"map@qompass.ai"</a>
                    ". There is no mailing list or newsletter. If we ever send commercial "
                    "email, it will comply with the CAN-SPAM Act — accurate headers, a real "
                    "postal address, and an opt-out we honor."
                </p>
                <p>
                    "Software downloads are delivered through GitHub Releases. When you "
                    "follow a download link, GitHub's own privacy policy governs what "
                    "GitHub collects on its service."
                </p>
                <p>
                    "This site is a general-audience site, is not directed to children "
                    "under 13, and does not knowingly collect personal information from "
                    "children under 13."
                </p>
                <p>
                    "Visitors from the European Union and the United Kingdom: this site "
                    "sets no tracking cookies and runs no analytics, so there is almost "
                    "nothing to exercise data rights over. If you email us, you may ask us "
                    "to delete that correspondence, and we will."
                </p>
                <p>
                    "Qompass AI does not collect consumer health data. If that ever "
                    "changes, Washington's My Health My Data Act (RCW 19.373) requires "
                    "consent, a dedicated policy, and deletion rights — and we would "
                    "implement them before collecting."
                </p>
                <p>"We do not collect or enroll biometric identifiers."</p>
                <p>
                    "Neither Washington nor Idaho has a comprehensive consumer-privacy "
                    "statute. The protections that do exist are specific laws, including "
                    "the ones named on this page: in Washington, the My Health My Data Act, "
                    "the breach-notification law, the Consumer Protection Act, and the "
                    "biometric-identifiers law; in Idaho, the breach-notification law and "
                    "the Consumer Protection Act."
                </p>
                <p>
                    "If we ever hold personal information and it is breached, we will "
                    "notify affected Washington residents within the 30 days Washington law "
                    "requires (RCW 19.255.010), and the Attorney General when the law "
                    "requires that too. For Idaho residents, we would notify without "
                    "unreasonable delay, as Idaho Code §28-51-105 requires."
                </p>
                <h2 id="accessibility">"Accessibility"</h2>
                <p>
                    "We design and test this site against WCAG 2.1 Level AA, the standard "
                    "courts and the Justice Department use as the benchmark for web "
                    "accessibility."
                </p>
                <p>
                    "That benchmark is shared. WCAG 2.1 AA is also the standard the Justice "
                    "Department mandates for state and local government websites (28 C.F.R. "
                    "Part 35); that rule phases in on April 26, 2027 and April 26, 2028, "
                    "binds public entities rather than private companies, and carries no "
                    "per-violation fine schedule. Washington sets WCAG 2.1 AA as the "
                    "minimum for its own state agencies (WaTech USER-01). We voluntarily "
                    "hold this private site to the same standard."
                </p>
                <p>
                    "We voluntarily follow WCAG 2.1 AA, which is a superset of the WCAG "
                    "2.0 AA standard federal agencies use under Section 508."
                </p>
                <p>
                    "In practice, that means pages built from semantic landmarks, links "
                    "and controls reachable by keyboard, text with real contrast against "
                    "the dark theme, and animation that stays decorative: motion on this "
                    "site is disabled when your system requests reduced motion."
                </p>
                <p>
                    "This is a small, hand-built site, and it has not been through a "
                    "formal third-party accessibility audit — so this page states a "
                    "standard we work to, not a certification we hold. If any part of this "
                    "site is hard for you to use, tell us and we will fix it."
                </p>
                <p>
                    "Accessibility contact: "
                    <a href="mailto:map@qompass.ai">"map@qompass.ai"</a>
                    "."
                </p>
                <h2 id="terms">"Terms of Service"</h2>
                <p>
                    "These terms govern this website and the software downloads linked "
                    "from it. The site is published by Qompass AI of Spokane, Washington. "
                    "It offers no user accounts and does not host user-submitted content."
                </p>
                <p>
                    "Every software download offered here is licensed under the Apache "
                    "License 2.0 and delivered through GitHub Releases. The license text "
                    "governs the software; these terms incorporate the license by "
                    "reference rather than restating it. Read the "
                    <a href="https://opensource.org/license/apache-2.0">
                        "full text of the Apache License 2.0"
                    </a>
                    "."
                </p>
                <p>
                    <strong>"Trademarks. "</strong>
                    "The Apache License 2.0 does not grant any right to use Qompass AI's "
                    "names or marks (Apache License §6)."
                </p>
                <p>
                    <strong>"Warranty. "</strong>
                    "All downloads are provided \"AS IS\" without warranties of any kind "
                    "(Apache License §7). You alone judge whether the software fits your "
                    "purpose, and you assume the risk of using it."
                </p>
                <p>
                    <strong>"Liability. "</strong>
                    "To the extent the Apache License applies, no contributor is liable "
                    "for damages arising from use of the software (Apache License §8)."
                </p>
                <p>
                    "The policy landscape these terms sit in is mostly federal, and mostly "
                    "recent. The 2023 federal AI executive order (EO 14110) was revoked in "
                    "January 2025. Federal AI policy since January 2025 (EO 14179) directs "
                    "federal agencies; it does not impose obligations on private companies "
                    "like Qompass AI. Our design principles are consistent in spirit with "
                    "the trustworthy-AI principles the federal government applies to "
                    "itself (EO 13960)."
                </p>
                <p>
                    "We note the federal AI Action Plan (July 2025) as part of that "
                    "landscape; it is guidance to agencies, not a law we certify against. "
                    "Federal procurement rules for AI (EO 14319) apply to vendors selling "
                    "AI to the government; Qompass AI does not currently sell AI systems "
                    "to the federal government."
                </p>
                <p>
                    "An executive order cannot by itself preempt state law; Washington and "
                    "Idaho statutes remain in force unless Congress legislates or a court "
                    "rules otherwise. Two enacted state laws are on the horizon; neither "
                    "is in force yet. Washington's AI companion chatbot law (HB 2225) "
                    "takes effect January 1, 2027, and will require disclosure and safety "
                    "protocols for AI companions. The law has no penalty schedule of its "
                    "own: a violation is a per se violation of the Washington Consumer "
                    "Protection Act, under which the Attorney General may recover a civil "
                    "penalty of up to $7,500 per violation — with an additional $5,000 "
                    "enhancement where a violation targets or impacts people by "
                    "demographic characteristics such as age — and an injured person may "
                    "sue for actual damages, which the court may increase up to three "
                    "times, the increased award capped at $25,000, plus costs and a "
                    "reasonable attorney's fee. Qompass AI does not offer an AI companion "
                    "chatbot. Idaho's Conversational AI Safety Act takes effect July 1, "
                    "2027; it sets safety standards for public conversational AI services "
                    "and is written to exclude developer- and business-focused tools of "
                    "the kind Qompass AI builds. Its penalties are its own: the Idaho "
                    "Attorney General may seek an injunction and civil penalties of "
                    "$1,000 per violation, up to $500,000 per operator, or actual damages "
                    "if greater — and the act creates no private right of action. "
                    "Qompass AI does not operate a conversational AI service."
                </p>
                <p>
                    "If these terms change materially, the updated version will be posted "
                    "on this page with a new effective date."
                </p>
                <p>
                    "Questions about these terms: "
                    <a href="mailto:map@qompass.ai">"map@qompass.ai"</a>
                    "."
                </p>
            </div>
        </section>
    }
}
