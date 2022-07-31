// https://github.com/Jackett/Jackett/releases/latest/download/Jackett.Binaries.LinuxAMDx64.tar.gz

#[frontend]
use super::components::button::Button;

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

#[backend]
pub fn configure_jackett() -> impl Stream<Item = Result<String, tracked::StringError>> {
 try_stream!({
  yield format!("configuring jackett...");

  let client = reqwest::Client::builder().cookie_store(true).build()?;

  let _ = client.get("http://localhost:9117/UI/Login").send().await?.text().await?;

  yield format!("logged in...");

  let res = client.post("http://localhost:9117/api/v2.0/indexers/rarbg/config")
   .header(reqwest::header::CONTENT_TYPE, "application/json")
   .body(r#"[{"id":"sitelink","type":"inputstring","name":"Site Link","value":"https://rarbg.to/"},{"id":"apiurl","type":"inputstring","name":"API URL","value":"https://torrentapi.org/pubapi_v2.php"},{"id":"sortrequestedfromsite","type":"inputselect","name":"Sort requested from site","value":"last","options":{"last":"created","seeders":"seeders","leechers":"leechers"}},{"id":"numberofretries","type":"inputselect","name":"Number of retries","value":"5","options":{"0":"No retries (fail fast)","1":"1 retry (0.5s delay)","2":"2 retries (1s delay)","3":"3 retries (2s delay)","4":"4 retries (4s delay)","5":"5 retries (8s delay)"}},{"id":"tags","type":"inputtags","name":"Tags","value":"","separator":",","delimiters":"[^A-Za-z0-9\\-\\._~]+","pattern":"^[A-Za-z0-9\\-\\._~]+$"}]"#)
   .send()
   .await?;

  let status = res.status();
  let text = res.text().await?;

  yield format!("jackett configured, status {} {}", status, text);
 })
}

#[backend]
pub async fn search_jackett() -> Result<JackettResults, tracked::StringError> {
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
 let download_status = use_state(&cx, || "download_status".to_string());
 let configure_status = use_state(&cx, || "configure_status".to_string());
 let search_status = use_state(&cx, || "search_status".to_string());

 rsx!(cx, p {
  p { class: "p-4", Button { onclick: move |_| {
   to_owned![download_status];
   cx.spawn(async move {
    let stream = download_jackett();
    pin_mut!(stream);
    while let Some(r) = stream.next().await {
     download_status.set(match r {
      Ok(r) => r,
      Err(e) => e.into(),
     });
    };
   });
  }, "Download Jackett" }}

  p{ class: "p-4", button { onclick: move |_| {
   to_owned![configure_status];
   cx.spawn(async move {
    let stream = configure_jackett();
    pin_mut!(stream);
    while let Some(r) = stream.next().await {
     configure_status.set(match r {
      Ok(r) => r,
      Err(e) => e.into(),
     });
    };
   });
  }, "Configure Jackett" }}

  p{ class: "p-4", button { onclick: move |_| {
   to_owned![search_status];
   cx.spawn(async move {
    search_status.set(match search_jackett().await {
     Ok(r) => format!("{:?}", r),
     Err(e) => e.into(),
    });
   });
  }, "Search Jackett" }}

  p { class: "p-4", "{download_status}" }
  p { class: "p-4", "{configure_status}" }
  p { class: "p-4", "{search_status}" }
 })
}
