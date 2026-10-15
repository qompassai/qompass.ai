//! Field Notes quizzes and glossary, compiled in by `build.rs`.
//!
//! Sources live in `content/field-notes/`: the telco-admin practice
//! sets copied byte-for-byte from Matt's study repository
//! (phaedrusflow/tds, Apache-2.0), the glossary adapted from the same
//! repository, and the glossary's equations pre-rendered to SVG by
//! `scripts/generate-field-notes-equations.cjs`.

include!(concat!(env!("OUT_DIR"), "/field_notes_data.rs"));

/// Total questions across every ported set.
pub fn question_count() -> usize {
    QUIZZES.iter().map(|q| q.questions.len()).sum()
}

/// Total equations across every glossary term.
pub fn equation_count() -> usize {
    GLOSSARY.iter().map(|t| t.equations.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::{equation_count, question_count, GLOSSARY, QUIZZES};

    #[test]
    fn every_set_is_ported_whole() {
        assert_eq!(QUIZZES.len(), 12);
        assert_eq!(question_count(), 359);
        for quiz in QUIZZES {
            for q in quiz.questions {
                assert!(q.answer < q.choices.len(), "{} q{}", quiz.slug, q.number);
                assert!(
                    q.choice_notes.is_empty() || q.choice_notes.len() == q.choices.len(),
                    "{} q{}",
                    quiz.slug,
                    q.number
                );
            }
        }
    }

    #[test]
    fn glossary_is_alphabetical_with_unique_anchors() {
        assert_eq!(equation_count(), 110);
        let mut slugs: Vec<&str> = GLOSSARY.iter().map(|t| t.slug).collect();
        for pair in GLOSSARY.windows(2) {
            let [a, b] = pair else { continue };
            assert!(a.term.to_lowercase() < b.term.to_lowercase(), "{} before {}", a.term, b.term);
        }
        slugs.sort_unstable();
        slugs.dedup();
        assert_eq!(slugs.len(), GLOSSARY.len());
    }

    #[test]
    fn every_equation_has_a_legend_and_a_plain_line() {
        for term in GLOSSARY {
            for e in term.equations {
                assert!(e.svg.starts_with("<svg "), "{}", e.id);
                assert!(!e.plain.is_empty(), "{}", e.id);
                assert!(!e.legend.is_empty() || !e.tex.chars().any(char::is_alphabetic), "{}", e.id);
            }
        }
    }
}
