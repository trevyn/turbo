mod backend;

use clap::Parser;

#[derive(clap::Parser)]
struct Opts {
 #[clap(short, long)]
 domain: Option<String>,
 #[clap(short, long)]
 cert_path: Option<String>,
 #[clap(short, long)]
 key_path: Option<String>,
 #[clap(short, long, default_value = "8080")]
 port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "build"]
 struct Frontend;

 pretty_env_logger::init_timed();
 let opts = Opts::parse();

 log::warn!("warn enabled");
 log::info!("info enabled");
 log::debug!("debug enabled");
 log::trace!("trace enabled");

 tokio::task::spawn_blocking(move || {
  let result = move || -> Result<(), Box<dyn ::std::error::Error>> {
   let releases = self_update::backends::github::ReleaseList::configure()
    .repo_owner("trevyn")
    .repo_name("turbo")
    .build()
    .unwrap()
    .fetch()
    .unwrap();
   println!("found releases:");
   println!("{:#?}\n", releases);

   // get the first available release
   let asset = releases[0].asset_for(&self_update::get_target()).unwrap();

   let tmp_dir =
    tempfile::Builder::new().prefix("self_update").tempdir_in(::std::env::current_dir()?)?;
   let tmp_file_path = tmp_dir.path().join(&asset.name);
   let tmp_file = ::std::fs::File::open(&tmp_file_path)?;

   self_update::Download::from_url(&asset.download_url)
    .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
    .download_to(&tmp_file)?;

   Ok(())
  }();

  dbg!(result).ok();
 })
 .await
 .ok();

 match (opts.domain, opts.key_path, opts.cert_path) {
  (Some(domain), None, None) => {
   let cert_paths = certbot::get_cert_paths("trevyn-git@protonmail.com", &domain)?;
   eprintln!("Serving HTTPS on port {}", opts.port);
   warp::serve(turbocharger::warp_routes(Frontend))
    .tls()
    .cert_path(cert_paths.fullchain)
    .key_path(cert_paths.privkey)
    .run(([0, 0, 0, 0], opts.port))
    .await;
  }
  (None, Some(key_path), Some(cert_path)) => {
   eprintln!("Serving HTTPS on port {}", opts.port);
   warp::serve(turbocharger::warp_routes(Frontend))
    .tls()
    .cert_path(cert_path)
    .key_path(key_path)
    .run(([0, 0, 0, 0], opts.port))
    .await;
  }
  (None, None, None) => {
   eprintln!("Serving (unsecured) HTTP on port {}", opts.port);
   opener::open(format!("http://127.0.0.1:{}", opts.port)).ok();
   warp::serve(turbocharger::warp_routes(Frontend)).run(([0, 0, 0, 0], opts.port)).await;
  }
  _ => eprintln!("Either domain or both of key-path and cert-path must be specified for HTTPS."),
 }

 Ok(())
}
