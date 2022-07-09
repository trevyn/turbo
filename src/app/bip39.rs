use turbocharger::prelude::*;

#[frontend]
pub fn Bip39(cx: Scope) -> Element {
 let m = use_state(&cx, || {
  let mut entropy = [0u8; 16];
  rand_core::RngCore::fill_bytes(&mut rand_core::OsRng, &mut entropy);
  bip39::Mnemonic::from_entropy(&entropy).unwrap()
 })
 .get();

 let seed = hex::encode(m.to_seed(""));

 rsx! {cx,
  p { "{m}" }
  p { "{seed}" }
 }
}
