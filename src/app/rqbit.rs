use super::*;

#[frontend]
pub fn Rqbit(cx: Scope) -> Element {
 rsx!(cx, p {
  ActionButton{action: do_torrent, "download"}
 })
}

#[backend]
fn do_torrent() -> impl Stream<Item = Result<String, tracked::StringError>> {
 try_stream!({
  connection_local!(authed: &mut bool);
  if !*authed {
   Err("not authed")?;
  }

  use futures::SinkExt;
  use size_format::SizeFormatterBinary as SF;
  use std::time::Duration;

  yield "downloading...".into();

  let sopts = librqbit::session::SessionOptions {
   disable_dht: false,
   disable_dht_persistence: false,
   dht_config: None,
   peer_id: None,
   peer_opts: Some(librqbit::peer_connection::PeerConnectionOptions {
    connect_timeout: Some(Duration::from_secs(10)),
    ..Default::default()
   }),
  };

  let mut download_path = directories::BaseDirs::new().unwrap().home_dir().to_owned();
  download_path.push("turbo-downloads");

  let session = std::sync::Arc::new(
   librqbit::session::Session::new_with_opts(
    download_path,
    librqbit::spawn_utils::BlockingSpawner::new(true),
    sopts,
   )
   .await?,
  );

  let torrent_opts = librqbit::session::AddTorrentOptions {
   only_files_regex: None,
   overwrite: true,
   list_only: false,
   force_tracker_interval: None,
   ..Default::default()
  };

  let torrent_path = "magnet:?xt=urn:btih:dd8255ecdc7ca55fb0bbf81323d87062db1f6d1c&dn=Big+Buck+Bunny&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fbig-buck-bunny.torrent";

  let handle = match session.add_torrent(torrent_path, Some(torrent_opts)).await? {
   librqbit::session::AddTorrentResponse::Added(handle) => handle,
   _ => Err("Unexpected response from session.add_torrent")?,
  };

  let (tx, mut rx) = futures::channel::mpsc::unbounded();

  librqbit::spawn_utils::spawn("Stats printer", {
   let session = session.clone();
   async move {
    loop {
     session.with_torrents(|torrents| {
      for (idx, torrent) in torrents.iter().enumerate() {
       match &torrent.state {
        librqbit::session::ManagedTorrentState::Initializing => {
         log::info!("[{}] initializing", idx);
        },
        librqbit::session::ManagedTorrentState::Running(handle) => {
         let peer_stats = handle.torrent_state().peer_stats_snapshot();
         let stats = handle.torrent_state().stats_snapshot();
         let speed = handle.speed_estimator().clone();
         let total = stats.total_bytes;
         let progress = stats.total_bytes - stats.remaining_bytes;
         let downloaded_pct = if stats.remaining_bytes == 0 {
          100f64
         } else {
          (progress as f64 / total as f64) * 100f64
         };
         let mut tx = tx.clone();

         tokio::spawn(async move {
          tx.send(format!(
           "[{}]: {:.2}% ({:.2}), down speed {:.2} Mbps, fetched {}, remaining {:.2} of {:.2}, uploaded {:.2}, peers: {{live: {}, connecting: {}, queued: {}, seen: {}}}",
           idx,
           downloaded_pct,
           SF::new(progress),
           speed.download_mbps(),
           SF::new(stats.fetched_bytes),
           SF::new(stats.remaining_bytes),
           SF::new(total),
           SF::new(stats.uploaded_bytes),
           peer_stats.live,
           peer_stats.connecting,
           peer_stats.queued,
           peer_stats.seen,
          )).await.ok();
         });
        },
       }
      }
     });
     tokio::time::sleep(Duration::from_secs(1)).await;
    }
   }
  });

  while let Some(msg) = rx.next().await {
   yield (msg);
  }

  handle.wait_until_completed().await?;
 })
}
