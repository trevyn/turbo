use serde::{Deserialize, Serialize};
use tracked::tracked;
use turbocharger::prelude::*;
use turbosql::{now_ms, select, Turbosql};

#[derive(Turbosql, Default, Debug, Clone, Serialize, Deserialize)]
pub struct mail {
 pub rowid: Option<i64>,
 pub recv_ms: Option<i64>,
 pub recv_ip_enc: Option<Vec<u8>>,
 pub domain_enc: Option<Vec<u8>>,
 pub from_addr_enc: Option<Vec<u8>>,
 pub is8bit: Option<bool>,
 pub to_addr_enc: Option<Vec<u8>>,
 pub data: Option<Vec<u8>>,
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
#[derive(Turbosql, Default)]
struct mail_config {
 rowid: Option<i64>,
 domain: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
#[derive(Turbosql, Default)]
struct mail_log {
 rowid: Option<i64>,
 timestamp: Option<i64>,
 line: Option<String>,
}

#[derive(Default, Debug)]
pub struct ParsedMail {
 pub from: Option<String>,
 pub to: Option<String>,
 pub subject: Option<String>,
 pub body: Option<String>,
}

#[tracked]
#[backend]
pub async fn mail_list() -> Result<Vec<i64>, tracked::StringError> {
 // Ok(select!(Vec<mail.rowid> "ORDER BY recv_ms DESC, rowid DESC")?)
 Ok(select!(Vec<i64> "SELECT rowid FROM mail ORDER BY recv_ms DESC, rowid DESC")?)
}

#[wasm_only]
pub fn MailList(cx: Scope) -> Element {
 use_future(&cx, (), |_| mail_list()).value().and_then(|r| match r {
  Ok(r) => rsx!(cx, r.iter().map(|rowid| rsx!(Mail(rowid: *rowid)))),
  Err(e) => rsx!(cx, p { "error: {e} " }),
 })
}

#[tracked]
#[backend]
pub async fn mail(rowid: i64) -> Result<Vec<u8>, tracked::StringError> {
 Ok(select!(mail "WHERE rowid = " rowid)?.data?)
}

#[wasm_only]
#[inline_props]
pub fn Mail(cx: Scope, rowid: i64) -> Element {
 #[tracked]
 fn mailparse(data: Vec<u8>) -> Result<ParsedMail, tracked::StringError> {
  let message = mail_parser::Message::parse(&data)?;
  Ok(ParsedMail {
   from: Some(format!("{:?}", message.get_from())),
   to: Some(format!("{:?}", message.get_to())),
   subject: message.get_subject().map(ToString::to_string),
   body: message.get_body_preview(100).map(std::borrow::Cow::into_owned),
  })
 }

 use_future(&cx, (rowid,), |(rowid,)| mail(rowid)).value().and_then(|r| match r {
  Ok(m) => {
   let r = format!("{:?}", super::wasm_crypto::wasm_decrypt_u8(m).and_then(mailparse));
   rsx! {cx,
    p {
     class: "text-red-500",
     "mail -> {r}"
    }
   }
  }
  Err(e) => rsx! {cx,
   p {
    class: "text-red-500",
    "ERROR {e}"
   }
  },
 })
}

#[server_only]
#[tracked]
pub fn start_server() -> Result<(), tracked::StringError> {
 use crate::app::encrypt;
 use mailin_embedded::response::{NO_MAILBOX, OK};
 use mailin_embedded::{Response, Server, SslConfig};

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
    mail_log {
     rowid: None,
     timestamp: Some(now_ms()),
     line: Some(format!("invalid rcpt: {}", to)),
    }
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
