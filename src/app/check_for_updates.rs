use once_cell::sync::Lazy;
use turbocharger::futures_util::StreamExt;
use turbocharger::prelude::*;

#[frontend]
pub fn CheckForUpdates(cx: Scope) -> Element {
 use_stream(&cx, check_for_updates, |s, v| *s = Some(v)).read().as_ref().and_then(|r| match r {
  Ok(r) => rsx!(cx, p { "{r}" }),
  Err(e) => rsx!(cx, p { "error: {e}" }),
 })
}

#[backend]
fn check_for_updates() -> impl Stream<Item = Result<String, tracked::StringError>> {
 try_stream!({
  yield "waiting for update lock...".into();

  // TODO: this fn seems to block the executor if the dl is slow, debug that?

  static UPDATE_MUTEX: Lazy<tokio::sync::Mutex<()>> = Lazy::new(Default::default);

  let update_mutex = UPDATE_MUTEX.lock().await;

  use std::os::unix::{prelude::OpenOptionsExt, process::CommandExt};

  if option_env!("BUILD_ID").is_none() {
   yield format!(
    "Running DEV-{}; updates disabled on DEV.",
    include_str!(concat!(env!("OUT_DIR"), "/BUILD_TIME.txt"))
   );
   return;
  }

  yield "checking for updates...".into();

  let res = reqwest::Client::builder()
   .redirect(reqwest::redirect::Policy::none())
   .build()?
   .get("https://github.com/trevyn/turbo/releases/latest/download/turbo-linux")
   .send()
   .await?;

  if res.status() != 302 {
   Err(format!("Err, HTTP status {}, expected 302 redirect", res.status()))?;
  }
  let location = res.headers().get(reqwest::header::LOCATION)?.to_str()?;

  let new_version =
   regex::Regex::new(r"/releases/download/([a-z]+-[a-z]+)/")?.captures(location)?.get(1)?.as_str();

  if option_env!("BUILD_ID").unwrap_or_default() == new_version {
   yield format!("Running latest! {}", new_version);
   return;
  }

  yield format!("downloading update {}...", new_version);

  let mut bytes = Vec::new();
  let res = reqwest::get(location).await?;
  let total_size = res.content_length()?;
  let mut stream = res.bytes_stream();

  while let Some(item) = stream.next().await {
   bytes.extend_from_slice(&item?);
   yield format!("downloading update {} {}/{}...", new_version, bytes.len(), total_size);
  }

  yield format!("downloading update {} complete, {} bytes...", new_version, bytes.len());

  if bytes.len() < 10_000_000 {
   Err(format!(
    "Not updating; new release {} is unexpectedly small: {} bytes.",
    new_version,
    bytes.len()
   ))?;
  }

  if bytes.len() != total_size as usize {
   Err(format!(
    "Not updating; downloaded incorrect number of bytes: {} of {}.",
    bytes.len(),
    total_size
   ))?;
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
   drop(update_mutex);
  });

  yield format!(
   "Updated from {} to {}, {} bytes, relaunching!",
   option_env!("BUILD_ID").unwrap_or_default(),
   new_version,
   bytes.len()
  )
 })
}
