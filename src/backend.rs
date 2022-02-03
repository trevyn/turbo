#[turbocharger::server_only]
use tracked::tracked;

use turbocharger::backend;

// use turbosql::Turbosql;

// #[backend]
// #[derive(Turbosql)]
// pub struct Person {
//  pub rowid: Option<i64>,
//  pub name: Option<String>,
// }

// #[backend]
// pub async fn insert_person(p: Person) -> Result<i64, turbosql::Error> {
//  p.insert() // returns rowid
// }

// #[backend]
// pub async fn get_person(rowid: i64) -> Result<Person, turbosql::Error> {
//  select!(Person "WHERE rowid = ?", rowid)
// }

// #[backend]
// pub async fn get_new_secret_key() -> Result<String, anyhow::Error> {
//  turbonet::KeyMaterial::generate_new();
//  Ok("(it's a secret)".to_string())
// }

#[backend]
pub async fn heartbeat() -> Result<String, tracked::Error> {
 Ok("beat".to_string())
}

#[backend]
#[tracked]
pub async fn getblockchaininfo() -> Result<String, tracked::Error> {
 let cookie = std::fs::read_to_string("/root/.bitcoin/.cookie")?;
 let mut cookie_iter = cookie.split(':');
 let username = cookie_iter.next()?;
 let password = cookie_iter.next()?;

 let client = reqwest::Client::new();
 let res = client
  .post("http://127.0.0.1:8332")
  .basic_auth(username, Some(password))
  .body(r#"{"jsonrpc": "1.0", "id":"curltest", "method": "getblockchaininfo", "params": [] }"#)
  .send()
  .await?
  .text()
  .await?;
 Ok(res)
}

#[backend]
pub async fn animal_time() -> String {
 animal_time::now()
}

#[backend]
fn animal_time_stream() -> impl Stream<Item = String> {
 turbocharger::async_stream::stream! {
  let mut i = 0;
  loop {
   yield format!("{} {}s!!", i, animal_time::now());
   i += 1;
   tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  }
 }
}

#[backend]
#[tracked]
pub async fn check_for_updates() -> Result<String, tracked::Error> {
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
  .get("https://github.com/trevyn/turbo/releases/latest/download/turbo-x86_64-unknown-linux-gnu")
  .send()
  .await?;

 tracked::ensure!(res.status() == 302);
 let location = res.headers().get(reqwest::header::LOCATION)?.to_str()?;

 let new_version =
  regex::Regex::new(r"/releases/download/([a-z]+-[a-z]+)/")?.captures(location)?.get(1)?.as_str();

 if option_env!("BUILD_ID").unwrap_or_default() == new_version {
  Ok(format!("Running latest! {}", new_version))
 } else {
  let res = reqwest::get(location).await?;
  let bytes = res.bytes().await?;
  let current_exe = std::env::current_exe()?;
  std::fs::remove_file(&current_exe)?;
  let mut f =
   std::fs::OpenOptions::new().create(true).write(true).mode(0o700).open(&current_exe)?;
  std::io::Write::write_all(&mut f, &bytes)?;
  f.sync_all()?;

  tokio::spawn(async move {
   tokio::time::sleep(std::time::Duration::from_secs(1)).await;
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
