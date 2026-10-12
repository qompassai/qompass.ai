pub mod app;
pub mod orcid_data;
pub mod pages;
pub mod tracker_data;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(crate::app::App);
}
