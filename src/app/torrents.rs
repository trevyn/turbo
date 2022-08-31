use super::*;

#[frontend]
pub fn Torrents(cx: Scope) -> Element {
 let results = use_state(&cx, || None);

 let torrent_url = "magnet:?xt=urn:btih:dd8255ecdc7ca55fb0bbf81323d87062db1f6d1c&dn=Big+Buck+Bunny&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fbig-buck-bunny.torrent";

 rsx!(cx, p {
  ActionButton{action: auth_password, "Submit Password"}
  ActionButton{action: do_test_action, "Do Test Action"}
  ActionButton{action: download_jackett, "Download Jackett"}
  ActionButton{action: launch_jackett, "Launch Jackett"}
  ActionButton{action: configure_jackett, "Configure Jackett"}
  ActionButton{action: search_jackett, results: results, "Search Jackett"}
  ActionButton{action: move || rqbit_do_torrent(torrent_url.into(), "bigbuckbunny".into()), "download torrent"}
  TorrentTable{results: results}
 })
}

#[frontend]
#[derive(Props)]
pub struct TorrentTableProps<'a> {
 results: &'a UseState<Option<JackettResults>>,
}

#[frontend]
pub fn TorrentTable<'a>(cx: Scope<'a, TorrentTableProps<'a>>) -> Element<'a> {
 match cx.props.results.get() {
  None => rsx! {cx,
   p { "no results" }
  },
  Some(r) => {
   let num_results = r.results.len();

   rsx! {cx,
    div { "{num_results} results" }
    table {
     r.results.iter().map(|r| {
      let seeders = r.seeders.unwrap();
      let size = r.size.unwrap()/(1024*1024);
      let title = r.title.clone().unwrap();

      let infohash = r.guid.as_ref().unwrap().strip_prefix("https://rarbg.to/infohash/").unwrap();
      let torrent_url = format!("magnet:?xt=urn:btih:{}&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz", infohash);

      rsx!(
      tr {
       td { "{title}" }
       td { "{seeders}" }
       td { "{size} MB" }

       td { ActionButton{action: move || rqbit_do_torrent(torrent_url.clone(), title.clone()), "download torrent"}
      }
      }
     )})
    }
   }
  }
 }
}
