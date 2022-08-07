use turbocharger::prelude::*;

#[frontend]
pub fn Settings(cx: Scope) -> Element {
 let verified_sk = super::wasm_crypto::wasm_client_sk();
 let inputvalue = use_state(&cx, || verified_sk.clone());

 rsx! {cx,
  super::button::Button {
   onclick: move |_| {
    cx.spawn(async move {
     let _ = super::wasm_crypto::wasm_notify_client_pk().await;
    });
   },
   "Notify Client PK"
  }

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
