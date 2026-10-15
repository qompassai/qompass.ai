//! Telco-admin practice quizzes at `/solutions/telco-admin-quizzes`.
//!
//! An Education solution with its own page (until 2026-10-15 they sat
//! on `/field-notes`): Matt's twelve study sets for the Washington
//! Telecommunications (09) administrator exam — NEC, RCW 19.28,
//! WAC 296-46B, and general theory — ported whole.
//!
//! Questions, choices, answers, and explanations are the source text,
//! byte for byte (see `content/field-notes/quizzes.toml`); nothing here
//! rewords them. Where a set carries code sections, index lookup paths,
//! or a note per answer choice, the feedback shows them too.
//!
//! Once the page hydrates, the picker and the one-question-at-a-time
//! runner appear: instant feedback, a running score, and a restart. The
//! run logic is [`Run`], plain Rust, so its rules are unit-tested.
//!
//! The answer keys are not in the page. Each set's key is an encrypted
//! envelope at `/answer-keys/<slug>.json` (written by
//! `scripts/generate-answer-keys.mjs`, one password per set) that
//! `/js/answer-keys.js` opens in the browser. The server renders a
//! panel per set; the panel's lock is `inner_html` because that script
//! owns it, and hydration must not walk nodes a page script owns.
use crate::field_notes_data::{question_count, Quiz, QuizQuestion, QUIZZES};
use crate::pages::compass_services::{solution_page, CARDINALS};
use leptos::prelude::*;
use leptos_meta::Title;

/// The page's slug under `/solutions/`.
pub const SLUG: &str = "telco-admin-quizzes";

const LETTERS: [&str; 4] = ["A", "B", "C", "D"];

fn letter(i: usize) -> &'static str {
    LETTERS.get(i).copied().unwrap_or("?")
}

/// One pass through one set, in source order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    pub quiz: usize,
    pub index: usize,
    pub picked: Option<usize>,
    pub correct: usize,
    pub answered: usize,
    /// Source question numbers answered wrong this pass.
    pub missed: Vec<u32>,
}

impl Run {
    pub const fn new(quiz: usize) -> Self {
        Self { quiz, index: 0, picked: None, correct: 0, answered: 0, missed: Vec::new() }
    }

    fn quiz(&self) -> Option<&'static Quiz> {
        QUIZZES.get(self.quiz)
    }

    pub fn question(&self) -> Option<&'static QuizQuestion> {
        self.quiz().and_then(|q| q.questions.get(self.index))
    }

    /// Record an answer; a second answer to the same question, or a
    /// choice that doesn't exist, is ignored.
    pub fn answer(&mut self, choice: usize) {
        let Some(q) = self.question() else { return };
        if self.picked.is_some() || choice >= q.choices.len() {
            return;
        }
        self.picked = Some(choice);
        self.answered += 1;
        if choice == q.answer {
            self.correct += 1;
        } else {
            self.missed.push(q.number);
        }
    }

    /// Advance past an answered question.
    pub fn next(&mut self) {
        if self.picked.is_some() && !self.is_last() {
            self.index += 1;
            self.picked = None;
        }
    }

    pub fn is_last(&self) -> bool {
        self.quiz().is_none_or(|q| self.index + 1 >= q.questions.len())
    }

    pub fn finished(&self) -> bool {
        self.picked.is_some() && self.is_last()
    }
}

type RunSignal = RwSignal<Option<Run>>;

fn references(q: &'static QuizQuestion) -> impl IntoView {
    view! {
        {(!q.sections.is_empty())
            .then(|| view! { <p class="fn-q-ref">{format!("Sections: {}", q.sections.join(", "))}</p> })}
        {(!q.index_paths.is_empty())
            .then(|| view! { <p class="fn-q-ref">{format!("Index lookup: {}", q.index_paths.join(" · "))}</p> })}
    }
}

fn question_view(run: RunSignal, state: &Run, quiz: &'static Quiz, q: &'static QuizQuestion) -> AnyView {
    let picked = state.picked;
    let choices = q
        .choices
        .iter()
        .enumerate()
        .map(|(i, &text)| {
            let class = match picked {
                Some(_) if i == q.answer => "fn-choice is-right",
                Some(p) if p == i => "fn-choice is-wrong",
                _ => "fn-choice",
            };
            view! {
                <button
                    type="button"
                    class=class
                    disabled=picked.is_some()
                    on:click=move |_| {
                        run.update(|r| {
                            if let Some(r) = r {
                                r.answer(i);
                            }
                        });
                    }
                >
                    <span class="fn-choice-letter">{letter(i)}</span>
                    <span>{text}</span>
                </button>
            }
        })
        .collect_view();
    let feedback = picked.map(|choice| {
        let answer_text = q.choices.get(q.answer).copied().unwrap_or_default();
        let (class, line) = if choice == q.answer {
            ("ft-feedback is-right", "Correct.".to_string())
        } else {
            ("ft-feedback is-wrong", format!("Not quite — the answer is {}: {answer_text}", letter(q.answer)))
        };
        let note = (choice != q.answer).then(|| q.choice_notes.get(choice)).flatten();
        view! {
            <p class=class>{line}</p>
            {(!q.explanation.is_empty()).then(|| view! { <p class="fn-q-expl">{q.explanation}</p> })}
            {references(q)}
            {note.map(|n| view! { <p class="fn-q-note">{format!("About your pick: {n}")}</p> })}
        }
    });
    let after = if state.finished() {
        let missed = if state.missed.is_empty() {
            "Nothing missed.".to_string()
        } else {
            let numbers: Vec<String> = state.missed.iter().map(u32::to_string).collect();
            format!("Worth another look: question {}.", numbers.join(", "))
        };
        let quiz_index = state.quiz;
        view! {
            <div class="ft-summary">
                <p class="ft-score">{format!("Set complete — {} of {}.", state.correct, quiz.questions.len())}</p>
                <p class="ft-sub">{missed}</p>
                <button type="button" class="ft-next" on:click=move |_| run.set(Some(Run::new(quiz_index)))>
                    "Restart this set"
                </button>
            </div>
        }
        .into_any()
    } else if picked.is_some() {
        view! {
            <button
                type="button"
                class="ft-next"
                on:click=move |_| {
                    run.update(|r| {
                        if let Some(r) = r {
                            r.next();
                        }
                    });
                }
            >
                "Next question →"
            </button>
        }
        .into_any()
    } else {
        ().into_any()
    };
    view! {
        <p class="entry-kind">
            {format!(
                "QUESTION {} OF {} · SCORE {} / {}",
                state.index + 1,
                quiz.questions.len(),
                state.correct,
                state.answered,
            )}
        </p>
        <p class="fn-q-set">{quiz.title}</p>
        <p class="fn-q-prompt">{q.prompt}</p>
        <div class="fn-choices">{choices}</div>
        {feedback}
        {after}
    }
    .into_any()
}

fn stage_view(run: RunSignal, state: Option<&Run>) -> AnyView {
    let current = state.and_then(|r| Some((r, QUIZZES.get(r.quiz)?, r.question()?)));
    current.map_or_else(
        || {
            view! {
                <p class="ft-idle">
                    "Pick a set to start. Questions come one at a time with instant feedback; "
                    "it all runs in your browser and nothing is sent anywhere."
                </p>
            }
            .into_any()
        },
        |(r, quiz, q)| question_view(run, r, quiz, q),
    )
}

/// The lock of one answer-key panel, owned by `/js/answer-keys.js`.
///
/// Without JavaScript only the note shows; the script unhides the form.
/// `slug` is a build-checked set id (`[a-z0-9-]`, see the tests), so it
/// needs no escaping in the attribute.
fn lock_html(slug: &str) -> String {
    format!(
        concat!(
            "<p class=\"ak-note\">This answer key is encrypted; it opens with this set's password.</p>",
            "<form class=\"ak-form\" data-key-slug=\"{}\" hidden>",
            "<input class=\"ak-input\" type=\"password\" aria-label=\"Password for this answer key\" ",
            "autocomplete=\"off\" required>",
            "<button type=\"submit\" class=\"ft-next\">Unlock</button>",
            "</form>",
            "<p class=\"ak-error\" role=\"alert\" hidden></p>",
            "<div class=\"ak-body\"></div>",
            "<button type=\"button\" class=\"ft-next ak-hide\" hidden>Hide</button>",
        ),
        slug
    )
}

/// One set's answer-key panel: title, question count, and the lock.
fn answer_key(quiz: &'static Quiz) -> impl IntoView {
    view! {
        <div class="ak-panel" id=format!("key-{}", quiz.slug)>
            <h3 class="ak-title">{quiz.title}</h3>
            <p class="ak-count">{format!("{} questions", quiz.questions.len())}</p>
            <div class="ak-lock" inner_html=lock_html(quiz.slug)></div>
        </div>
    }
}

#[component]
pub fn TelcoQuizzes() -> impl IntoView {
    let run: RunSignal = RwSignal::new(None);
    let enhanced = RwSignal::new(false);
    Effect::new(move |_| enhanced.set(true));
    let solution = solution_page(SLUG);
    let name = solution.map_or("Telco-Admin Practice Quizzes", |s| s.name);
    view! {
        <Title text=format!("{name} - Qompass AI") />
        <section class="page project-page">
            <article class="project-article">
                <header class="study-head">
                    <p class="page-bearing">{solution.and_then(|s| CARDINALS.get(s.cardinal)).map(|c| c.mark)}</p>
                    <h1>{name}</h1>
                </header>
                {quiz_section(run, enhanced)}
                <section class="study-section fn-quizzes" id="answer-keys">
                    <h2 class="band-label">"ANSWER KEYS"</h2>
                    <p>
                        "Each set's answer key is encrypted, with its own password. Unlocking happens "
                        "in your browser; the password and the key are never stored or sent anywhere."
                    </p>
                    {QUIZZES.iter().map(answer_key).collect_view()}
                </section>
            </article>
            <script src="/js/answer-keys.js" defer></script>
        </section>
    }
}

/// The intro, the picker, and the runner.
fn quiz_section(run: RunSignal, enhanced: RwSignal<bool>) -> impl IntoView {
    view! {
        <section class="study-section fn-quizzes" id="quizzes">
            <h2 class="band-label">"PRACTICE SETS"</h2>
            <p>
                {format!(
                    concat!(
                        "The {} practice sets I built studying for the Washington Telecommunications (09) ",
                        "administrator exam — {} questions on the NEC, RCW 19.28, WAC 296-46B, and ",
                        "general trade theory, each answer with its explanation. They are study aids, not ",
                        "the exam; the code books and the law are the authority.",
                    ),
                    QUIZZES.len(),
                    question_count(),
                )}
            </p>
            <p class="fn-noscript-note" hidden=move || enhanced.get()>
                "The interactive quiz and the answer keys run with JavaScript."
            </p>
            <div class="fn-quiz-live" hidden=move || !enhanced.get()>
                <div class="fn-quiz-picker" role="group" aria-label="Choose a practice set">
                    {QUIZZES
                        .iter()
                        .enumerate()
                        .map(|(i, quiz)| {
                            let active = move || run.with(|r| r.as_ref().map(|r| r.quiz)) == Some(i);
                            view! {
                                <button
                                    type="button"
                                    class=move || if active() { "ft-mode is-active" } else { "ft-mode" }
                                    aria-pressed=move || active().to_string()
                                    on:click=move |_| run.set(Some(Run::new(i)))
                                >
                                    <span class="ft-mode-name">{quiz.title}</span>
                                    <span class="ft-mode-hint">
                                        {format!("{} questions", quiz.questions.len())}
                                    </span>
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
                <div class="ft-stage" aria-live="polite">
                    {move || run.with(|r| stage_view(run, r.as_ref()))}
                </div>
            </div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::{solution_page, Run, CARDINALS, QUIZZES, SLUG};
    use crate::pages::compass_services::Solution;

    #[test]
    fn right_answer_scores_and_wrong_answer_is_remembered() {
        let mut run = Run::new(0);
        let Some(answer) = run.question().map(|q| q.answer) else { return };
        run.answer(answer);
        assert_eq!((run.correct, run.answered), (1, 1));
        assert_eq!(run.missed, Vec::<u32>::new());
        run.next();
        let second = run.question().map(|q| ((q.answer + 1) % q.choices.len(), q.number));
        let Some((wrong, number)) = second else { return };
        run.answer(wrong);
        assert_eq!((run.correct, run.answered), (1, 2));
        assert_eq!(run.missed, vec![number]);
    }

    #[test]
    fn double_answer_out_of_range_and_unanswered_next_are_ignored() {
        let mut run = Run::new(0);
        run.next();
        assert_eq!(run.index, 0, "next before answering must not advance");
        run.answer(99);
        assert_eq!(run.picked, None);
        run.answer(0);
        run.answer(1);
        assert_eq!((run.picked, run.answered), (Some(0), 1));
    }

    #[test]
    fn a_full_pass_ends_on_the_last_question_of_every_set() {
        for (i, quiz) in QUIZZES.iter().enumerate() {
            let mut run = Run::new(i);
            let mut steps = 0;
            while !run.finished() {
                let Some(answer) = run.question().map(|q| q.answer) else { break };
                run.answer(answer);
                run.next();
                steps += 1;
                assert!(steps <= quiz.questions.len(), "{} never finishes", quiz.slug);
            }
            assert_eq!(run.correct, quiz.questions.len(), "{}", quiz.slug);
            run.next();
            assert_eq!(run.index + 1, quiz.questions.len(), "next past the end must not move");
        }
    }

    #[test]
    fn every_set_has_exactly_one_sealed_envelope_and_a_plain_slug() {
        let dir = format!("{}/public/answer-keys", env!("CARGO_MANIFEST_DIR"));
        let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
            .map(|d| d.flatten().filter_map(|e| e.file_name().into_string().ok()).collect())
            .unwrap_or_default();
        on_disk.sort();
        let mut want: Vec<String> = QUIZZES.iter().map(|q| format!("{}.json", q.slug)).collect();
        want.sort();
        assert_eq!(on_disk, want);
        for quiz in QUIZZES {
            assert!(quiz.slug.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'));
            let body = std::fs::read_to_string(format!("{dir}/{}.json", quiz.slug)).unwrap_or_default();
            assert!(body.starts_with(r#"{"v":1,"kdf":"PBKDF2-SHA256","iter":310000,"salt":""#), "{}", quiz.slug);
            // Sealed, not plaintext: no explanation survives into the envelope.
            for q in quiz.questions.iter().filter(|q| q.explanation.len() > 20) {
                assert!(!body.contains(q.explanation), "{} {}", quiz.slug, q.number);
            }
        }
    }

    #[test]
    fn the_page_is_an_education_solution_without_a_version() {
        let page = solution_page(SLUG);
        assert_eq!(page.map(|s| (s.name, s.cardinal, s.version)), Some(("Telco-Admin Practice Quizzes", 1, None)));
        assert_eq!(page.and_then(|s| CARDINALS.get(s.cardinal)).map(|c| c.name), Some("Education"));
        assert_eq!(page.and_then(Solution::version_href), None);
    }

    #[test]
    fn a_missing_set_is_inert() {
        let mut run = Run::new(QUIZZES.len());
        run.answer(0);
        assert_eq!(run.answered, 0);
        assert!(run.is_last());
    }
}
