use turbocharger::backend;
#[allow(unused_imports)]
use turbosql::{select, Turbosql};
use wasm_bindgen::prelude::*;

#[backend]
#[derive(Turbosql)]
pub struct Person {
 pub rowid: Option<i64>,
 pub name: Option<String>,
}

#[backend]
async fn insert_person(p: Person) -> Result<i64, turbosql::Error> {
 p.insert() // returns rowid
}

#[backend]
async fn get_person(rowid: i64) -> Result<Person, turbosql::Error> {
 select!(Person "WHERE rowid = ?", rowid)
}

#[backend]
async fn getblockchaininfo() -> Result<String, reqwest::Error> {
 let cookie = std::fs::read_to_string("/root/.bitcoin/.cookie").unwrap();
 let mut cookie_iter = cookie.split(":");
 let username = cookie_iter.next().unwrap();
 let password = cookie_iter.next().unwrap();

 let client = reqwest::Client::new();
 let res = client
  .post("http://127.0.0.1:8332")
  .basic_auth(username, Some(password))
  .body(r#"{"jsonrpc": "1.0", "id":"curltest", "method": "getblockchaininfo", "params": [] }"#)
  .send()
  .await?
  .text()
  .await;
 res
}
