#![allow(unknown_lints, clippy::derive_partial_eq_without_eq)]
use turbocharger::prelude::*;

automod!(pub use "src/app");
pub mod components;
#[allow(unused_imports)]
pub use components::*;

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

#[backend(js)]
pub async fn notify_client_pk(client_pk: Vec<u8>) -> Result<(), tracked::StringError> {
 let rowid = client {
  rowid: None,
  timestamp: Some(now_ms()),
  animal_timestamp: Some(animal_time::now()),
  remote_addr: remote_addr!().map(|addr| addr.to_string()),
  user_agent: user_agent!().cloned(),
  client_pk: client_pk.try_into().ok(),
 }
 .insert()?;
 log::info!("inserted client_pk row {}", rowid);
 Ok(())
}

#[server_only]
#[tracked]
pub fn encrypt<T: AsRef<[u8]>>(m: T) -> Result<Vec<u8>, tracked::StringError> {
 let pk = crypto_box::PublicKey::from(select!(client "WHERE rowid = 1")?.client_pk?);
 Ok(crypto_box::seal(&mut rand::thread_rng(), &pk, m.as_ref())?)
}

#[frontend]
pub fn App(cx: Scope) -> Element {
 NavBar(
  cx,
  vec![
   ("Torrents", Torrents),
   ("Auth", Auth),
   ("Mail", MailList),
   ("Bip39", Bip39),
   ("CheckForUpdates", CheckForUpdates),
   ("AnimalTimeStream", AnimalTimeStream),
   ("Settings", Settings),
  ],
 )
}
