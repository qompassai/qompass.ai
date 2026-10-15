//! The how-to under `/light-show`: ten steps from a learning goal to a
//! shipped Android game, each told as the general practice, then what
//! Light Show actually did.
//!
//! Every fact in [`STEPS`] comes from the light-show repo's own record
//! (design doc, build notes, manifest, publish scripts, store listing
//! files) or from the site's existing Light Show pages. Nothing here is
//! a secret, and the `copy_carries_no_secret_smell` test keeps it that
//! way.
//!
//! Progressive enhancement: the server renders every step open, in
//! order, so the whole guide reads with no JavaScript. Once the WASM bundle hydrates, an effect (effects never
//! run on the server) flips `enhanced`, and the list becomes an
//! accordion with prev/next movement and a tally of ticked steps. The
//! state lives in [`Walk`], plain Rust with no Leptos in it, held in a
//! signal: nothing is stored and nothing leaves the browser.
use leptos::html;
use leptos::prelude::*;

pub struct Step {
    pub title: &'static str,
    /// The general practice, for any game.
    pub practice: &'static [&'static str],
    /// What Light Show did, with its real numbers.
    pub example: &'static [&'static str],
    /// Hard external requirements, stated exactly; empty when the step
    /// has none.
    pub requirements: &'static [&'static str],
    /// An onward link for the step, if it has one.
    pub link: Option<(&'static str, &'static str)>,
    /// "You have done this when…" — the step is complete when every
    /// box is ticked.
    pub checks: &'static [&'static str],
}

pub const STEP_COUNT: usize = 10;

pub const STEPS: [Step; STEP_COUNT] = [
    Step {
        title: "Design the learning before the game",
        practice: &[
            "Start from what the player should be able to do when they put the phone down, not from a genre. Write the learning goals first, then choose a mechanic that cannot be won without using each one, and only then dress it. If the mechanic can be won without the skill, the game teaches the mechanic.",
            "Make the win condition the trade's own pass/fail line: a reading inside a window, a constraint satisfied, an answer verified. A real check teaches the real threshold; a score teaches the score.",
        ],
        example: &[
            "Light Show teaches outside-plant (OSP) fiber work, and the design document came before the code. You play a rookie OSP tech. Four companions each own one transmission medium: Séraphine (fiber), Ondine (coax), Linka (wireless), Lattice (Ethernet). Each one teaches her medium through a ten-level track, so picking a companion picks what you learn.",
            "The fiber win check is the link budget: received power, P_rx = P_tx − Σ losses, must land inside the level's dBm window. Running too hot fails just like too much loss, as it does in the field. On Ondine's first coax level, Unity Gain, the distribution point launches at +35 dBmV and the plant eats 34.95 dB before the customer's tap, so the carrier arrives at 0.05 dBmV before any amplifier. The window is [0, 15] dBmV. The 5 dB amplifier lands 5.05 dBmV, inside. The 35 dB amplifier lands 35.05 dBmV, and the picture distorts.",
        ],
        requirements: &[],
        link: None,
        checks: &[
            "You can list what a player should be able to do afterward, in the trade's own words.",
            "Every learning goal maps to a mechanic that cannot be won without it.",
            "The win condition is a real-world pass/fail check, not a score.",
        ],
    },
    Step {
        title: "Build it on your desk first",
        practice: &[
            "Develop on the desktop and ship to the phone. A desktop edit-run loop takes seconds. A phone loop means packaging, installing, and launching every time. Pick an engine that targets both from one codebase, so the phone build is a packaging job, not a port.",
            "Keep the subject's math out of the engine. A plain library with no engine types in it can be unit-tested in milliseconds, and those tests are where you prove the teaching is correct.",
            "Design for both shapes from the first screen. Phones are held in portrait, desktops and tablets run in landscape, and a resizable window can be any shape in between.",
        ],
        example: &[
            "Light Show is written in Rust with the Bevy engine. The daily loop is one command on the desktop: cargo run -p light-show. The link-budget math lives in osp_sim, an engine-free crate with its own tests (cargo test -p osp_sim). A wrong dB figure fails a test before it ever reaches a level.",
            "Screens are designed on a 720 × 1280 portrait canvas. A responsive module cover-fits every full-window backdrop to the window the game actually gets: on spawn, when the texture loads, and on every resize. It also swaps the title art between portrait and landscape versions by window shape. The release gates run locally before any push: the full test suite, clippy with warnings denied, and a format check.",
        ],
        requirements: &[],
        link: None,
        checks: &[
            "The game runs on your desktop with one command.",
            "The subject's math has unit tests that run without the engine.",
            "Every screen fits a portrait phone and a landscape window.",
        ],
    },
    Step {
        title: "Shape it for Android",
        practice: &[
            "Google Play takes an Android App Bundle (AAB) and builds device-specific installs from it. Sideloading and F-Droid use a plain APK. Plan to produce both from the same source.",
            "minSdk is the oldest Android version you will run on. Raise it only when an API you need demands it, and write down which one. targetSdk is the Android behavior you have tested against. Play sets a floor for it and raises that floor every year, so check the current number in Play Console Help before each release.",
            "Request no permission you do not need. Every permission becomes a review question, a line in the Data safety form, and a reason for a player to decline the install. A game that asks for nothing has nothing to explain.",
        ],
        example: &[
            "Light Show sets minSdk 26 (Android 8.0) and targets and compiles against SDK 36. The 26 has a reason: the Android audio library the game links, libaaudio, first exists at API 26. A native build left on the NDK's default platform, API 21, fails to link with \u{201c}library not found: aaudio\u{201d}.",
            "For Play, the game compiles to a native library for three CPU types (arm64-v8a, armeabi-v7a, x86_64). A minimal Gradle project wraps that library behind Android's NativeActivity, and the bundle comes out around 235 MB. The build notes record that apps targeting Android 15 and up must ship native libraries aligned for 16 KB memory pages. The 64-bit Rust targets already align that way, but 32-bit ARM defaults to 4 KB, so the build forces 16 KB for that one target. readelf then confirms every LOAD segment of every library shows 0x4000.",
            "The manifest requests no permissions at all: no network, location, storage, or contacts. The game runs fully offline. That one decision answers the Data safety form (step 7) and keeps F-Droid's anti-features list empty (step 9).",
        ],
        requirements: &[
            "Target API level: since August 31, 2026, new apps and app updates on Google Play must target Android 16 (API 36) or higher, with an extension to November 1, 2026 available on request (Play Console Help, \u{201c}Target API level requirements for Google Play apps\u{201d}).",
        ],
        link: None,
        checks: &[
            "Your build produces an AAB for Play and an APK for everything else.",
            "minSdk and targetSdk are written down, each with its reason.",
            "The merged manifest requests only permissions you can defend, ideally none.",
        ],
    },
    Step {
        title: "Sign it, and guard the key",
        practice: &[
            "Android will not install an update unless it is signed with the same key as the copy already on the phone. That makes the signing key the app's real identity, more than its name or its icon.",
            "Play App Signing splits the job in two. Google holds the app signing key and signs what players download. You hold an upload key and sign what you send to Google. The upload key only proves that an upload came from you.",
            "Treat the keystore as the one file you cannot lose. Make it once, with a long validity, and back it up offline the day you create it. Keep its passwords in a password manager. Never put them in the repository or in chat, and never type them on a command line, where other users of the machine can read them in the process list.",
        ],
        example: &[
            "Light Show uses Play App Signing. The upload keystore was made once, on one machine (RSA 2048, valid for 10,950 days, which is thirty years). The script that makes it has a dry-run mode: it runs the exact keytool command against a throwaway keystore and then deletes it, so the real run is the first time a real key exists. The real run leaves out the password flags, so keytool asks for the passwords on the terminal instead of reading them from arguments.",
            "The Gradle release build declares no signing configuration, on purpose, so an automated build can never produce a signed release. Signing is a separate, human step. One trap worth knowing: an AAB is signed as a JAR, with jarsigner. apksigner treats its input as an APK and cannot sign or verify a bundle. After signing, run jarsigner -verify, then bundletool validate.",
        ],
        requirements: &["Every bundle uploaded to Play must be signed with the upload key registered for that app."],
        link: None,
        checks: &[
            "You can say which key Google holds and which key you hold.",
            "The keystore has an offline backup, and you have checked that the backup reads.",
            "No key, keystore, or password is anywhere in the repository or its history.",
        ],
    },
    Step {
        title: "Open the Play Console and claim the package name",
        practice: &[
            "Register a Google Play developer account, and know whether it is personal or an organization: the two face different rules. A personal account has to run a closed test before it can publish to production (step 8).",
            "Create the app: its name, default language, whether it is an app or a game, and whether it is free or paid.",
            "Choose the package name as if it were permanent, because in practice it is. Play identifies the app by it, and a different package name is a different app, with its own listing, no reviews, and no installs. Build it from a domain you own, reversed.",
        ],
        example: &[
            "Light Show's developer account is phaedrusflow, a personal account. The app was created as Light Show, default language en-US, a game, free, with no ads.",
            "The package name is ai.qompass.lightshow, which is qompass.ai reversed. It is declared in two places that must agree: the Gradle applicationId and the game crate's Android metadata. The publishing checklist names both.",
        ],
        requirements: &[],
        link: None,
        checks: &[
            "Your developer account is set up, and you know whether it is personal or an organization.",
            "The app exists in the Play Console with its name, language, and app-or-game choice.",
            "The package name is final and matches in every build file.",
        ],
    },
    Step {
        title: "Write the store listing",
        practice: &[
            "The listing is your store page, and Play enforces its sizes. Keep the text and images in the repository as files. That way the listing is versioned, reviewable, and checked by a script before you paste anything.",
            "Screenshots should be genuine captures of the game running. The format rules are strict, and a raw phone screen capture is RGBA. Strip the alpha channel losslessly before you upload, or the file is rejected.",
            "Declare AI-generated assets honestly, one asset at a time.",
        ],
        example: &[
            "Light Show keeps its listing as fastlane-format files: title, short and full description, icon, feature graphic, phone screenshots, and one changelog per versionCode. The same files feed both Google Play and F-Droid, so the two listings never drift. A metadata-check script fails if any size or count is off.",
            "The title, \u{201c}Light Show: Fiber Optic Puzzle\u{201d}, is 30 characters. The short description, \u{201c}Route real fiber-optic light from A to B. Splice, split, survive outages\u{201d}, is 72.",
            "Every screenshot is a genuine capture of the game running. The 7-inch and 10-inch tablet sets came from API 36 emulators at 1080 × 1920 and 2560 × 1440, with the alpha stripped and the pixels unchanged. The screenshots are not labeled AI-generated, because they are not. The icon and the feature graphic are AI-generated art, and they are labeled that way.",
        ],
        requirements: &[
            "App name: at most 30 characters. Short description: at most 80. Full description: at most 4,000.",
            "Icon: 512 × 512, 32-bit PNG. Feature graphic: 1024 × 500, JPEG or 24-bit PNG with no alpha.",
            "Screenshots: at least 2 to publish. Every screenshot must be JPEG or 24-bit PNG with no alpha, 320 to 3,840 px per side, with the long side no more than twice the short side, and up to 8 per device type.",
            "Release notes: at most 500 characters per release.",
        ],
        link: None,
        checks: &[
            "Every listing text and image lives in the repository and passes a size check.",
            "Screenshots are real captures, 24-bit, with no alpha.",
            "Each AI-generated asset is declared as AI-generated.",
        ],
    },
    Step {
        title: "Clear the policy gauntlet",
        practice: &[
            "Before a release leaves draft, the Play Console's App content section asks for a series of declarations. Each one is quick if your design is simple: zero permissions and no accounts turn most of them into one-word answers. Take them one at a time.",
            "Privacy policy: a public URL that says what the app collects, uses, and shares. It must be live before you submit. Open it in a private browser window to check.",
            "Data safety: a form declaring what data the app collects or shares, and why. The answers must match what the code actually does, including every library and SDK you bundle.",
            "Content rating: the IARC questionnaire asks about violence, language, gambling, user interaction, and similar content, and the ratings come back from your answers.",
            "Target audience: the age groups the app is designed for. Including children adds requirements, so decide deliberately.",
            "Ads and app access: whether the app shows ads, and whether a reviewer needs a login to reach the content.",
        ],
        example: &[
            "Privacy policy: qompass.ai/privacy names Light Show by its package name and states that the game collects nothing.",
            "Data safety: no data collected, no data shared. That is the honest answer for a game with no network permission, no accounts, no analytics, and no ads.",
            "Content rating: every questionnaire answer was No, so the expected rating is Everyone.",
            "Target audience: 18+. The game is written for working technicians and students of the trade.",
            "Ads: none. App access: there are no accounts, so there is no login wall. News app: no. Government app: no.",
        ],
        requirements: &[],
        link: Some(("/privacy", "Light Show's privacy policy →")),
        checks: &[
            "Your privacy policy is live at a public URL and names the app.",
            "Your Data safety answers match what the code does, SDKs included.",
            "Content rating, target audience, ads, and app access are all answered.",
        ],
    },
    Step {
        title: "Run the testing tracks",
        practice: &[
            "Play has a ladder of tracks before the public: internal testing (a short list of testers you name), closed testing (invited testers who join through an opt-in link), and production. Promote a build up the ladder rather than uploading a new one for each rung.",
            "On a personal developer account created after November 13, 2023, production stays locked until a closed test has had at least 12 testers opted in continuously for 14 days. Testers drop out, so recruit more than 12, and ask each one to stay the full two weeks: an account that leaves and rejoins restarts its own count.",
            "Make opting in easy: one link, a page that walks every screen, and a private way to send feedback.",
        ],
        example: &[
            "Light Show's first bundle, versionCode 1 (0.1.0), went to internal testing on October 13, 2026. The publishing checklist's next step is to install it through the testing link and play World 1 end to end. versionCode 2 (0.1.1) went to the closed track. The recruiting target is 15 or more testers, so that at least 12 remain through the 14 days.",
            "The testers page walks the real opt-in flow screen by screen, from the invitation link to the private feedback box.",
        ],
        requirements: &[
            "Personal developer accounts created after November 13, 2023: a closed test with at least 12 testers opted in continuously for at least the 14 days before you apply for production access.",
        ],
        link: Some(("/light-show/testers", "The opt-in flow, screen by screen →")),
        checks: &[
            "A build is on the internal track, and you installed it through the testing link.",
            "The closed track has a build and an opt-in link.",
            "At least 12 testers have stayed opted in for 14 days, and you recruited spares.",
        ],
    },
    Step {
        title: "Ship beyond Play",
        practice: &[
            "Play is one channel. F-Droid, the free-software Android store, builds your app from source on its own servers, from a recipe you submit to its data repository. It does not take your binaries, and it signs the result with its own key. That rewards a reproducible build: a pinned toolchain, a committed lockfile, no proprietary SDKs, and nothing fetched at runtime.",
            "Trial builds on your own release page let people play before the stores open. Label each one for exactly what it is.",
        ],
        example: &[
            "Light Show's F-Droid recipe builds a pinned commit with cargo-apk. There, the versionCode is not 1: cargo-apk derives it from the crate version, so 0.1.0 becomes 16777472. That number was read from the built APK with aapt, never typed in, because F-Droid's build fails when the recipe and the APK disagree. The bundle passed lint clean and a deterministic-build check, and the submission is in preparation.",
            "The recipe is pinned to the last commit where quiz answers are plain data. Later release builds check answers against a key that only the Play and desktop builds carry, and a public build farm has no way to keep a key secret.",
            "Trial builds for Windows, Linux, and Android are on the project's GitHub releases page (trial-v0.1.0-2026-10-13). The Windows build is unsigned, so SmartScreen warns on first launch. The Android APK is debug-signed, not release-signed, so its signature will never match the store build: a tester has to uninstall the trial before installing from a store (step 4 is why). Both are labeled as trial builds, not store releases.",
        ],
        requirements: &["F-Droid builds from source on its own infrastructure; it does not accept prebuilt binaries."],
        link: Some(("https://github.com/qompassai/light-show/releases", "Light Show's trial builds →")),
        checks: &[
            "Your build reproduces from a pinned commit with a committed lockfile.",
            "Your dependency tree has no ads, trackers, or proprietary SDKs.",
            "Every trial build is published and labeled for what it is.",
        ],
    },
    Step {
        title: "Ship, listen, and iterate",
        practice: &[
            "Every upload needs a higher versionCode than the last. That integer is what Play compares; the version name is what players see. Bump both, every time.",
            "Write the release notes for each versionCode in the repository before you upload: what changed, in a player's words, in under 500 characters.",
            "Fix in stages. Reproduce the report, fix it on a branch, capture proof on the kind of device that showed the bug, then ship the fix as the next versionCode.",
            "Read tester feedback for the level and the moment. \u{201c}It restarted when I rotated the tablet\u{201d} is a fix. \u{201c}Buggy\u{201d} is a question to ask back.",
        ],
        example: &[
            "Light Show's second build was a large-screen fix. Android 16 (API 36) ignores orientation locks and aspect-ratio limits on large screens (smallest width 600 dp and up), so a game that assumed portrait broke on tablets. Rotating restarted the game, the title backdrop did not refit, and the companion picker spilled off short landscape screens.",
            "The fix had three parts. The orientation lock came out, and the activity was declared resizable. The manifest now lists every configuration change that rotation, resizing, or folding can cause, so Android hands the change to the running game instead of destroying and recreating it mid-level. And every backdrop now refits on resize. Proof came from API 36 tablet emulators at 7 and 10 inches. The fix shipped as versionCode 2, version 0.1.1, with this note: \u{201c}Tablet and large-screen support, done right: Light Show now resizes freely to fit any window, split-screen, or foldable.\u{201d}",
        ],
        requirements: &["Every upload to Play needs a versionCode strictly higher than the last one uploaded."],
        link: None,
        checks: &[
            "The versionCode went up with this upload.",
            "This versionCode has release notes in the repository.",
            "Every tester report has a reply or a tracked issue.",
        ],
    },
];

/// The visitor's walk through the guide: which step is open and which
/// boxes are ticked. Out-of-range indices are ignored everywhere.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Walk {
    open: Option<usize>,
    ticked: Vec<Vec<bool>>,
}

impl Default for Walk {
    fn default() -> Self {
        Self { open: Some(0), ticked: STEPS.iter().map(|step| vec![false; step.checks.len()]).collect() }
    }
}

impl Walk {
    pub fn open(&self) -> Option<usize> {
        self.open
    }

    /// Open a closed step (closing the open one), or close the open one.
    pub fn toggle(&mut self, step: usize) {
        if step >= STEP_COUNT {
            return;
        }
        self.open = if self.open == Some(step) { None } else { Some(step) };
    }

    /// Open exactly `step`.
    pub fn go(&mut self, step: usize) {
        if step < STEP_COUNT {
            self.open = Some(step);
        }
    }

    pub fn ticked(&self, step: usize, item: usize) -> bool {
        self.ticked.get(step).and_then(|items| items.get(item)).copied().unwrap_or(false)
    }

    pub fn set_tick(&mut self, step: usize, item: usize, on: bool) {
        if let Some(slot) = self.ticked.get_mut(step).and_then(|items| items.get_mut(item)) {
            *slot = on;
        }
    }

    /// A step is complete when it has boxes and every one is ticked.
    pub fn step_done(&self, step: usize) -> bool {
        self.ticked.get(step).is_some_and(|items| !items.is_empty() && items.iter().all(|&on| on))
    }

    pub fn done_count(&self) -> usize {
        (0..STEP_COUNT).filter(|&step| self.step_done(step)).count()
    }
}

pub fn tally_line(done: usize) -> String {
    format!("{done} of {STEP_COUNT} steps complete")
}

#[component]
pub fn LightShowHowTo() -> impl IntoView {
    let walk = RwSignal::new(Walk::default());
    let enhanced = RwSignal::new(false);
    Effect::new(move |_| enhanced.set(true));
    let heads: [NodeRef<html::Button>; STEP_COUNT] = std::array::from_fn(|_| NodeRef::new());
    let move_to = move |step: usize| {
        walk.update(|w| w.go(step));
        if let Some(head) = heads.get(step).and_then(NodeRef::get) {
            let _ = head.focus();
        }
    };
    let steps = STEPS
        .iter()
        .enumerate()
        .map(|(i, step)| {
            let is_open = move || !enhanced.get() || walk.with(|w| w.open() == Some(i));
            let done = move || walk.with(|w| w.step_done(i));
            let head = heads.get(i).copied().unwrap_or_default();
            let checks = step
                .checks
                .iter()
                .enumerate()
                .map(|(j, check)| {
                    view! {
                        <label class="howto-check">
                            <input
                                type="checkbox"
                                prop:checked=move || walk.with(|w| w.ticked(i, j))
                                on:change=move |ev| {
                                    let on = event_target_checked(&ev);
                                    walk.update(|w| w.set_tick(i, j, on));
                                }
                            />
                            <span>{*check}</span>
                        </label>
                    }
                })
                .collect_view();
            let requirements = (!step.requirements.is_empty()).then(|| {
                view! {
                    <div class="howto-req">
                        <p class="entry-kind">"HARD REQUIREMENT"</p>
                        <ul class="feature-list">
                            {step.requirements.iter().map(|r| view! { <li>{*r}</li> }).collect_view()}
                        </ul>
                    </div>
                }
            });
            let link = step.link.map(|(href, text)| {
                view! {
                    <p class="study-link">
                        <a href=href>{text}</a>
                    </p>
                }
            });
            let prev = i.checked_sub(1).map(|p| {
                view! {
                    <button type="button" class="howto-move" on:click=move |_| move_to(p)>
                        "← Previous step"
                    </button>
                }
            });
            let next = (i + 1 < STEP_COUNT).then(|| {
                view! {
                    <button type="button" class="howto-move" on:click=move |_| move_to(i + 1)>
                        "Next step →"
                    </button>
                }
            });
            view! {
                <li class=move || if done() { "howto-step is-done" } else { "howto-step" }>
                    <h3 class="howto-head">
                        <button
                            type="button"
                            class="howto-toggle"
                            id=format!("howto-head-{}", i + 1)
                            aria-controls=format!("howto-panel-{}", i + 1)
                            aria-expanded=move || is_open().to_string()
                            node_ref=head
                            on:click=move |_| walk.update(|w| w.toggle(i))
                        >
                            <span class="entry-kind">
                                {format!("STEP {} OF {STEP_COUNT}", i + 1)}
                                <span class="howto-mark" hidden=move || !done()>
                                    " · DONE"
                                </span>
                            </span>
                            <span class="howto-title">{step.title}</span>
                        </button>
                    </h3>
                    <div
                        class="howto-panel"
                        id=format!("howto-panel-{}", i + 1)
                        role="region"
                        aria-labelledby=format!("howto-head-{}", i + 1)
                        hidden=move || !is_open()
                    >
                        {step.practice.iter().map(|p| view! { <p>{*p}</p> }).collect_view()}
                        <div class="howto-example">
                            <p class="entry-kind">"LIGHT SHOW — WORKED EXAMPLE"</p>
                            {step.example.iter().map(|p| view! { <p>{*p}</p> }).collect_view()}
                        </div>
                        {requirements}
                        {link}
                        <fieldset class="howto-checks">
                            <legend>"You have done this when…"</legend>
                            {checks}
                        </fieldset>
                        <div class="howto-nav" hidden=move || !enhanced.get()>
                            {prev}
                            {next}
                        </div>
                    </div>
                </li>
            }
        })
        .collect_view();
    view! {
        <section
            class=move || if enhanced.get() { "study-section howto is-enhanced" } else { "study-section howto" }
            id="how-to"
        >
            <p class="band-label">"BUILD YOUR OWN"</p>
            <h2 class="howto-heading">"How to build an educational game for Android"</h2>
            <p>
                "Light Show went from a design document to a closed test on Google Play. These are "
                "the ten steps it took, in order, written so you can follow them with your own game. "
                "Each step gives the general practice first, then what Light Show actually did, with "
                "its real numbers."
            </p>
            <p>
                "Tick the boxes as you finish each step. The ticks stay in this browser tab: nothing "
                "is saved or sent anywhere, and they reset when you leave the page."
            </p>
            <p class="howto-tally" aria-live="polite" hidden=move || !enhanced.get()>
                {move || tally_line(walk.with(Walk::done_count))}
            </p>
            <ol class="howto-steps">{steps}</ol>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_copy() -> Vec<&'static str> {
        let mut copy = Vec::new();
        for step in &STEPS {
            copy.push(step.title);
            copy.extend_from_slice(step.practice);
            copy.extend_from_slice(step.example);
            copy.extend_from_slice(step.requirements);
            copy.extend_from_slice(step.checks);
            if let Some((href, text)) = step.link {
                copy.push(href);
                copy.push(text);
            }
        }
        copy
    }

    #[test]
    fn ten_steps_each_with_practice_example_and_checks() {
        assert_eq!(STEPS.len(), 10);
        let mut titles: Vec<&str> = STEPS.iter().map(|s| s.title).collect();
        titles.sort_unstable();
        titles.dedup();
        assert_eq!(titles.len(), STEP_COUNT, "step titles must be unique");
        for step in &STEPS {
            assert!(!step.practice.is_empty(), "{}: no practice", step.title);
            assert!(!step.example.is_empty(), "{}: no worked example", step.title);
            assert!(!step.checks.is_empty(), "{}: no checklist", step.title);
            for text in step.practice.iter().chain(step.example).chain(step.requirements).chain(step.checks) {
                assert!(!text.trim().is_empty(), "{}: empty paragraph", step.title);
            }
        }
    }

    #[test]
    fn ticking_every_box_of_a_step_completes_it() {
        let mut walk = Walk::default();
        assert_eq!(walk.done_count(), 0);
        let items = STEPS.get(3).map_or(0, |s| s.checks.len());
        for item in 0..items {
            assert!(!walk.step_done(3));
            walk.set_tick(3, item, true);
        }
        assert!(walk.step_done(3));
        assert_eq!(walk.done_count(), 1);
        assert_eq!(tally_line(walk.done_count()), "1 of 10 steps complete");
    }

    #[test]
    fn ticking_everything_completes_all_ten() {
        let mut walk = Walk::default();
        for (step, s) in STEPS.iter().enumerate() {
            for item in 0..s.checks.len() {
                walk.set_tick(step, item, true);
            }
        }
        assert_eq!(walk.done_count(), STEP_COUNT);
        assert_eq!(tally_line(walk.done_count()), "10 of 10 steps complete");
    }

    #[test]
    fn walk_opens_the_first_step_and_moves_one_at_a_time() {
        let mut walk = Walk::default();
        assert_eq!(walk.open(), Some(0));
        walk.go(4);
        assert_eq!(walk.open(), Some(4));
        walk.toggle(7);
        assert_eq!(walk.open(), Some(7));
        walk.toggle(7);
        assert_eq!(walk.open(), None);
    }

    // — adversarial —

    #[test]
    fn out_of_range_moves_and_ticks_are_ignored() {
        let mut walk = Walk::default();
        let before = walk.clone();
        walk.go(STEP_COUNT);
        walk.go(usize::MAX);
        walk.toggle(STEP_COUNT);
        walk.toggle(usize::MAX);
        walk.set_tick(STEP_COUNT, 0, true);
        walk.set_tick(0, 99, true);
        walk.set_tick(usize::MAX, usize::MAX, true);
        assert_eq!(walk, before);
        assert!(!walk.ticked(usize::MAX, 0));
        assert!(!walk.step_done(STEP_COUNT));
    }

    #[test]
    fn unticking_one_box_reopens_a_done_step() {
        let mut walk = Walk::default();
        let items = STEPS.first().map_or(0, |s| s.checks.len());
        for item in 0..items {
            walk.set_tick(0, item, true);
        }
        assert_eq!(walk.done_count(), 1);
        walk.set_tick(0, 0, false);
        assert!(!walk.step_done(0));
        assert_eq!(walk.done_count(), 0);
        walk.set_tick(0, 0, true);
        walk.set_tick(0, 0, true);
        assert_eq!(walk.done_count(), 1, "re-ticking is idempotent");
    }

    #[test]
    fn partial_ticks_across_steps_never_count() {
        let mut walk = Walk::default();
        for step in 0..STEP_COUNT {
            walk.set_tick(step, 0, true);
        }
        assert_eq!(walk.done_count(), 0);
    }

    #[test]
    fn copy_carries_no_secret_smell() {
        // The guide teaches keystore discipline; it must never carry a
        // value that discipline protects. Case-insensitive.
        let smells = [
            "begin ",
            "private key",
            "storepass",
            "keypass",
            "password=",
            "password:",
            "passwd",
            ".jks",
            ".keystore",
            ".json",
            "gserviceaccount",
            "pass qompassai",
            "api_key",
            "apikey",
            "secret=",
            "token=",
            "akia",
            "aiza",
            "ghp_",
            "sk-ant",
            "-----",
            "/home/",
            "~/",
            ".config/",
            "sops",
            "age1",
        ];
        for text in all_copy() {
            let lower = text.to_lowercase();
            for smell in smells {
                assert!(!lower.contains(smell), "secret smell {smell:?} in: {text}");
            }
        }
    }
}
