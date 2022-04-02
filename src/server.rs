#![feature(generators)]
#![forbid(unsafe_code)]
#![allow(non_camel_case_types)]

mod backend;
mod mail;

use tracked::tracked;
use turbosql::{select, Turbosql};

gflags::define!(--tls = true);
gflags::define!(-p, --port: u16);
gflags::define!(-h, --help = false);

#[derive(Turbosql, Default, Clone)]
struct Flags {
 rowid: Option<i64>,
 tls: Option<bool>,
 port: Option<u16>,
}

#[tokio::main]
#[tracked]
async fn main() -> tracked::Result<()> {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "src-frontend/dist"]
 struct Frontend;

 if std::env::var_os("RUST_LOG").is_none() {
  std::env::set_var("RUST_LOG", "info")
 }

 let dev_string = format!("DEV {}", option_env!("BUILD_TIME").unwrap_or_default());
 let build_id = option_env!("BUILD_ID").unwrap_or(&dev_string);

 tracked::set_build_id(build_id);

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

 turbonet::spawn_server(build_id).await?;

 std::thread::spawn(move || {
  mail::start_server().unwrap();
 });

 match flags {
  Flags { tls: Some(true), port, .. } => {
   let port = port.unwrap_or(443);
   log::info!("Serving HTTPS on port {}", port);
   eprintln!("Serving HTTPS on port {}", port);
   eprintln!("Connect via HTTPS to a domain pointing to this machine to auto-generate TLS certificate with Let's Encrypt.");
   eprintln!("(First connection will take about 10 seconds while certificate is provisioned.)");
   eprintln!("Pass `--notls` to disable TLS.");
   turbocharger::serve_tls::<Frontend>(&std::net::SocketAddr::from(([0, 0, 0, 0], port))).await;
  }
  Flags { port, .. } => {
   let port = port.unwrap_or(8080);
   log::info!("Serving (unsecured) HTTP on port {}", port);
   eprintln!("Serving (unsecured) HTTP on port {}", port);
   eprintln!("Pass `--tls` to enable TLS.");
   #[cfg(debug_assertions)]
   opener::open(format!("http://127.0.0.1:{}", 3000)).ok();
   turbocharger::serve::<Frontend>(&std::net::SocketAddr::from(([0, 0, 0, 0], port))).await;
  }
 }

 Ok(())
}
