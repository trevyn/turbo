#[backend]
#[tracked]
pub async fn getblockchaininfo() -> Result<String, tracked::StringError> {
 let cookie = std::fs::read_to_string("/root/.bitcoin/.cookie")?;
 let [username, password]: [&str; 2] = cookie.split(':').collect::<Vec<&str>>().try_into().ok()?;

 Ok(
  reqwest::Client::new()
   .post("http://127.0.0.1:8332")
   .basic_auth(username, Some(password))
   .body(r#"{"jsonrpc": "1.0", "id":"test", "method": "getblockchaininfo", "params": [] }"#)
   .send()
   .await?
   .text()
   .await?,
 )
}
