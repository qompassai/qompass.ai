//! Telco-admin practice quizzes on `/field-notes`: Matt's twelve study
//! sets for the Washington Telecommunications (09) administrator exam
//! — NEC, RCW 19.28, WAC 296-46B, and general theory — ported whole.
//!
//! Questions, choices, answers, and explanations are the source text,
//! byte for byte (see `content/field-notes/quizzes.toml`); nothing here
//! rewords them. Where a set carries code sections, index lookup paths,
//! or a note per answer choice, the feedback shows them too.
//!
//! Two renderings, both honest. The server renders every set as a
//! static answer key — each question readable, its answer behind a
//! disclosure — so the whole bank works without JavaScript. Once the
//! page hydrates, the picker and the one-question-at-a-time runner
//! appear: instant feedback, a running score, and a restart. The run
//! logic is [`Run`], plain Rust, so its rules are unit-tested.
use crate::field_notes_data::{question_count, Quiz, QuizQuestion, QUIZZES};
use leptos::prelude::*;

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

/// The static answer key for one set: every question, its choices, and
/// the answer behind a disclosure.
fn answer_key(quiz: &'static Quiz) -> impl IntoView {
    view! {
        <details class="fn-quiz-key" id=format!("key-{}", quiz.slug)>
            <summary>{format!("{} — {} questions, answer key", quiz.title, quiz.questions.len())}</summary>
            {(!quiz.description.is_empty()).then(|| view! { <p class="fn-key-desc">{quiz.description}</p> })}
            <ol class="fn-key-list">
                {quiz
                    .questions
                    .iter()
                    .map(|q| {
                        view! {
                            <li class="fn-key-q" value=q.number>
                                <p class="fn-key-prompt">{q.prompt}</p>
                                <ol class="fn-key-choices" type="A">
                                    {q.choices.iter().map(|&c| view! { <li>{c}</li> }).collect_view()}
                                </ol>
                                <details class="fn-key-answer">
                                    <summary>"Answer"</summary>
                                    <p>
                                        <strong>
                                            {format!(
                                                "{}. {}",
                                                letter(q.answer),
                                                q.choices.get(q.answer).copied().unwrap_or_default(),
                                            )}
                                        </strong>
                                    </p>
                                    {(!q.explanation.is_empty()).then(|| view! { <p>{q.explanation}</p> })}
                                    {references(q)}
                                </details>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </details>
    }
}

#[component]
pub fn TelcoQuizzes() -> impl IntoView {
    let run: RunSignal = RwSignal::new(None);
    let enhanced = RwSignal::new(false);
    Effect::new(move |_| enhanced.set(true));
    view! {
        <section class="study-section fn-quizzes" id="quizzes">
            <h2 class="band-label">"TELCO-ADMIN PRACTICE QUIZZES"</h2>
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
                "The interactive quiz runs with JavaScript. Every question and its answer are in "
                "the answer keys below either way."
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
            <h3 class="study-subhead">"Answer keys"</h3>
            {QUIZZES.iter().map(answer_key).collect_view()}
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::{Run, QUIZZES};

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
    fn a_missing_set_is_inert() {
        let mut run = Run::new(QUIZZES.len());
        run.answer(0);
        assert_eq!(run.answered, 0);
        assert!(run.is_last());
    }
}
