#![feature(generators)]
#![allow(non_camel_case_types)]
#![forbid(unsafe_code)]

mod backend;

use turbocharger::wasm_only;

#[wasm_only]
#[wasm_bindgen]
pub fn wasm_new_secret_key() -> String {
 let mut rng = crypto_box::rand_core::OsRng;
 let crypto_box_secret_key = crypto_box::SecretKey::generate(&mut rng);
 hex::encode(crypto_box_secret_key.as_bytes())
}
