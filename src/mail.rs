use mailin_embedded::response::{NO_MAILBOX, OK};
use mailin_embedded::{Response, Server, SslConfig};
use tracked::tracked;
use turbosql::{now_ms, select, Turbosql};

#[derive(Turbosql, Default, Clone)]
struct mail {
 rowid: Option<i64>,
 recv_ms: Option<i64>,
 recv_ip: Option<String>,
 domain: Option<String>,
 from_addr: Option<String>,
 is8bit: Option<bool>,
 to_addr: Option<String>,
 data: Option<Vec<u8>>,
}

#[derive(Turbosql, Default)]
struct mail_config {
 rowid: Option<i64>,
 domain: Option<String>,
}

#[derive(Turbosql, Default)]
struct mail_log {
 rowid: Option<i64>,
 timestamp: Option<i64>,
 line: Option<String>,
}

impl mailin_embedded::Handler for mail {
 fn helo(&mut self, ip: std::net::IpAddr, domain: &str) -> Response {
  *self = mail {
   recv_ms: Some(now_ms()),
   recv_ip: Some(ip.to_string()),
   domain: Some(domain.to_string()),
   data: Some(Vec::new()),
   ..Default::default()
  };
  OK
 }

 fn rcpt(&mut self, to: &str) -> Response {
  if to.contains(&select!(mail_config).unwrap_or_default().domain.unwrap_or_default()) {
   OK
  } else {
   mail_log {
    rowid: None,
    timestamp: Some(now_ms()),
    line: Some(format!("{:?} {:?} invalid rcpt: {}", self.recv_ip, self.domain, to)),
   }
   .insert()
   .unwrap();
   NO_MAILBOX
  }
 }

 fn data_start(&mut self, _domain: &str, from: &str, is8bit: bool, to: &[String]) -> Response {
  self.from_addr = Some(from.to_string());
  self.is8bit = Some(is8bit);
  self.to_addr = Some(to.join(", "));
  OK
 }

 fn data(&mut self, buf: &[u8]) -> std::io::Result<()> {
  self.data.as_mut().unwrap().extend_from_slice(buf);
  Ok(())
 }

 fn data_end(&mut self) -> Response {
  self.insert().unwrap();
  OK
 }
}

#[tracked]
pub fn start_server() -> Result<(), tracked::Error> {
 let mut server = Server::new(mail::default());
 server.with_name("turbonet").with_ssl(SslConfig::None).unwrap();
 if std::env::var_os("CI") == Some(std::ffi::OsString::from("true")) {
  return Ok(());
 }
 let listener = std::net::TcpListener::bind("0.0.0.0:25")?;
 server.with_tcp_listener(listener);
 server.serve().unwrap();
 Ok(())
}
