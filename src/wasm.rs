#![forbid(unsafe_code)]
#![allow(non_camel_case_types, non_snake_case, unknown_lints, clippy::derive_partial_eq_without_eq)]
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use turbocharger::prelude::*;

mod app;

#[wasm_only]
#[wasm_bindgen]
pub fn turbo_start_web() {
 dioxus::web::launch(app::App);
}
