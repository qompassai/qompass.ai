//! The TIA-598 color code trainer, `/field-notes/trainer`.
//!
//! A hydrated Leptos island in the same model as the home page's
//! quick-start picker: the server renders the idle trainer (no round
//! in progress), and once the WASM bundle hydrates, every tap is a
//! signal update — nothing leaves the browser.
//!
//! The drill logic lives in [`Drill`], plain Rust with no Leptos in
//! it, so the rules are unit-tested without a browser. The view only
//! reads a `Drill` and forwards taps to it. Randomness is drawn only
//! when a round starts (a click, so always after hydration), which
//! keeps the server render and the hydrated render identical.
use crate::pages::field_service::flow_block;
use leptos::prelude::*;
use leptos_meta::Title;

/// The twelve TIA-598 position colors, in sequence order, with the
/// CSS class suffix that paints each swatch.
pub const FIBERS: [(&str, &str); 12] = [
    ("Blue", "blue"),
    ("Orange", "orange"),
    ("Green", "green"),
    ("Brown", "brown"),
    ("Slate", "slate"),
    ("White", "white"),
    ("Red", "red"),
    ("Black", "black"),
    ("Yellow", "yellow"),
    ("Violet", "violet"),
    ("Rose", "rose"),
    ("Aqua", "aqua"),
];

/// Questions per round in the two quiz drills.
pub const ROUND: u32 = 10;

/// Color name for a zero-based position.
pub fn fiber_name(position: usize) -> &'static str {
    FIBERS.get(position).map_or("?", |fiber| fiber.0)
}

/// Swatch class for a zero-based position.
pub fn swatch_class(position: usize) -> String {
    format!("ft-swatch fiber-{}", FIBERS.get(position).map_or("unknown", |fiber| fiber.1))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    ColorToPosition,
    PositionToColor,
    Sequence,
}

/// What a quiz question shows. `target` on [`Question`] is always the
/// zero-based position whose color (or number) is the answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ask {
    /// Show a color, answer its position number.
    Color,
    /// Name a fiber position inside a tube, answer the fiber's color.
    Fiber { tube: usize },
    /// Name a tube position, answer the tube's color.
    Tube,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Question {
    pub ask: Ask,
    pub target: usize,
}

/// One round of one drill.
#[derive(Clone, Debug)]
pub struct Drill {
    pub mode: Mode,
    rng: u64,
    /// The open quiz question; `None` in sequence mode.
    pub question: Option<Question>,
    /// Quiz questions answered this round.
    pub asked: u32,
    pub correct: u32,
    /// The answer given to the open question, once given.
    pub picked: Option<usize>,
    /// Questions answered wrong this round, for the review list.
    pub missed: Vec<Question>,
    /// Button order for color answers and the sequence grid, shuffled
    /// per round so a position on screen never gives the answer away.
    pub layout: [usize; 12],
    /// Sequence mode: positions placed so far (the next expected one).
    pub placed: usize,
    pub misses: u32,
    pub last_miss: Option<usize>,
}

/// One splitmix64 step: advances `state` and returns a mixed value.
fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A value in `0..n` (`n` is at most 12 here, so modulo bias is
/// negligible for a practice drill).
fn below(state: &mut u64, n: usize) -> usize {
    debug_assert!(n > 0 && n <= FIBERS.len());
    let n64 = u64::try_from(n).unwrap_or(1);
    usize::try_from(splitmix(state) % n64).unwrap_or(0)
}

impl Drill {
    pub fn new(mode: Mode, seed: u64) -> Self {
        let mut drill = Self {
            mode,
            rng: seed,
            question: None,
            asked: 0,
            correct: 0,
            picked: None,
            missed: Vec::new(),
            layout: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            placed: 0,
            misses: 0,
            last_miss: None,
        };
        for i in (1..drill.layout.len()).rev() {
            let j = below(&mut drill.rng, i + 1);
            drill.layout.swap(i, j);
        }
        if mode != Mode::Sequence {
            drill.question = Some(drill.draw(None));
        }
        drill
    }

    /// A fresh question whose target differs from `previous`.
    fn draw(&mut self, previous: Option<usize>) -> Question {
        let mut target = below(&mut self.rng, FIBERS.len());
        if Some(target) == previous {
            target = (target + 1 + below(&mut self.rng, FIBERS.len() - 1)) % FIBERS.len();
        }
        let ask = match self.mode {
            Mode::ColorToPosition | Mode::Sequence => Ask::Color,
            Mode::PositionToColor if below(&mut self.rng, 3) == 0 => Ask::Tube,
            Mode::PositionToColor => Ask::Fiber { tube: below(&mut self.rng, FIBERS.len()) },
        };
        Question { ask, target }
    }

    /// Answer the open quiz question. A second answer, an answer out
    /// of range, or an answer in sequence mode is ignored.
    pub fn answer(&mut self, choice: usize) {
        let Some(question) = self.question else {
            return;
        };
        if self.picked.is_some() || choice >= FIBERS.len() {
            return;
        }
        self.picked = Some(choice);
        self.asked += 1;
        if choice == question.target {
            self.correct += 1;
        } else {
            self.missed.push(question);
        }
    }

    /// Move to the next question, once the open one is answered and
    /// the round still has questions left.
    pub fn next(&mut self) {
        if self.picked.is_none() || self.round_over() {
            return;
        }
        let previous = self.question.map(|q| q.target);
        self.picked = None;
        self.question = Some(self.draw(previous));
    }

    /// Sequence mode: tap a color. The expected one advances the
    /// sequence; a later one counts a miss; an already-placed one (its
    /// button is disabled anyway) is ignored.
    pub fn tap(&mut self, color: usize) {
        if self.mode != Mode::Sequence || self.round_over() || color >= FIBERS.len() {
            return;
        }
        if color == self.placed {
            self.placed += 1;
            self.last_miss = None;
        } else if color > self.placed {
            self.misses += 1;
            self.last_miss = Some(color);
        }
    }

    pub fn round_over(&self) -> bool {
        match self.mode {
            Mode::Sequence => self.placed == FIBERS.len(),
            Mode::ColorToPosition | Mode::PositionToColor => self.asked >= ROUND && self.picked.is_some(),
        }
    }
}

/// The one-line fact a question teaches, shown after it is answered
/// and again in the end-of-round review.
pub fn fact(question: Question) -> String {
    let number = question.target + 1;
    let color = fiber_name(question.target);
    match question.ask {
        Ask::Color => format!("{color} is position {number}."),
        Ask::Fiber { .. } => format!("Fiber {number} is {color} — the same in every tube."),
        Ask::Tube => format!("Tube {number} is {color}."),
    }
}

/// How a given answer reads back: a number for color questions, a
/// color name otherwise.
fn answer_label(ask: Ask, choice: usize) -> String {
    match ask {
        Ask::Color => format!("position {}", choice + 1),
        Ask::Fiber { .. } | Ask::Tube => fiber_name(choice).to_string(),
    }
}

#[cfg(feature = "hydrate")]
fn fresh_seed() -> u64 {
    js_sys::Math::random().to_bits() ^ js_sys::Date::now().to_bits().rotate_left(17)
}

#[cfg(not(feature = "hydrate"))]
fn fresh_seed() -> u64 {
    0x5EED_F1BE
}

const MODES: [(Mode, &str, &str); 3] = [
    (Mode::ColorToPosition, "Color → position", "See a color, tap its number."),
    (Mode::PositionToColor, "Position → color", "A fiber or tube number, tap its color."),
    (Mode::Sequence, "Sequence order", "Tap all twelve, in order."),
];

type DrillSignal = RwSignal<Option<Drill>>;

fn restart(drill: DrillSignal, mode: Mode) {
    drill.set(Some(Drill::new(mode, fresh_seed())));
}

fn swatch(position: usize) -> impl IntoView {
    view! { <span class=swatch_class(position) aria-hidden="true"></span> }
}

fn new_round_button(drill: DrillSignal, mode: Mode) -> impl IntoView {
    view! {
        <button type="button" class="ft-next" on:click=move |_| restart(drill, mode)>
            "New round"
        </button>
    }
}

fn quiz_view(drill: DrillSignal, state: &Drill, question: Question) -> AnyView {
    let picked = state.picked;
    let target = question.target;
    let number = state.asked + u32::from(picked.is_none());
    let prompt = match question.ask {
        Ask::Color => view! {
            <p class="ft-prompt">{swatch(target)} {fiber_name(target)}</p>
            <p class="ft-sub">"Tap its position number."</p>
        }
        .into_any(),
        Ask::Fiber { tube } => view! {
            <p class="ft-prompt">
                {format!("Fiber {} in the ", target + 1)} {swatch(tube)}
                {format!("{} tube", fiber_name(tube))}
            </p>
            <p class="ft-sub">"Tap the fiber's color."</p>
        }
        .into_any(),
        Ask::Tube => view! {
            <p class="ft-prompt">{format!("Tube {} of the cable", target + 1)}</p>
            <p class="ft-sub">"Tap the tube's color."</p>
        }
        .into_any(),
    };
    let order: Vec<usize> =
        if question.ask == Ask::Color { (0..FIBERS.len()).collect() } else { state.layout.to_vec() };
    let numbers = question.ask == Ask::Color;
    let grid_class = if numbers { "ft-answers ft-answers-numbers" } else { "ft-answers" };
    let locked = picked.is_some();
    let answers = order
        .into_iter()
        .map(|choice| {
            let class = match picked {
                Some(_) if choice == target => "ft-answer is-right",
                Some(p) if p == choice => "ft-answer is-wrong",
                _ => "ft-answer",
            };
            let label = if numbers {
                view! { <span class="ft-number">{choice + 1}</span> }.into_any()
            } else {
                view! {
                    {swatch(choice)}
                    <span>{fiber_name(choice)}</span>
                }
                .into_any()
            };
            view! {
                <button
                    type="button"
                    class=class
                    disabled=locked
                    on:click=move |_| {
                        drill
                            .update(|s| {
                                if let Some(d) = s {
                                    d.answer(choice);
                                }
                            });
                    }
                >
                    {label}
                </button>
            }
        })
        .collect_view();
    let feedback = picked.map(|choice| {
        let line = if choice == target {
            format!("Right. {}", fact(question))
        } else {
            format!("Not quite — you picked {}. {}", answer_label(question.ask, choice), fact(question))
        };
        let class = if choice == target { "ft-feedback is-right" } else { "ft-feedback is-wrong" };
        view! { <p class=class>{line}</p> }
    });
    let after = if state.round_over() {
        let review = state.missed.iter().map(|q| view! { <li>{fact(*q)}</li> }).collect_view();
        view! {
            <div class="ft-summary">
                <p class="ft-score">{format!("Round complete — {} of {ROUND}.", state.correct)}</p>
                {(!state.missed.is_empty())
                    .then(|| {
                        view! {
                            <p class="ft-sub">"Worth another look:"</p>
                            <ul class="feature-list">{review}</ul>
                        }
                    })}
                {new_round_button(drill, state.mode)}
            </div>
        }
        .into_any()
    } else if picked.is_some() {
        view! {
            <button
                type="button"
                class="ft-next"
                on:click=move |_| {
                    drill
                        .update(|s| {
                            if let Some(d) = s {
                                d.next();
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
            {format!("QUESTION {number} OF {ROUND} · SCORE {} / {}", state.correct, state.asked)}
        </p>
        {prompt}
        <div class=grid_class>{answers}</div>
        {feedback}
        {after}
    }
    .into_any()
}

fn sequence_view(drill: DrillSignal, state: &Drill) -> AnyView {
    let placed = state.placed;
    let last_miss = state.last_miss;
    let built = (0..placed)
        .map(|position| {
            view! {
                <li class="ft-built-item">
                    {swatch(position)}
                    <span>{format!("{} {}", position + 1, fiber_name(position))}</span>
                </li>
            }
        })
        .collect_view();
    let grid = state
        .layout
        .iter()
        .map(|&color| {
            let done = color < placed;
            let class = if done {
                "ft-answer is-placed"
            } else if last_miss == Some(color) {
                "ft-answer is-wrong"
            } else {
                "ft-answer"
            };
            view! {
                <button
                    type="button"
                    class=class
                    disabled=done
                    on:click=move |_| {
                        drill
                            .update(|s| {
                                if let Some(d) = s {
                                    d.tap(color);
                                }
                            });
                    }
                >
                    {swatch(color)}
                    <span>{fiber_name(color)}</span>
                </button>
            }
        })
        .collect_view();
    let status = if state.round_over() {
        let misses = match state.misses {
            0 => "no misses".to_string(),
            1 => "1 miss".to_string(),
            n => format!("{n} misses"),
        };
        view! {
            <div class="ft-summary">
                <p class="ft-score">{format!("Sequence complete — all twelve, with {misses}.")}</p>
                {new_round_button(drill, state.mode)}
            </div>
        }
        .into_any()
    } else {
        let line = last_miss.map(|m| {
            view! {
                <p class="ft-feedback is-wrong">
                    {format!(
                        "Not yet — {} is position {}. Find position {} first.",
                        fiber_name(m),
                        m + 1,
                        placed + 1,
                    )}
                </p>
            }
        });
        view! {
            <p class="ft-sub">{format!("Tap position {} next.", placed + 1)}</p>
            {line}
        }
        .into_any()
    };
    view! {
        <p class="entry-kind">
            {format!("PLACED {placed} OF {} · MISSES {}", FIBERS.len(), state.misses)}
        </p>
        <p class="ft-prompt">"Tap the twelve colors in order, Blue first."</p>
        <ol class="ft-built" aria-label="Placed so far">{built}</ol>
        <div class="ft-answers">{grid}</div>
        {status}
    }
    .into_any()
}

/// The stage under the drill picker: idle before the first tap, then
/// the open round.
fn stage_view(drill: DrillSignal, state: Option<&Drill>) -> AnyView {
    state.map_or_else(
        || {
            view! {
                <p class="ft-idle">
                    "Pick a drill to start a round. It all runs here in your "
                    "browser — no account, nothing sent anywhere."
                </p>
            }
            .into_any()
        },
        |d| d.question.map_or_else(|| sequence_view(drill, d), |q| quiz_view(drill, d, q)),
    )
}

#[component]
pub fn FiberTrainer() -> impl IntoView {
    let drill: DrillSignal = RwSignal::new(None);
    view! {
        <Title text="TIA-598 Color Code Trainer - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"FIELD NOTES — TRAINER"</p>
            <article class="project-article">
                <header class="study-head">
                    <h1>"TIA-598 color code trainer"</h1>
                    <p class="page-lede">
                        "Twelve colors, drilled until your hands know them before your head does."
                    </p>
                </header>
                <section class="study-section">
                    <p class="band-label">"PICK A DRILL"</p>
                    <div class="ft-modes" role="group" aria-label="Choose a drill">
                        {MODES
                            .iter()
                            .map(|&(mode, name, hint)| {
                                let active = move || {
                                    drill.with(|s| s.as_ref().map(|d| d.mode)) == Some(mode)
                                };
                                view! {
                                    <button
                                        type="button"
                                        class=move || {
                                            if active() { "ft-mode is-active" } else { "ft-mode" }
                                        }
                                        aria-pressed=move || active().to_string()
                                        on:click=move |_| restart(drill, mode)
                                    >
                                        <span class="ft-mode-name">{name}</span>
                                        <span class="ft-mode-hint">{hint}</span>
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                    <div class="ft-stage" aria-live="polite">
                        {move || drill.with(|s| stage_view(drill, s.as_ref()))}
                    </div>
                </section>
                <section class="study-section">
                    <p class="band-label">"THE SEQUENCE"</p>
                    <details class="ft-reference">
                        <summary>"Show all twelve (no peeking mid-round)"</summary>
                        <ol class="ft-built">
                            {(0..FIBERS.len())
                                .map(|position| {
                                    view! {
                                        <li class="ft-built-item">
                                            {swatch(position)}
                                            <span>
                                                {format!("{} {}", position + 1, fiber_name(position))}
                                            </span>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ol>
                    </details>
                    <p>
                        "The why behind the colors — units, tracers, high-count cables, and the "
                        "splice tray — is in the "
                        <a href="/field-notes/notes/tia-598-color-code">"TIA-598 field note"</a> "."
                    </p>
                    <p class="study-link">
                        <a href="/field-notes">"← Field Notes"</a>
                    </p>
                </section>
                {flow_block()}
            </article>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEEDS: [u64; 6] = [0, 1, 42, 0x5EED_F1BE, u64::MAX - 1, u64::MAX];

    #[test]
    fn sequence_is_the_tia_598_order() {
        let names: Vec<&str> = FIBERS.iter().map(|f| f.0).collect();
        assert_eq!(
            names,
            ["Blue", "Orange", "Green", "Brown", "Slate", "White", "Red", "Black", "Yellow", "Violet", "Rose", "Aqua"]
        );
        for (name, slug) in FIBERS {
            assert_eq!(name.to_ascii_lowercase(), slug);
        }
    }

    #[test]
    fn right_answer_scores_and_wrong_answer_is_reviewed() {
        let mut d = Drill::new(Mode::ColorToPosition, 7);
        let q = d.question.unwrap_or(Question { ask: Ask::Tube, target: 99 });
        d.answer(q.target);
        assert_eq!((d.asked, d.correct), (1, 1));
        d.next();
        let q2 = d.question.unwrap_or(Question { ask: Ask::Tube, target: 99 });
        d.answer((q2.target + 1) % 12);
        assert_eq!((d.asked, d.correct), (2, 1));
        assert_eq!(d.missed, vec![q2]);
    }

    #[test]
    fn layout_is_a_permutation_for_every_seed() {
        for seed in SEEDS {
            let mut layout = Drill::new(Mode::Sequence, seed).layout;
            layout.sort_unstable();
            assert_eq!(layout, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], "seed {seed}");
        }
    }

    #[test]
    fn a_full_round_ends_at_round_length() {
        for seed in SEEDS {
            let mut d = Drill::new(Mode::PositionToColor, seed);
            for _ in 0..ROUND {
                assert!(!d.round_over());
                let target = d.question.map_or(0, |q| q.target);
                d.answer(target);
                d.next();
            }
            assert!(d.round_over());
            assert_eq!((d.asked, d.correct), (ROUND, ROUND));
        }
    }

    // — adversarial —

    #[test]
    fn double_answer_and_out_of_range_answer_are_ignored() {
        let mut d = Drill::new(Mode::ColorToPosition, 3);
        d.answer(12);
        d.answer(usize::MAX);
        assert_eq!((d.asked, d.picked), (0, None));
        let target = d.question.map_or(0, |q| q.target);
        d.answer(target);
        d.answer((target + 1) % 12);
        assert_eq!((d.asked, d.correct, d.picked), (1, 1, Some(target)));
    }

    #[test]
    fn next_before_answering_and_after_round_end_does_nothing() {
        let mut d = Drill::new(Mode::ColorToPosition, 11);
        let first = d.question;
        d.next();
        assert_eq!(d.question, first);
        for _ in 0..ROUND {
            d.answer(0);
            d.next();
        }
        let last = d.question;
        d.next();
        d.next();
        assert_eq!((d.question, d.asked), (last, ROUND));
        d.answer(0);
        assert_eq!(d.asked, ROUND);
    }

    #[test]
    fn questions_stay_in_range_and_never_repeat_back_to_back() {
        for seed in SEEDS {
            for mode in [Mode::ColorToPosition, Mode::PositionToColor] {
                let mut d = Drill::new(mode, seed);
                let mut previous: Option<usize> = None;
                for _ in 0..500 {
                    let Some(q) = d.question else {
                        panic!("quiz mode lost its question");
                    };
                    assert!(q.target < 12);
                    if let Ask::Fiber { tube } = q.ask {
                        assert!(tube < 12);
                    }
                    assert_ne!(Some(q.target), previous);
                    previous = Some(q.target);
                    d.picked = Some(0);
                    d.asked = 0;
                    d.next();
                }
            }
        }
    }

    #[test]
    fn sequence_counts_misses_ignores_replays_and_locks_when_done() {
        let mut d = Drill::new(Mode::Sequence, 5);
        assert_eq!(d.question, None);
        d.answer(0);
        assert_eq!(d.asked, 0);
        d.tap(3);
        assert_eq!((d.placed, d.misses, d.last_miss), (0, 1, Some(3)));
        d.tap(0);
        d.tap(0);
        d.tap(99);
        assert_eq!((d.placed, d.misses, d.last_miss), (1, 1, None));
        for color in 1..12 {
            d.tap(color);
        }
        assert!(d.round_over());
        d.tap(5);
        assert_eq!((d.placed, d.misses), (12, 1));
    }
}
