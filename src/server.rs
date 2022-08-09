#![forbid(unsafe_code)]
#![allow(non_camel_case_types, non_snake_case)]
#![cfg_attr(feature = "wasm", allow(dead_code))]

use std::process::Command;
use turbocharger::prelude::*;

mod app;

gflags::define!(--tls = true);
gflags::define!(-p, --port: u16);
gflags::define!(-h, --help = false);

#[derive(Turbosql, Clone)]
struct Flags {
 rowid: Option<i64>,
 tls: Option<bool>,
 port: Option<u16>,
}

impl Default for Flags {
 fn default() -> Self {
  Self { rowid: None, tls: Some(true), port: None }
 }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
#[tracked]
async fn main() -> Result<(), tracked::StringError> {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "src-frontend/dist"]
 struct Frontend;

 if std::env::var_os("RUST_LOG").is_none() {
  std::env::set_var("RUST_LOG", "info")
 }

 let dev_string = format!("DEV-{}", include_str!(concat!(env!("OUT_DIR"), "/BUILD_TIME.txt")));
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

 let output = Command::new("whoami").output().unwrap();
 let whoami = std::str::from_utf8(&output.stdout[..output.stdout.len() - 1]).unwrap();
 log::warn!("Running as {}", whoami);

 match whoami {
  "root" => {
   use std::io::{stdin, stdout, Write};
   let mut s = String::new();
   print!("Install turbo as systemd service? [Y/n] ");
   let _ = stdout().flush();
   stdin().read_line(&mut s).unwrap();
   if let Some('\n') = s.chars().next_back() {
    s.pop();
   }
   if let Some('\r') = s.chars().next_back() {
    s.pop();
   }
   println!("You typed: {}", s);
   println!("Installing systemd service...");
   Command::new("/usr/sbin/useradd")
    .args(["-r", "-d", "/home/turbo", "-s", "/sbin/nologin", "turbo"])
    .output()
    .unwrap();
   Command::new("/usr/bin/mkdir").args(["-p", "/home/turbo"]).output().unwrap();
   Command::new("/usr/bin/chown").args(["turbo:turbo", "/home/turbo"]).output().unwrap();
   std::fs::write("/etc/systemd/system/turbo.service", include_str!("../turbo.service"))?;
   Command::new("/usr/bin/chmod")
    .args(["a+r", "/etc/systemd/system/turbo.service"])
    .output()
    .unwrap();
   std::fs::write("/home/turbo/turbo", std::fs::read(&std::env::current_exe()?)?)?;
   Command::new("/usr/bin/chown").args(["turbo:turbo", "/home/turbo/turbo"]).output().unwrap();
   Command::new("/usr/bin/chmod").args(["a+x", "/home/turbo/turbo"]).output().unwrap();
   Command::new("/usr/bin/systemctl").args(["daemon-reload"]).output().unwrap();
   Command::new("/usr/bin/systemctl").args(["start", "turbo"]).output().unwrap();
   Command::new("/usr/bin/systemctl").args(["enable", "turbo"]).output().unwrap();
   println!("Now running as systemd service.");
   std::process::exit(0);
  }
  "turbo" => {}
  _ => {
   log::warn!("Run as root or sudo to install as systemd service.");
  }
 }

 if TLS.is_present() || PORT.is_present() {
  Flags {
   rowid: Some(1),
   tls: if TLS.is_present() { Some(TLS.flag) } else { Some(true) },
   port: if PORT.is_present() { Some(PORT.flag) } else { None },
  }
  .update()?;
 };

 let flags = select!(Flags)?;

 turbonet::spawn_server(build_id).await?;

 std::thread::spawn(move || {
  app::mail::start_server().unwrap();
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
