// https://github.com/Jackett/Jackett/releases/latest/download/Jackett.Binaries.LinuxAMDx64.tar.gz

use serde::{Deserialize, Serialize};
use turbocharger::prelude::*;
use turbosql::Turbosql;

#[derive(Debug, Deserialize)]
pub struct ConfigResponse {
 pub api_key: String,
 pub app_version: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JackettResults {
 pub indexers: Vec<JackettIndexer>,
 pub results: Vec<JackettResult>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JackettIndexer {
 #[serde(alias = "ID")]
 pub id: String,
 pub name: String,
 pub status: i64,
 pub results: i64,
 pub error: Option<String>,
}

#[derive(Turbosql, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JackettResult {
 pub rowid: Option<i64>,
 pub tracker: Option<String>,
 pub tracker_id: Option<String>,
 pub category_desc: Option<String>,
 pub title: Option<String>,
 pub guid: Option<String>,
 pub link: Option<String>,
 pub details: Option<String>,
 pub publish_date: Option<String>,
 pub category: Option<Vec<i64>>,
 pub size: Option<i64>,
 pub seeders: Option<i64>,
 pub peers: Option<i64>,
 pub gain: Option<f64>,
}

#[backend]
pub fn download_jackett() -> impl Stream<Item = Result<String, tracked::StringError>> {
 // #[cfg(target_os = "linux")]
 try_stream!({
  yield "downloading jackett...".into();

  let mut bytes = Vec::new();
  let res = reqwest::get("https://github.com/Jackett/Jackett/releases/latest/download/Jackett.Binaries.LinuxAMDx64.tar.gz").await?;
  let total_size = res.content_length()?;
  let mut stream = res.bytes_stream();

  while let Some(item) = stream.next().await {
   bytes.extend_from_slice(&item?);
   yield format!("downloading jackett {}/{}...", bytes.len(), total_size);
  }

  yield format!("downloading jackett complete, {} bytes...", bytes.len());

  // save to disk

  std::fs::write("/home/turbo/Jackett.Binaries.LinuxAMDx64.tar.gz", bytes)?;

  yield format!("saved to disk, extracting...");

  // extract

  let output = std::process::Command::new("tar")
   .args(["-xvf", "Jackett.Binaries.LinuxAMDx64.tar.gz"])
   .current_dir("/home/turbo")
   .output()?;

  yield format!("extracted: {}", String::from_utf8_lossy(&output.stderr));
 })
}

// #[backend]
// pub fn run_jackett() -> impl Stream<Item = Result<String, tracked::StringError>> {
//  try_stream!({

//  }
// }

#[backend]
pub async fn jackett_search() -> Result<JackettResults, tracked::StringError> {
 let client = reqwest::Client::builder().cookie_store(true).build()?;

 let resp = client
  .get("http://localhost:9117/api/v2.0/server/config")
  .send()
  .await?
  .json::<ConfigResponse>()
  .await?;

 println!("{:#?}", resp);

 let api_key = resp.api_key;

 let resp = client
  .get(format!(
   "http://localhost:9117/api/v2.0/indexers/all/results?apikey={}&Query=test&Tracker[]=rarbg",
   api_key
  ))
  .send()
  .await?
  .json::<JackettResults>()
  .await?;

 println!("{:#?}", resp);

 Ok(resp)
}

#[frontend]
pub fn JackettList(cx: Scope) -> Element {
 let stream =
  use_stream(&cx, download_jackett, |s, v| *s = Some(v)).read().as_ref().and_then(|r| match r {
   Ok(r) => rsx!(cx, p { "{r}" }),
   Err(e) => rsx!(cx, p { "error: {e}" }),
  });

 let search = use_future(&cx, (), |_| jackett_search()).value().and_then(|r| match r {
  Ok(r) => rsx!(cx, p { "{r:?}" }),
  Err(e) => rsx!(cx, p { "error: {e}" }),
 });

 rsx!(cx, p {
  p { class: "p-4", stream }
  p { class: "p-4", search }
 })
}
