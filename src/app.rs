use turbocharger::prelude::*;
use turbosql::{now_ms, select, Turbosql};

automod::dir!(pub "src/app");
mod components;
#[allow(unused_imports)]
use components::*;

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
 client {
  rowid: None,
  timestamp: Some(now_ms()),
  animal_timestamp: Some(animal_time::now()),
  remote_addr: remote_addr!().map(|addr| addr.to_string()),
  user_agent: user_agent!().cloned(),
  client_pk: client_pk.try_into().ok(),
 }
 .insert()?;
 Ok(())
}

#[server_only]
#[tracked]
pub fn encrypt<T: AsRef<[u8]>>(m: T) -> Result<Vec<u8>, tracked::StringError> {
 let pk = crypto_box::PublicKey::from(select!(client "WHERE rowid = 1")?.client_pk?);
 Ok(sealed_box::seal(m.as_ref(), &pk))
}

#[frontend]
pub fn App(cx: Scope) -> Element {
 navbar::NavBar(
  cx,
  vec![
   ("Mail", mail::MailList),
   ("Bip39", bip39::Bip39),
   ("CheckForUpdates", check_for_updates::CheckForUpdates),
   ("AnimalTimeStream", animal_time_stream::AnimalTimeStream),
   ("Settings", settings::Settings),
  ],
 )
}
