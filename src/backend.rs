#![allow(unused_imports)]
use crypto_box::PublicKey;
use tracked::tracked;
use turbocharger::{backend, server_only};
use turbosql::{now_ms, select, Turbosql};

#[derive(Turbosql, Default, Clone)]
pub struct mail {
 pub rowid: Option<i64>,
 pub recv_ms: Option<i64>,
 pub recv_ip: Option<String>,
 pub domain: Option<String>,
 pub from_addr: Option<String>,
 pub is8bit: Option<bool>,
 pub to_addr: Option<String>,
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

#[backend]
pub async fn heartbeat() -> Result<String, tracked::Error> {
 Ok("beat".to_string())
}

#[backend]
#[tracked]
pub async fn getblockchaininfo() -> Result<String, tracked::Error> {
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

#[backend]
pub async fn animal_time() -> String {
 animal_time::now()
}

#[tracked]
#[backend]
pub async fn notify_client_pk(client_pk: Vec<u8>) -> Result<(), tracked::Error> {
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
#[backend]
#[async_stream::try_stream_attribute]
fn animal_time_stream() -> impl Stream<Item = Result<String, tracked::Error>> {
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
  yield format!("{:?} - {} {}s!!", remote_addr, i, animal_time().await);
  tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
 }
}

#[tracked]
#[server_only]
pub fn encrypt<T: AsRef<[u8]>>(m: T) -> Result<Vec<u8>, tracked::Error> {
 let pk = crypto_box::PublicKey::from(select!(client "WHERE rowid = 1")?.client_pk?);
 Ok(sealed_box::seal(m.as_ref(), &pk))
}

#[tracked]
#[backend]
#[async_stream::try_stream_attribute]
fn encrypted_animal_time_stream() -> impl Stream<Item = Result<String, tracked::Error>> {
 for i in 0.. {
  dbg!(i);
  let val = format!("{:?} - {} {}s!!", remote_addr, i, animal_time().await);
  let c = encrypt(val.as_bytes())?;
  yield hex::encode(c);
  tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
 }
}

#[tracked]
#[backend]
async fn mail(rowid: i64) -> Result<String, tracked::Error> {
 let data = select!(mail "WHERE rowid = " rowid)?.data?;
 Ok(hex::encode(data))
}

#[server_only]
#[tracked]
fn row_to_string(row: animal_time_stream_log) -> Result<String, tracked::Error> {
 use tracked::Track;
 Ok(format!(
  "{} {} {}\n{}\n",
  Track::t(row.remote_addr)?,
  Track::t(row.animal_timestamp)?,
  Track::t(row.timestamp)?,
  Track::t(row.user_agent)?
 ))
}

#[backend]
#[tracked]
pub async fn animal_log() -> Result<String, tracked::Error> {
 Ok(
  select!(Vec<animal_time_stream_log> "ORDER BY rowid DESC")?
   .into_iter()
   .map(row_to_string)
   .collect::<Result<Vec<_>, _>>()?
   .join("\n"),
 )
}

#[tracked]
#[backend]
#[async_stream::try_stream_attribute]
fn stream_example_result() -> impl Stream<Item = Result<String, tracked::Error>> {
 for i in 0.. {
  yield format!("r{}", i);
  if i == 5 {
   None?;
  }
  tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
 }
}

#[backend]
#[tracked]
pub async fn check_for_updates() -> Result<String, tracked::Error> {
 // TODO: handle race conditions
 // also, this seems to block the executor if slow, maybe put it in a spawn_blocking?

 use std::os::unix::{prelude::OpenOptionsExt, process::CommandExt};

 if option_env!("BUILD_ID").is_none() {
  return Ok(format!(
   "Running DEV {}; updates disabled on DEV.",
   option_env!("BUILD_TIME").unwrap_or_default()
  ));
 }

 let res = reqwest::Client::builder()
  .redirect(reqwest::redirect::Policy::none())
  .build()?
  .get("https://github.com/trevyn/turbo/releases/latest/download/turbo-linux")
  .send()
  .await?;

 tracked::ensure!(res.status() == 302);
 let location = res.headers().get(reqwest::header::LOCATION)?.to_str()?;

 let new_version =
  regex::Regex::new(r"/releases/download/([a-z]+-[a-z]+)/")?.captures(location)?.get(1)?.as_str();

 if option_env!("BUILD_ID").unwrap_or_default() == new_version {
  Ok(format!("Running latest! {}", new_version))
 } else {
  let bytes = reqwest::get(location).await?.bytes().await?;
  if bytes.len() < 14_000_000 {
   tracked::bail!(
    "Not updating; new release {} is unexpectedly small: {} bytes.",
    new_version,
    bytes.len()
   );
  }
  let current_exe = std::env::current_exe()?;
  std::fs::remove_file(&current_exe)?;
  let mut f =
   std::fs::OpenOptions::new().create(true).write(true).mode(0o700).open(&current_exe)?;
  std::io::Write::write_all(&mut f, &bytes)?;
  f.sync_all()?;

  tokio::spawn(async move {
   tokio::time::sleep(std::time::Duration::from_secs(2)).await;
   std::process::Command::new(current_exe).exec();
  });

  Ok(format!(
   "Updated from {} to {}, {} bytes, relaunching!",
   option_env!("BUILD_ID").unwrap_or_default(),
   new_version,
   bytes.len()
  ))
 }
}
