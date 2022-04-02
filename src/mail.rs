use crate::backend::{encrypt, mail};
use mailin_embedded::response::{NO_MAILBOX, OK};
use mailin_embedded::{Response, Server, SslConfig};
use tracked::tracked;
use turbosql::{now_ms, select, Turbosql};

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
   recv_ip_enc: Some(encrypt(ip.to_string()).unwrap()),
   domain_enc: Some(encrypt(domain).unwrap()),
   data: Some(Vec::new()),
   ..Default::default()
  };
  OK
 }

 fn rcpt(&mut self, to: &str) -> Response {
  if to.contains(&select!(mail_config).unwrap_or_default().domain.unwrap_or_default()) {
   OK
  } else {
   mail_log { rowid: None, timestamp: Some(now_ms()), line: Some(format!("invalid rcpt: {}", to)) }
    .insert()
    .unwrap();
   NO_MAILBOX
  }
 }

 fn data_start(&mut self, _domain: &str, from: &str, is8bit: bool, to: &[String]) -> Response {
  self.from_addr_enc = Some(encrypt(from).unwrap());
  self.is8bit = Some(is8bit);
  self.to_addr_enc = Some(encrypt(to.join(", ")).unwrap());
  OK
 }

 fn data(&mut self, buf: &[u8]) -> std::io::Result<()> {
  self.data.as_mut().unwrap().extend_from_slice(buf);
  Ok(())
 }

 fn data_end(&mut self) -> Response {
  self.data = Some(encrypt(self.data.take().unwrap()).unwrap());
  self.insert().unwrap();
  OK
 }
}

#[tracked]
pub fn start_server() -> Result<(), tracked::StringError> {
 let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
 std::fs::write("/tmp/mail.cert", cert.serialize_pem()?)?;
 std::fs::write("/tmp/mail.key", cert.serialize_private_key_pem())?;

 let mut server = Server::new(mail::default());
 server.with_name("turbonet").with_ssl(SslConfig::SelfSigned {
  cert_path: "/tmp/mail.cert".into(),
  key_path: "/tmp/mail.key".into(),
 })?;

 if std::env::var_os("CI") == Some("true".into()) {
  return Ok(());
 }

 let listener = std::net::TcpListener::bind("0.0.0.0:25")?;
 server.with_tcp_listener(listener);

 server.serve()?;
 Ok(())
}
