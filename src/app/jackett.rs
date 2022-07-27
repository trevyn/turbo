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

// #[server_only]
// pub fn launch_jackett() {
//  #[cfg(target_os = "linux")]
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
 use_future(&cx, (), |_| jackett_search()).value().and_then(|r| match r {
  Ok(r) => rsx!(cx, p { "{r:?}" }),
  Err(e) => rsx!(cx, p { "error: {e}" }),
 })
}
