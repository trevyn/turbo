use turbocharger::prelude::*;

#[frontend]
pub fn AnimalTimeStream(cx: Scope) -> Element {
 use_stream(&cx, encrypted_animal_time_stream, |s, v| {
  *s = Some(super::wasm_crypto::wasm_decrypt(&v.unwrap_or_default()))
 })
 .read()
 .as_ref()
 .and_then(|r| match r {
  Ok(r) => rsx!(cx, p { "{r}" }),
  Err(e) => rsx!(cx, p { "error: {e}" }),
 })
}

#[backend]
fn encrypted_animal_time_stream() -> impl Stream<Item = Result<Vec<u8>, tracked::StringError>> {
 try_stream!({
  for i in 0.. {
   dbg!(i);
   let val = format!("{:?} - {} {}s!!", remote_addr!(), i, animal_time::now());
   let c = super::encrypt(val.as_bytes())?;
   yield c;
   tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  }
 })
}
