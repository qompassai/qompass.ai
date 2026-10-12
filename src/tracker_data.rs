//! Public research-tracker entries, compiled in by `build.rs`.
//!
//! The data comes from `content/tracker/*.toml`. Only entries flagged
//! `public` are emitted, and an entry's abstract is included only when
//! `abstract_public` is also set — private work never enters the build
//! artifacts.
include!(concat!(env!("OUT_DIR"), "/tracker_data.rs"));
