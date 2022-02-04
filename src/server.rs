mod backend;
use turbosql::{select, Turbosql};

gflags::define!(--tls = false);
gflags::define!(-p, --port: u16);
gflags::define!(-h, --help = false);

#[derive(Turbosql, Default, Clone)]
struct Flags {
 rowid: Option<i64>,
 tls: Option<bool>,
 port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "build"]
 struct Frontend;

 if std::env::var_os("RUST_LOG").is_none() {
  std::env::set_var("RUST_LOG", "info")
 }

 pretty_env_logger::init_timed();
 gflags::parse();

 if HELP.flag {
  gflags::print_help_and_exit(0);
 }

 if select!(Option<Flags>)?.is_none() {
  Flags::default().insert()?;
 }

 if TLS.is_present() || PORT.is_present() {
  Flags {
   rowid: Some(1),
   tls: if TLS.is_present() { Some(TLS.flag) } else { None },
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
  Flags { tls: Some(true), port, .. } => {
   let port = port.unwrap_or(443);
   log::info!("Serving HTTPS on port {}", port);
   eprintln!("Serving HTTPS on port {}", port);
   turbocharger::serve_tls::<Frontend>(&std::net::SocketAddr::from(([0, 0, 0, 0], port))).await;
  }
  Flags { port, .. } => {
   let port = port.unwrap_or(8080);
   log::info!("Serving (unsecured) HTTP on port {}", port);
   eprintln!("Serving (unsecured) HTTP on port {}", port);
   eprintln!("Pass `-d server.domain.com` to auto-setup TLS certificate with Let's Encrypt.");
   opener::open(format!("http://127.0.0.1:{}", port)).ok();
   turbocharger::serve::<Frontend>(&std::net::SocketAddr::from(([0, 0, 0, 0], port))).await;
  }
 }

 Ok(())
}
