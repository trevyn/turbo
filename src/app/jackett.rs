use super::*;

#[derive(Debug, Deserialize)]
pub struct ConfigResponse {
 pub api_key: String,
 pub app_version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JackettResults {
 pub indexers: Vec<JackettIndexer>,
 pub results: Vec<JackettResult>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JackettIndexer {
 #[serde(alias = "ID")]
 pub id: String,
 pub name: String,
 pub status: i64,
 pub results: i64,
 pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Turbosql)]
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
  connection_local!(authed: &mut bool);
  if !*authed {
   Err("not authed")?;
  }

  yield "downloading jackett...".into();

  let res = reqwest::get("https://github.com/Jackett/Jackett/releases/latest/download/Jackett.Binaries.LinuxAMDx64.tar.gz").await?;
  let total_size: usize = res.content_length()?.try_into()?;
  let mut bytes = Vec::with_capacity(total_size);
  let mut stream = res.bytes_stream();

  while let Some(item) = stream.next().await {
   bytes.extend_from_slice(&item?);
   yield format!(
    "downloading jackett {}% {}/{}...",
    bytes.len() * 100 / total_size,
    bytes.len(),
    total_size
   );
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

  if !output.status.success() {
   Err(format!("Extract failed, exit code {:?}: {:?}", output.status.code(), &output.stderr))?;
  }

  yield format!(
   "extracted: {:?} {:?} {:?}",
   String::from_utf8_lossy(&output.stdout),
   &output.stdout,
   &output.stderr
  );
 })
}

#[backend]
pub fn launch_jackett() -> impl Stream<Item = Result<String, tracked::StringError>> {
 try_stream!({
  connection_local!(authed: &mut bool);
  if !*authed {
   Err("not authed")?;
  }

  yield "launching jackett...".into();

  std::process::Command::new("/home/turbo/Jackett/jackett")
   .current_dir("/home/turbo/Jackett")
   .spawn()?;

  yield "launched.".into();
 })
}

#[backend]
pub fn configure_jackett() -> impl Stream<Item = Result<String, tracked::StringError>> {
 try_stream!({
  connection_local!(authed: &mut bool);
  if !*authed {
   Err("not authed")?;
  }

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

  yield format!("jackett configured, status {} {:?}", status, text);
 })
}

#[backend]
pub fn search_jackett(
) -> impl Stream<Item = Result<(String, Option<JackettResults>), tracked::StringError>> {
 try_stream!({
  connection_local!(authed: &mut bool);
  if !*authed {
   Err("not authed")?;
  }

  yield ("Searching...".into(), None);

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

  yield (format!("{} results", resp.results.len()), Some(resp));
 })
}

#[backend]
pub fn do_test_action() -> impl Stream<Item = Result<String, tracked::StringError>> {
 try_stream!({
  connection_local!(authed: &mut bool);
  if !*authed {
   Err("not authed")?;
  }

  yield format!("doing test action...");
  tokio::time::sleep(std::time::Duration::from_secs(2)).await;
  yield format!("test action done!");
 })
}

#[frontend]
pub fn JackettList(cx: Scope) -> Element {
 let results = use_state(&cx, || None);

 rsx!(cx, p {
  ActionButton{action: do_test_action, "Do Test Action"}
  ActionButton{action: download_jackett, "Download Jackett"}
  ActionButton{action: launch_jackett, "Launch Jackett"}
  ActionButton{action: configure_jackett, "Configure Jackett"}
  ResultsButton{action: search_jackett, results: results, "Search Jackett"}
  Table{results: results}
  "{results:?}"
 })
}
