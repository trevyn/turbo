mod backend;
use turbosql::{select, Turbosql};

gflags::define!(-d, --domain: &str);
gflags::define!(-c, --cert_path: &str);
gflags::define!(-k, --key_path: &str);
gflags::define!(-p, --port: u16);
gflags::define!(-h, --help = false);

#[derive(Turbosql, Default, Clone)]
struct Flags {
 rowid: Option<i64>,
 domain: Option<String>,
 cert_path: Option<String>,
 key_path: Option<String>,
 port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "build"]
 struct Frontend;

 pretty_env_logger::init_timed();
 gflags::parse();

 dbg!(option_env!("BUILD_ID"));
 dbg!(option_env!("BUILD_TIME"));

 if HELP.flag {
  gflags::print_help_and_exit(0);
 }

 log::warn!("warn enabled");
 log::info!("info enabled");
 log::debug!("debug enabled");
 log::trace!("trace enabled");

 let flags = if !DOMAIN.is_present()
  && !CERT_PATH.is_present()
  && !KEY_PATH.is_present()
  && !PORT.is_present()
 {
  // If we have no flags set, use saved flags if we have them.
  let flags = select!(Option<Flags> "WHERE rowid == 1")?;
  if let Some(flags) = flags {
   flags
  } else {
   Flags::default()
  }
 } else {
  // If we have flags set, use and save them.
  let flags = Flags {
   rowid: Some(1),
   domain: if DOMAIN.is_present() { Some(DOMAIN.flag.to_string()) } else { None },
   cert_path: if CERT_PATH.is_present() { Some(CERT_PATH.flag.to_string()) } else { None },
   key_path: if KEY_PATH.is_present() { Some(KEY_PATH.flag.to_string()) } else { None },
   port: if PORT.is_present() { Some(PORT.flag) } else { None },
  };
  if flags.update()? == 0 {
   let flags = Flags { rowid: None, ..flags.clone() };
   flags.insert()?;
  }
  flags
 };

 #[allow(clippy::or_fun_call)]
 turbonet::spawn_server(
  option_env!("BUILD_ID")
   .unwrap_or(format!("DEV {}", option_env!("BUILD_TIME").unwrap_or_default()).as_str()),
 )
 .await
 .unwrap();

 match flags {
  Flags { domain: Some(domain), cert_path: None, key_path: None, port, .. } => {
   let port = port.unwrap_or(443);
   let cert_paths = certbot::get_cert_paths("trevyn-git@protonmail.com", &domain)?;
   eprintln!("Serving HTTPS on port {}", port);
   warp::serve(turbocharger::warp_routes(Frontend))
    .tls()
    .cert_path(cert_paths.fullchain)
    .key_path(cert_paths.privkey)
    .run(([0, 0, 0, 0], port))
    .await;
  }
  Flags { domain: None, cert_path: Some(cert_path), key_path: Some(key_path), port, .. } => {
   let port = port.unwrap_or(443);
   eprintln!("Serving HTTPS on port {}", port);
   warp::serve(turbocharger::warp_routes(Frontend))
    .tls()
    .cert_path(cert_path)
    .key_path(key_path)
    .run(([0, 0, 0, 0], port))
    .await;
  }
  Flags { domain: None, cert_path: None, key_path: None, port, .. } => {
   let port = port.unwrap_or(8080);
   eprintln!("Serving (unsecured) HTTP on port {}", port);
   opener::open(format!("http://127.0.0.1:{}", port)).ok();
   warp::serve(turbocharger::warp_routes(Frontend)).run(([0, 0, 0, 0], port)).await;
  }
  _ => eprintln!("Either domain or both of key-path and cert-path must be specified for HTTPS."),
 }

 Ok(())
}
