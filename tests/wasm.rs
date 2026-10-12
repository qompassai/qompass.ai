//! Browser-run integration tests for the WASM (hydrate) side.
//!
//! Run with `wasm-pack test --headless --firefox` (or `--chrome`). The
//! data contracts below are the ones the Research page relies on when
//! it renders the baked-in ORCID snapshot and the public research
//! tracker, and the DOM test exercises the web-sys stack the hydrate
//! entry point mounts into. On non-WASM targets this file is empty:
//! the contracts only mean something in the browser build.
#![cfg(target_arch = "wasm32")]

use qompass_ai::orcid_data::WORKS;
use qompass_ai::tracker_data::TRACKER;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::wasm_bindgen::JsValue;

wasm_bindgen_test_configure!(run_in_browser);

/// Stage ordering rank, mirroring `stage_rank` in build.rs: the
/// tracker is emitted sorted by this rank, most-advanced first.
fn stage_rank(stage: &str) -> u8 {
    match stage {
        "ready" => 0,
        "in-review" => 1,
        "drafting" => 2,
        "soundings" => 3,
        "published" => 4,
        _ => 5,
    }
}

#[wasm_bindgen_test]
fn tracker_emits_only_public_entries_in_stage_order() {
    // The two private seeds in content/tracker/ must never be emitted:
    // private work entering the build artifacts would ship to every
    // visitor in the WASM and the prerendered HTML.
    let private_titles =
        ["Qompass Benchmark v1 (Kaggle)", "The grounded/ungrounded gap: MATE protocol coordination results"];
    let mut previous_rank: Option<u8> = None;
    for entry in TRACKER {
        let title = entry.title;
        let stage = entry.stage;
        assert!(!title.is_empty(), "tracker entry needs a title");
        assert!(!stage.is_empty(), "tracker entry needs a stage");
        assert!(!private_titles.contains(&title), "private tracker entry leaked into the build: {title}");
        let rank = stage_rank(stage);
        assert!(rank < 5, "unknown tracker stage: {stage}");
        if let Some(previous) = previous_rank {
            assert!(previous <= rank, "tracker entries out of stage order");
        }
        previous_rank = Some(rank);
    }
}

#[wasm_bindgen_test]
fn works_carry_titles_and_absolute_urls() {
    assert!(!WORKS.is_empty(), "ORCID snapshot must not be empty");
    for work in WORKS {
        let title = work.title;
        let url = work.url;
        assert!(!title.is_empty(), "every work needs a title");
        // Absolute http(s), not https-only: ORCID itself carries one
        // http:// Scopus link, and the page renders what ORCID says.
        // The contract that matters is "absolute and followable".
        assert!(url.starts_with("https://") || url.starts_with("http://"), "work URL must be absolute http(s): {url}");
        if let Some(doi) = url.strip_prefix("https://doi.org/") {
            assert!(!doi.is_empty(), "DOI URL must carry an identifier: {url}");
        }
    }
}

#[wasm_bindgen_test]
fn works_include_the_corr_letter() {
    // Anchor record: the CORR letter on generative AI and value-based
    // healthcare. If a snapshot regeneration drops it, the snapshot
    // pipeline is broken, not the record.
    let corr = "https://doi.org/10.1097/corr.0000000000003001";
    assert!(WORKS.iter().any(|work| work.url == corr), "the CORR letter must stay in the ORCID snapshot");
}

#[wasm_bindgen_test]
fn works_years_are_plausible() {
    for work in WORKS {
        if work.year.is_empty() {
            // ORCID carries no date for a few entries; those render
            // without one. A non-empty year must be a real year.
            continue;
        }
        let year = work.year;
        let parsed = year.parse::<u32>();
        assert!(parsed.is_ok(), "year must be numeric: {year}");
        if let Ok(value) = parsed {
            assert!((2000..=2100).contains(&value), "year out of range: {value}");
        }
    }
}

#[wasm_bindgen_test]
fn dom_round_trip_through_web_sys() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("browser window missing"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("browser document missing"))?;
    let body = document.body().ok_or_else(|| JsValue::from_str("browser body missing"))?;
    let probe = document.create_element("section")?;
    probe.set_attribute("id", "wasm-test-probe")?;
    probe.set_text_content(Some("qompass probe"));
    let appended = body.append_child(&probe);
    assert!(appended.is_ok(), "append_child failed: {appended:?}");
    let found = document.query_selector("#wasm-test-probe")?;
    assert!(found.is_some(), "probe element must be findable");
    if let Some(node) = found {
        assert_eq!(node.text_content(), Some("qompass probe".to_owned()));
    }
    probe.remove();
    let after = document.query_selector("#wasm-test-probe")?;
    assert!(after.is_none(), "probe element must be removable");
    Ok(())
}
