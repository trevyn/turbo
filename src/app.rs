#![allow(unused_imports)]
use crypto_box::PublicKey;
use serde::{Deserialize, Serialize};
use tracked::tracked;
use turbocharger::{backend, prelude::*, server_only};
use turbosql::{now_ms, select, Turbosql};

mod check_for_updates;
pub use check_for_updates::check_for_updates;

#[derive(Turbosql, Default, Debug, Clone, Serialize, Deserialize)]
pub struct mail {
 pub rowid: Option<i64>,
 pub recv_ms: Option<i64>,
 pub recv_ip_enc: Option<Vec<u8>>,
 pub domain_enc: Option<Vec<u8>>,
 pub from_addr_enc: Option<Vec<u8>>,
 pub is8bit: Option<bool>,
 pub to_addr_enc: Option<Vec<u8>>,
 pub data: Option<Vec<u8>>,
}

#[backend]
#[derive(Turbosql, Default)]
pub struct animal_time_stream_log {
 pub rowid: Option<i64>,
 pub timestamp: Option<i64>,
 pub animal_timestamp: Option<String>,
 pub remote_addr: Option<String>,
 pub user_agent: Option<String>,
}

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
#[tracked]
pub async fn getblockchaininfo() -> Result<String, tracked::StringError> {
 let cookie = std::fs::read_to_string("/root/.bitcoin/.cookie")?;
 let [username, password]: [&str; 2] = cookie.split(':').collect::<Vec<&str>>().try_into().ok()?;

 Ok(
  reqwest::Client::new()
   .post("http://127.0.0.1:8332")
   .basic_auth(username, Some(password))
   .body(r#"{"jsonrpc": "1.0", "id":"test", "method": "getblockchaininfo", "params": [] }"#)
   .send()
   .await?
   .text()
   .await?,
 )
}

#[backend(js)]
pub async fn animal_time() -> String {
 animal_time::now()
}

#[tracked]
#[backend(js)]
pub async fn notify_client_pk(client_pk: Vec<u8>) -> Result<(), tracked::StringError> {
 client {
  rowid: None,
  timestamp: Some(now_ms()),
  animal_timestamp: Some(animal_time().await),
  remote_addr: remote_addr.map(|addr| addr.to_string()),
  user_agent,
  client_pk: client_pk.try_into().ok(),
 }
 .insert()?;
 Ok(())
}

// #[backend]
// fn animal_time_stream() -> impl Stream<Item = String> {
//  turbocharger::async_stream::stream! {
//   let mut i = 0;
//   loop {
//    dbg!(i);
//    yield format!("{:?} {:?} - {} {}s!!", remote_addr, user_agent, i, animal_time().await);
//    i += 1;
//    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//   }
//  }
// }

#[tracked]
#[backend(js)]
fn animal_time_stream() -> impl Stream<Item = Result<String, tracked::StringError>> {
 turbocharger::async_stream::try_stream!({
  animal_time_stream_log {
   rowid: None,
   timestamp: Some(now_ms()),
   animal_timestamp: Some(animal_time().await),
   remote_addr: remote_addr.map(|addr| addr.to_string()),
   user_agent,
  }
  .insert()?;
  for i in 0.. {
   dbg!(i);
   if i == 5 {
    Err("oh no")?;
   }
   yield format!("{:?} - {} {}s!!", remote_addr, i, animal_time().await);
   tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  }
 })
}

#[tracked]
#[server_only]
pub fn encrypt<T: AsRef<[u8]>>(m: T) -> Result<Vec<u8>, tracked::StringError> {
 let pk = crypto_box::PublicKey::from(select!(client "WHERE rowid = 1")?.client_pk?);
 Ok(sealed_box::seal(m.as_ref(), &pk))
}

#[tracked]
#[backend]
pub fn encrypted_animal_time_stream() -> impl Stream<Item = Result<Vec<u8>, tracked::StringError>> {
 turbocharger::async_stream::try_stream!({
  for i in 0.. {
   dbg!(i);
   let val = format!("{:?} - {} {}s!!", remote_addr, i, animal_time().await);
   let c = encrypt(val.as_bytes())?;
   yield c;
   tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  }
 })
}

#[tracked]
#[backend]
pub async fn mail(rowid: i64) -> Result<Vec<u8>, tracked::StringError> {
 Ok(select!(mail "WHERE rowid = " rowid)?.data?)
}

#[tracked]
#[backend]
pub async fn mail_list() -> Result<Vec<i64>, tracked::StringError> {
 // Ok(select!(Vec<mail.rowid> "ORDER BY recv_ms DESC, rowid DESC")?)
 Ok(select!(Vec<i64> "SELECT rowid FROM mail ORDER BY recv_ms DESC, rowid DESC")?)
}

#[server_only]
#[tracked]
fn row_to_string(row: animal_time_stream_log) -> Result<String, tracked::StringError> {
 Ok(format!(
  "{} {} {}\n{}\n",
  row.remote_addr?, row.animal_timestamp?, row.timestamp?, row.user_agent?
 ))
}

#[backend(js)]
#[tracked]
pub async fn animal_log() -> Result<String, tracked::StringError> {
 Ok(
  select!(Vec<animal_time_stream_log> "ORDER BY rowid DESC")?
   .into_iter()
   .map(row_to_string)
   .collect::<Result<Vec<_>, _>>()?
   .join("\n"),
 )
}

#[tracked]
#[backend(js)]
fn stream_example_result() -> impl Stream<Item = Result<String, tracked::StringError>> {
 turbocharger::async_stream::try_stream!({
  for i in 0.. {
   yield format!("r{}", i);
   if i == 5 {
    None?;
   }
   tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  }
 })
}
