// https://github.com/Jackett/Jackett/releases/latest/download/Jackett.Binaries.LinuxAMDx64.tar.gz

use serde::Deserialize;
use turbocharger::prelude::*;
use turbosql::Turbosql;

#[derive(Deserialize, Debug)]
pub struct ConfigResponse {
 pub api_key: String,
 pub app_version: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct JackettResults {
 pub results: Vec<JackettResult>,
}

#[derive(Turbosql, Debug, Deserialize)]
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
pub async fn jackett_search() -> Result<(), tracked::StringError> {
 let client = reqwest::Client::builder().cookie_store(true).build().unwrap();

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

 Ok(())
}

#[frontend]
pub fn JackettList(cx: Scope) -> Element {
 use_future(&cx, (), |_| jackett_search()).value();

 rsx! {cx,
  "jackett"
 }
}
