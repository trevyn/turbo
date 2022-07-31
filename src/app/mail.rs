use serde::{Deserialize, Serialize};
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
 pub from: String,
 pub to: String,
 pub subject: String,
 pub body: String,
}

#[frontend]
impl TryFrom<Vec<u8>> for ParsedMail {
 type Error = tracked::StringError;
 #[tracked]
 fn try_from(m: Vec<u8>) -> Result<ParsedMail, tracked::StringError> {
  fn address_to_string(header: &mail_parser::HeaderValue) -> String {
   match header {
    mail_parser::HeaderValue::Address(a) => {
     format!(
      "{}{}",
      a.address.as_ref().unwrap_or(&"".into()),
      a.name.as_ref().map(|n| format!(" ({})", n)).unwrap_or_default()
     )
    }
    // mail_parser::HeaderValue::AddressList(_) => todo!(),
    other => format!("{:?}", other),
   }
  }
  let m = mail_parser::Message::parse(&m)?;
  Ok(ParsedMail {
   from: address_to_string(m.get_from()),
   to: address_to_string(m.get_to()),
   subject: m.get_subject().map(ToString::to_string).unwrap_or_default(),
   body: m.get_body_preview(1000).map(std::borrow::Cow::into_owned).unwrap_or_default(),
  })
 }
}

#[backend]
pub async fn mail_list() -> Result<Vec<i64>, tracked::StringError> {
 // Ok(select!(Vec<mail.rowid> "ORDER BY recv_ms DESC, rowid DESC")?)
 Ok(select!(Vec<i64> "rowid FROM mail ORDER BY recv_ms DESC, rowid DESC")?)
 // Ok(select!(Vec<"rowid": i64> "FROM mail ORDER BY recv_ms DESC, rowid DESC")?)
 // Ok(select!(Vec<"rowid": i64, "bla AS blaa": String> "FROM mail ORDER BY recv_ms DESC, rowid DESC")?)
}

#[frontend]
pub fn MailList(cx: Scope) -> Element {
 use_future(&cx, (), |_| mail_list()).value().and_then(|r| match r {
  Ok(r) => rsx! {cx,
   div { class: "px-4 sm:px-6 lg:px-8",
    div { class: "-mx-4 mt-px overflow-hidden sm:-mx-6 md:mx-0",
     table { class: "min-w-full divide-y divide-gray-300",
      tbody { class: "divide-y divide-gray-200 bg-white",
       r.iter().map(|rowid| rsx! {
        tr {
         td { class: "w-full max-w-0 py-4 pl-4 pr-3 text-sm font-extralight text-gray-900 sm:pl-6",
          Mail(rowid: *rowid)
         }
        }
       })
      }
     }
    }
   }
  },
  Err(e) => rsx!(cx, p { "error: {e} " }),
 })
}

#[backend]
pub async fn mail(rowid: i64) -> Result<Vec<u8>, tracked::StringError> {
 Ok(select!(mail "WHERE rowid = " rowid)?.data?)
}

#[frontend]
#[inline_props]
pub fn Mail(cx: Scope, rowid: i64) -> Element {
 use_future(&cx, (rowid,), |(rowid,)| {
  mail(rowid)
   .and_then(super::wasm_crypto::wasm_decrypt_u8)
   .and_then(|v| async { ParsedMail::try_from(v) })
 })
 .value()
 .and_then(|r| match r {
  Ok(m) => rsx! {cx,
   "{m.from} ⇀ {m.to}"
   dl {
    dd { class: "mt-1 truncate font-normal text-gray-700", "{m.subject}" }
    dd { class: "mt-1 break-words font-light text-gray-500", "{m.body}" }
   }
  },
  Err(e) => rsx! {cx,
   p {
    class: "text-red-500",
    "mail rowid {rowid} ERROR {e}"
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
