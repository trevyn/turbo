use turbocharger::backend;

use turbosql::Turbosql;

#[turbocharger::server_only]
use {anyhow::Context, turbosql::select};

#[backend]
#[derive(Turbosql)]
pub struct Person {
 pub rowid: Option<i64>,
 pub name: Option<String>,
}

#[backend]
pub async fn insert_person(p: Person) -> Result<i64, turbosql::Error> {
 p.insert() // returns rowid
}

#[backend]
pub async fn get_person(rowid: i64) -> Result<Person, turbosql::Error> {
 select!(Person "WHERE rowid = ?", rowid)
}

#[backend]
pub async fn get_new_secret_key() -> Result<String, anyhow::Error> {
 turbonet::KeyMaterial::generate_new();
 Ok("(it's a secret)".to_string())
}

#[backend]
pub async fn getblockchaininfo() -> Result<String, anyhow::Error> {
 let cookie = std::fs::read_to_string("/root/.bitcoin/.cookie")?;
 let mut cookie_iter = cookie.split(':');
 let username = cookie_iter.next().context("no username")?;
 let password = cookie_iter.next().context("no password")?;

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
pub async fn check_for_updates() -> Result<String, anyhow::Error> {
 let result = tokio::task::spawn_blocking(move || -> Result<String, anyhow::Error> {
  eprintln!("checking for releases");

  let releases = self_update::backends::github::ReleaseList::configure()
   .repo_owner("trevyn")
   .repo_name("turbo")
   .build()
   .unwrap()
   .fetch()
   .unwrap();
  eprintln!("found releases:");
  eprintln!("{:#?}\n", releases);

  // get the first available release
  // self_update::get_target()
  let asset = releases[0].asset_for("linux").unwrap();

  dbg!(&releases[0]);

  // let tmp_dir =
  //  tempfile::Builder::new().prefix("self_update").tempdir_in(::std::env::current_dir()?)?;
  // let tmp_file_path = tmp_dir.path().join(&asset.name);
  // let tmp_file = ::std::fs::File::create(&tmp_file_path)?;

  // let header_value: Result<reqwest::header::HeaderValue, _> = "application/octet-stream".parse();
  // let header_value = header_value?;

  // self_update::Download::from_url(&asset.download_url)
  //  .set_header(reqwest::header::ACCEPT, header_value)
  //  .download_to(&tmp_file)?;

  Ok(format!("{:?}", releases))
 })
 .await?;

 result
}
