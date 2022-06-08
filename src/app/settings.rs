use turbocharger::prelude::*;

#[wasm_only]
pub fn Settings(cx: Scope) -> Element {
 let verified_sk = super::wasm_crypto::wasm_client_sk();
 let inputvalue = use_state(&cx, || verified_sk.clone());

 rsx! {cx,
  p { "{verified_sk}" }
  input { class: "font-mono text-gray-600",
   "type": "text",
   size: "64",
   value: "{inputvalue}",
   oninput: move |event| {
    super::wasm_crypto::wasm_set_client_sk(event.value.clone());
    inputvalue.set(event.value.clone());
   },
  }
 }
}
