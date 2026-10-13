// The home page's view tree is deep enough (long static sections plus the
// reactive quick-start picker) that computing its layout overflows rustc's
// default query depth in the wasm build; 256 is the compiler's own remedy.
#![recursion_limit = "256"]

pub mod app;
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
}
