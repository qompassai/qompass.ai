// The home page's view tree is deep enough (long static sections plus the
// reactive compass) that computing its layout overflows rustc's
// default query depth in the wasm build; 256 is the compiler's own remedy.
#![recursion_limit = "256"]

pub mod app;
pub mod download_data;
pub mod field_notes_data;
pub mod orcid_data;
pub mod pages;
pub mod project_data;
pub mod tiger_data;
pub mod tracker_data;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(crate::app::App);
    signal_hydration_complete();
}

/// Announce that the hydration walk has finished, so plain-script
/// islands that own a DOM subtree (public/js/orcid.js owns the two
/// ORCID lists on /research) may safely rewrite it.
///
/// The contract exists because hydration consumes tachys' region
/// markers (emitted in SSR HTML as bogus-comment nodes) as it
/// walks: a script that replaces those nodes mid-walk leaves the
/// walker facing a node of the wrong type, and it panics in
/// `tachys::hydration::failed_to_cast_marker_node`. `hydrate_body`
/// walks synchronously, so when it returns every marker has been
/// consumed and the DOM is safe to mutate. The flag covers
/// scripts that evaluate after the walk; the event covers
/// scripts already waiting when it finishes.
#[cfg(feature = "hydrate")]
fn signal_hydration_complete() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let target: &web_sys::EventTarget = window.as_ref();
    if let Ok(event) = web_sys::Event::new("qompass:hydrated") {
        let _ = target.dispatch_event(&event);
    }
    let window_js = wasm_bindgen::JsValue::from(window);
    let _ = js_sys::Reflect::set(
        &window_js,
        &wasm_bindgen::JsValue::from_str("__qompassHydrated"),
        &wasm_bindgen::JsValue::TRUE,
    );
}
