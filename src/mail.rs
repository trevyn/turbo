use mailin_embedded::response::OK;
use mailin_embedded::{Response, Server, SslConfig};
use tracked::tracked;
use turbosql::Turbosql;

#[allow(non_camel_case_types)]
#[derive(Turbosql, Default, Clone)]
struct mail {
 rowid: Option<i64>,
 domain: Option<String>,
 from_addr: Option<String>,
 is8bit: Option<bool>,
 to_addr: Option<String>,
 data: Option<Vec<u8>>,
}

impl mailin_embedded::Handler for mail {
 fn data_start(&mut self, domain: &str, from: &str, is8bit: bool, to: &[String]) -> Response {
  *self = mail {
   rowid: None,
   domain: Some(domain.to_string()),
   from_addr: Some(from.to_string()),
   is8bit: Some(is8bit),
   to_addr: Some(to.join(", ")),
   data: Some(Vec::new()),
  };
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
pub fn start_server() -> tracked::Result<()> {
 let mut server = Server::new(mail::default());
 server.with_name("turbonet").with_ssl(SslConfig::None).unwrap();
 let listener = std::net::TcpListener::bind("0.0.0.0:25").unwrap();
 server.with_tcp_listener(listener);
 server.serve().unwrap();
 Ok(())
}
