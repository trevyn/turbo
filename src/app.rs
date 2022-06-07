use turbocharger::prelude::*;
use turbosql::{now_ms, select, Turbosql};

mod animal_time_stream;
mod bip39;
mod check_for_updates;
pub mod mail;

#[cfg(any(feature = "wasm", target_arch = "wasm32"))]
#[path = "wasm_crypto.rs"]
pub mod wasm_crypto;

#[derive(Turbosql, Default)]
pub struct client {
 pub rowid: Option<i64>,
 pub timestamp: Option<i64>,
 pub animal_timestamp: Option<String>,
 pub remote_addr: Option<String>,
 pub user_agent: Option<String>,
 pub client_pk: Option<[u8; 32]>,
}

#[backend(js)]
pub async fn heartbeat() -> Result<String, tracked::StringError> {
 Ok("beat".to_string())
}

#[tracked]
#[backend(js)]
pub async fn notify_client_pk(client_pk: Vec<u8>) -> Result<(), tracked::StringError> {
 client {
  rowid: None,
  timestamp: Some(now_ms()),
  animal_timestamp: Some(animal_time::now()),
  remote_addr: remote_addr.map(|addr| addr.to_string()),
  user_agent,
  client_pk: client_pk.try_into().ok(),
 }
 .insert()?;
 Ok(())
}

#[tracked]
#[server_only]
pub fn encrypt<T: AsRef<[u8]>>(m: T) -> Result<Vec<u8>, tracked::StringError> {
 let pk = crypto_box::PublicKey::from(select!(client "WHERE rowid = 1")?.client_pk?);
 Ok(sealed_box::seal(m.as_ref(), &pk))
}

#[wasm_only]
pub fn App(cx: Scope) -> Element {
 rsx! {cx,
  bip39::Bip39()
  check_for_updates::CheckForUpdates()
  animal_time_stream::AnimalTimeStream()
  mail::MailList()
 }
}
