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

#[derive(Turbosql, Default, Clone)]
struct Certs {
 rowid: Option<i64>,
 domain: Option<String>,
 cert: Option<String>,
 key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "build"]
 struct Frontend;

 pretty_env_logger::init_timed();
 gflags::parse();

 if HELP.flag {
  gflags::print_help_and_exit(0);
 }

 if select!(Option<Flags>)?.is_none() {
  Flags::default().insert()?;
 }

 if DOMAIN.is_present() || CERT_PATH.is_present() || KEY_PATH.is_present() || PORT.is_present() {
  Flags {
   rowid: Some(1),
   domain: if DOMAIN.is_present() { Some(DOMAIN.flag.to_string()) } else { None },
   cert_path: if CERT_PATH.is_present() { Some(CERT_PATH.flag.to_string()) } else { None },
   key_path: if KEY_PATH.is_present() { Some(KEY_PATH.flag.to_string()) } else { None },
   port: if PORT.is_present() { Some(PORT.flag) } else { None },
  }
  .update()?;
 };

 let flags = select!(Flags)?;

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
   if select!(Option<Certs> "WHERE domain = ?", domain)?.is_none() {
    let cert = request_cert(&domain)?;
    Certs {
     rowid: None,
     domain: Some(domain.clone()),
     cert: Some(cert.certificate().to_string()),
     key: Some(cert.private_key().to_string()),
    }
    .insert()?;
   }
   let cert = select!(Certs "WHERE domain = ?", domain)?;

   log::info!("Serving HTTPS on port {} for {}", port, domain);
   eprintln!("Serving HTTPS on port {} for {}", port, domain);
   turbocharger::serve_tls::<Frontend>(
    &std::net::SocketAddr::from(([0, 0, 0, 0], port)),
    &cert.key.unwrap(),
    &cert.cert.unwrap(),
   )
   .await;
  }
  Flags { domain: None, cert_path: Some(cert_path), key_path: Some(key_path), port, .. } => {
   let port = port.unwrap_or(443);
   log::info!("Serving HTTPS on port {}", port);
   eprintln!("Serving HTTPS on port {}", port);
   turbocharger::serve_tls::<Frontend>(
    &std::net::SocketAddr::from(([0, 0, 0, 0], port)),
    &std::fs::read_to_string(key_path).unwrap(),
    &std::fs::read_to_string(cert_path).unwrap(),
   )
   .await;
  }
  Flags { domain: None, cert_path: None, key_path: None, port, .. } => {
   let port = port.unwrap_or(8080);
   log::info!("Serving (unsecured) HTTP on port {}", port);
   eprintln!("Serving (unsecured) HTTP on port {}", port);
   eprintln!("Pass `-d server.domain.com` to auto-setup TLS certificate with Let's Encrypt.");
   opener::open(format!("http://127.0.0.1:{}", port)).ok();
   turbocharger::serve::<Frontend>(&std::net::SocketAddr::from(([0, 0, 0, 0], port))).await;
  }
  _ => eprintln!("Either domain or both of key-path and cert-path must be specified for HTTPS."),
 }

 Ok(())
}

fn request_cert(domain: &str) -> Result<acme_lib::Certificate, acme_lib::Error> {
 log::info!("requesting new TLS cert for {}", domain);
 eprintln!("requesting new TLS cert for {}", domain);

 let url = acme_lib::DirectoryUrl::LetsEncrypt;
 let persist = acme_lib::persist::FilePersist::new(".");
 let dir = acme_lib::Directory::from_url(persist, url)?;
 let acc = dir.account("trevyn-git@protonmail.com")?;
 let mut ord_new = acc.new_order(domain, &[])?;

 log::info!("proving domain ownership");

 let ord_csr = loop {
  if let Some(ord_csr) = ord_new.confirm_validations() {
   break ord_csr;
  }

  let auths = ord_new.authorizations()?;
  let chall = auths[0].http_challenge();
  let token = chall.http_token();
  let path = format!("/.well-known/acme-challenge/{}", token);
  let proof = chall.http_proof();

  let app =
   axum::routing::Router::new().route(&path, axum::routing::get(move || acme_handler(proof)));
  async fn acme_handler(proof: String) -> impl axum::response::IntoResponse {
   log::info!("served proof");
   proof
  }
  let server = tokio::spawn(async move {
   axum::Server::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], 80)))
    .serve(app.into_make_service())
    .await
    .unwrap();
  });

  log::info!("confirming ownership");
  chall.validate(1000)?;
  log::info!("updating state");

  ord_new.refresh()?;
  log::info!("finalizing order");

  server.abort();
 };

 let pkey_pri = acme_lib::create_p384_key();
 let ord_cert = ord_csr.finalize_pkey(pkey_pri, 1000)?;
 log::info!("downloading certificate");
 let cert = ord_cert.download_and_save_cert()?;
 log::info!("certificate downloaded");
 Ok(cert)
}
