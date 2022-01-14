mod backend;

gflags::define!(-d, --domain: &str);
gflags::define!(-c, --cert_path: &str);
gflags::define!(-k, --key_path: &str);
gflags::define!(-p, --port: u16 = 8080);
gflags::define!(-h, --help = false);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "build"]
 struct Frontend;

 pretty_env_logger::init_timed();
 gflags::parse();

 dbg!(option_env!("BUILD_ID"));

 if HELP.flag {
  gflags::print_help_and_exit(0);
 }

 log::warn!("warn enabled");
 log::info!("info enabled");
 log::debug!("debug enabled");
 log::trace!("trace enabled");

 turbonet::spawn_server().await.unwrap();

 match (DOMAIN.is_present(), KEY_PATH.is_present(), CERT_PATH.is_present()) {
  (true, false, false) => {
   let cert_paths = certbot::get_cert_paths("trevyn-git@protonmail.com", DOMAIN.flag)?;
   eprintln!("Serving HTTPS on port {}", PORT.flag);
   warp::serve(turbocharger::warp_routes(Frontend))
    .tls()
    .cert_path(cert_paths.fullchain)
    .key_path(cert_paths.privkey)
    .run(([0, 0, 0, 0], PORT.flag))
    .await;
  }
  (false, true, true) => {
   eprintln!("Serving HTTPS on port {}", PORT.flag);
   warp::serve(turbocharger::warp_routes(Frontend))
    .tls()
    .cert_path(CERT_PATH.flag)
    .key_path(KEY_PATH.flag)
    .run(([0, 0, 0, 0], PORT.flag))
    .await;
  }
  (false, false, false) => {
   eprintln!("Serving (unsecured) HTTP on port {}", PORT.flag);
   opener::open(format!("http://127.0.0.1:{}", PORT.flag)).ok();
   warp::serve(turbocharger::warp_routes(Frontend)).run(([0, 0, 0, 0], PORT.flag)).await;
  }
  _ => eprintln!("Either domain or both of key-path and cert-path must be specified for HTTPS."),
 }

 Ok(())
}
