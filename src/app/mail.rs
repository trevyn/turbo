use serde::{Deserialize, Serialize};
use tracked::tracked;
use turbocharger::prelude::*;
use turbosql::{select, Turbosql};

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

#[derive(Default, Debug)]
pub struct ParsedMail {
 pub from: Option<String>,
 pub to: Option<String>,
 pub subject: Option<String>,
 pub body: Option<String>,
}

#[tracked]
#[backend]
pub async fn mail(rowid: i64) -> Result<Vec<u8>, tracked::StringError> {
 Ok(select!(mail "WHERE rowid = " rowid)?.data?)
}

#[tracked]
#[backend]
pub async fn mail_list() -> Result<Vec<i64>, tracked::StringError> {
 // Ok(select!(Vec<mail.rowid> "ORDER BY recv_ms DESC, rowid DESC")?)
 Ok(select!(Vec<i64> "SELECT rowid FROM mail ORDER BY recv_ms DESC, rowid DESC")?)
}

#[wasm_only]
#[tracked]
pub fn mailparse(data: Vec<u8>) -> Result<ParsedMail, tracked::StringError> {
 let message = mail_parser::Message::parse(&data)?;
 Ok(ParsedMail {
  from: Some(format!("{:?}", message.get_from())),
  to: Some(format!("{:?}", message.get_to())),
  subject: message.get_subject().map(ToString::to_string),
  body: message.get_body_preview(100).map(std::borrow::Cow::into_owned),
 })
}

#[wasm_only]
pub fn MailList(cx: Scope) -> Element {
 let mail_list = match use_backend(&cx, || mail_list()).get() {
  None => rsx!(""),
  Some(r) => match r {
   Ok(r) => rsx!(r.iter().map(|rowid| rsx!(Mail(rowid: *rowid)))),
   Err(e) => rsx!(p { "error: {e} " }),
  },
 };

 cx.render(rsx! {
  mail_list
 })
}

#[wasm_only]
#[inline_props]
pub fn Mail(cx: Scope, rowid: i64) -> Element {
 let rowid = *rowid;
 cx.render(match use_backend(&cx, move || mail(rowid)).get() {
  None => rsx!(""),
  Some(m) => match m {
   Ok(m) => {
    let r = format!("{:?}", super::wasm_crypto::wasm_decrypt_u8(m).and_then(mailparse));
    rsx! {
     p {
      class: "text-red-500",
      "mail -> {r}"
     }
    }
   }
   Err(e) => rsx! {
    p {
     class: "text-red-500",
     "ERROR {e}"
    }
   },
  },
 })
}
